use std::mem;
use std::ptr;

use context;
use context::{Context, MSG_ARGS, MSG_SELECTOR, MSG_SIZE};
use mcache::{MethodCacheEntry, CACHE_SIZE};
use om::{hash, int_val, METH_ARRAY, MSG_DICT, OM, OOP, SEL_START};
use prim_table::{INT_MESSAGES, PRIMITIVE_DISPATCH};
use process::{ACTIVE_PROCESS, SUSPENDED_CTX};
use std_ptrs::{
    CLASS_ARRAY, CLASS_METH_CTX, CLASS_MSG, DOES_NOT_UNDERSTAND, FALSE_PTR, MINUS_ONE_PTR,
    MUST_BE_BOOLEAN, NIL_PTR, ONE_PTR, SCHED_ASS_PTR, SPECIAL_SELECTORS, TRUE_PTR, TWO_PTR,
    ZERO_PTR,
};

static VALUE: u8 = 1;

// should be u8?
static HEADER: u8 = 0;
static LITERAL_START: u32 = 1;

static SUPER_CLASS: u8 = 0;

pub struct Interpreter<'a> {
    pub om: &'a mut OM,
    ip: *const u8,
    sp: *mut OOP,

    // "registers" in the smalltalk formal spec
    active_context: OOP,
    home_context: OOP,
    receiver: OOP,
    method: OOP,
    msg_selector: OOP,
    arg_count: usize,
    new_method: OOP,
    prim_index: u8,

    //
    pub mcache: Vec<MethodCacheEntry>,
}

impl<'a> Interpreter<'a> {
    pub fn new(om: &'a mut OM) -> Interpreter {
        Interpreter {
            om,
            ip: vec![0; 1].as_mut_ptr(),
            sp: vec![0; 1].as_mut_ptr(),
            active_context: NIL_PTR,
            home_context: NIL_PTR,
            receiver: NIL_PTR,
            method: NIL_PTR,
            msg_selector: NIL_PTR,
            arg_count: 0,
            new_method: NIL_PTR,
            prim_index: 0,
            mcache: vec![
                MethodCacheEntry {
                    selector: NIL_PTR,
                    class: NIL_PTR,
                    method: NIL_PTR,
                    prim_index: 0
                };
                CACHE_SIZE
            ],
        }
    }

    //    fn fetch_ctx_regs

    pub fn startup(&mut self) {
        self.active_context = self.process_first_context();
        self.om.register_active_context(self.active_context);

        let ac: &Context = unsafe { mem::transmute(self.om.addr_of_oop(self.active_context)) };

        println!("caller: {}", ac.FSENDER_CALLER);
        println!("instr_ptr: {:x}", ac.FINSTR_PTR);
        println!("stack_ptr: {:x}", ac.FSTACK_PTR);
        println!("init_ip: {}", ac.FINIT_IP);
        println!("receiver: {}", ac.FRECEIVER_HOME);

        self.fetch_ctx_regs(ac);

        self.run()
    }

    fn fetch_ctx_regs(&mut self, ac: &Context) {
        self.home_context = if ac.is_block_ctx() {
            ac.FRECEIVER_HOME
        } else {
            self.active_context
        };
        let hc: &Context = unsafe { mem::transmute(self.om.addr_of_oop(self.home_context)) };
        self.receiver = hc.FRECEIVER_HOME;
        self.method = hc.FMETHOD_BLOCK_ARGC;
        unsafe {
            let absmethod = self.om.addr_of_oop(self.method);
            self.ip = (absmethod as *mut u8).offset((int_val(ac.FINSTR_PTR) as isize) - 1);
            self.sp = self
                .om
                .addr_of_oop(self.active_context)
                .offset(int_val(ac.FSTACK_PTR) as isize + context::TEMP_FR_START - 1);
            // println!("absmethod = {:p}", absmethod);
            // println!("ip = {:p}", self.ip);
            // println!("sp = {:p}", self.sp);
        }
    }

    fn run(&mut self) {
        loop {
            let bytecode = self.next_byte();

            match bytecode {
                0...15 => self.push_receiver_var(bytecode),
                16...31 => self.push_temp_var(bytecode),
                32...63 => self.push_literal_const(bytecode),
                64...95 => self.push_literal_var(bytecode),
                96...103 => self.pop_and_store_receiver_var(bytecode),
                104...111 => self.pop_and_store_temp_var(bytecode),
                112 => self.push_receiver(),
                113 => self.push_const(TRUE_PTR),
                114 => self.push_const(FALSE_PTR),
                115 => self.push_const(NIL_PTR),
                116 => self.push_const(MINUS_ONE_PTR),
                117 => self.push_const(ZERO_PTR),
                118 => self.push_const(ONE_PTR),
                119 => self.push_const(TWO_PTR),
                // why are these next 4 duplicated?
                120 => self.push_receiver(),
                121 => self.push_const(TRUE_PTR),
                122 => self.push_const(FALSE_PTR),
                123 => self.push_const(NIL_PTR),
                124 => self.return_stack_top_from_message(),
                125 => self.return_stack_top_from_block(),
                126...127 => panic!("illegal bytecode"),
                128 => self.extended_push(),
                129 => self.extended_store(),
                130 => self.extended_store_and_pop(),
                131 => self.extended_send_of_literal(),
                132 => self.extended_send_of_extended_literal(),
                // 133 => eSsLit (extended_superclass_send_of_literal)
                // 134 => eSseLit (extended_superclass_send_of_extended_literal),
                135 => self.pop_stack_top(),
                136 => self.dup_stack_top(),
                137 => self.push_active_context(),
                138...143 => panic!("illegal bytecode"),
                144...151 => self.short_unconditional_jump(bytecode),
                152...159 => self.jump_if_false(bytecode),
                160...167 => self.extended_unconditional_jump(bytecode),
                168...171 => self.extended_jump_on_true(bytecode),
                172...175 => self.extended_jump_on_false(bytecode),
                176...191 => self.send_arith_msg(bytecode),
                192...207 => panic!("special ops not implemented"), //sCommonMsg
                208...223 => self.send_literal_with_argc(bytecode, 0),
                224...239 => self.send_literal_with_argc(bytecode, 1),
                240...255 => self.send_literal_with_argc(bytecode, 2),

                _ => panic!("illegal bytecode"),
            }
        }
    }

    fn push_receiver(&mut self) {
        let oop = self.receiver;
        self.push(oop);
    }

    fn push_const(&mut self, val: OOP) {
        self.push(val)
    }

    fn push_receiver_var(&mut self, bytecode: u8) {
        let oop = self.fetch_ptr(self.receiver, bytecode & 15);
        self.push(oop)
    }

    fn push_temp_var(&mut self, bytecode: u8) {
        let oop = self.temp(bytecode & 15);
        self.push(oop);
    }

    fn push_literal_const(&mut self, bytecode: u8) {
        let oop = self.literal(bytecode & 31);
        self.push(oop);
    }

    fn push_literal_var(&mut self, bytecode: u8) {
        let oop = self.fetch_ptr(self.literal(bytecode & 31), VALUE);
        self.push(oop);
    }

    fn pop_and_store_receiver_var(&mut self, bytecode: u8) {
        let val = self.pop();
        self.store_ptr(self.receiver, bytecode & 7, val);
    }

    fn pop_and_store_temp_var(&mut self, bytecode: u8) {
        let val = self.pop();
        self.store_ptr(
            self.home_context,
            context::TEMP_FR_START as u8 + (bytecode & 7),
            val,
        );
    }

    fn return_stack_top_from_message(&mut self) {
        // not-implemented
    }

    fn return_stack_top_from_block(&mut self) {
        // not-implemented
    }

    fn extended_push(&mut self) {
        let desc = self.next_byte();

        let oop = match desc & 0xc0 {
            0 => self.fetch_ptr(self.receiver, desc & 0x3f),
            0x40 => self.temp(desc & 0x3f),
            0x80 => self.literal(desc & 0x3f),
            0xc0 => self.fetch_ptr(self.literal(desc & 0x3f), VALUE),
            _ => panic!("invalid extended_push desc"),
        };
        self.push(oop);
    }

    fn extended_store(&mut self) {
        let desc = self.next_byte();

        match desc & 0xc0 {
            0 => {
                let val = self.stack_top();
                self.store_ptr(self.receiver, desc & 0x3f, val)
            }
            0x40 => {
                let val = self.stack_top();
                self.store_ptr(
                    self.home_context,
                    (desc & 0x3F) + context::TEMP_FR_START as u8,
                    val,
                )
            }
            0x80 => panic!("invalid extended_store desc"),
            0xc0 => {
                let val = self.stack_top();
                self.store_ptr(self.literal(desc & 0x3f), VALUE, val)
            }
            _ => panic!("invalid extended_store desc"),
        }
    }

    fn extended_store_and_pop(&mut self) {
        self.extended_store();
        self.pop();
    }

    fn extended_send_of_literal(&mut self) {
        let desc = self.next_byte();
        let literal = self.literal(desc & 31);
        self.send_selector(literal, desc >> 5);
    }

    fn extended_send_of_extended_literal(&mut self) {
        let count = self.next_byte();
        let offset = self.next_byte();
        let literal = self.literal(offset);
        self.send_selector(literal, count);
    }

    fn dup_stack_top(&mut self) {
        let val = self.stack_top();
        self.push(val);
    }

    fn push_active_context(&mut self) {
        let ctx = self.active_context;
        self.push(ctx);
    }

    fn send_arith_msg(&mut self, bytecode: u8) {
        let mut sel_idx = bytecode - 176;
        if !INT_MESSAGES[sel_idx as usize](self).is_ok() {
            println!("int primitive {} returned error", sel_idx);

            sel_idx += sel_idx;
            let sel = self.fetch_ptr(SPECIAL_SELECTORS, sel_idx);
            let argc = int_val(self.fetch_ptr(SPECIAL_SELECTORS, sel_idx + 1));
            self.send_selector(sel, argc as u8);
        }
    }

    fn send_literal_with_argc(&mut self, bytecode: u8, argc: u8) {
        let literal = self.literal(bytecode & 0x15);
        self.send_selector(literal, argc);
    }

    fn short_unconditional_jump(&mut self, bytecode: u8) {
        self.jump(((bytecode & 7) + 1) as isize);
    }

    fn jump_if_false(&mut self, bytecode: u8) {
        self.jumplf(FALSE_PTR, TRUE_PTR, ((bytecode & 7) + 1) as isize);
    }

    fn extended_unconditional_jump(&mut self, bytecode: u8) {
        let next = self.next_byte() as isize;
        let offset = 256 * (((bytecode as isize) & 7) - 4) + next;
        self.jump(offset);
    }

    fn extended_jump_on_true(&mut self, bytecode: u8) {
        let next = self.next_byte() as isize;
        self.jumplf(TRUE_PTR, FALSE_PTR, 256 * ((bytecode as isize) & 3) + next);
    }

    fn extended_jump_on_false(&mut self, bytecode: u8) {
        let next = self.next_byte() as isize;
        self.jumplf(FALSE_PTR, TRUE_PTR, 256 * ((bytecode as isize) & 3) + next);
    }

    // primitives
    fn next_byte(&mut self) -> u8 {
        unsafe {
            let byte = *self.ip;
            self.ip = self.ip.offset(1);
            byte
        }
    }

    fn jump(&mut self, offset: isize) {
        unsafe {
            self.ip = self.ip.offset(offset);
        }
    }

    /* assume that cond is one of TRUE_PTR, FALSE_PTR, and notcond is its inverse */
    // XXX(toshok) that's a terrible assumption.  why?
    fn jumplf(&mut self, cond: OOP, notcond: OOP, offset: isize) {
        let bool_val = self.stack_top();
        if bool_val == cond {
            // true branch
            self.pop();
            self.jump(offset);
        } else if bool_val != notcond {
            // error branch
            self.send_selector(MUST_BE_BOOLEAN, 0);
        } else {
            // false branch
            self.pop();
            // execution continues from next ip
        }
    }

    pub fn push(&mut self, obj: OOP) {
        unsafe {
            self.sp = self.sp.offset(1);
            *self.sp = obj;
        }
    }

    pub fn pop_n(&mut self, n: isize) {
        unsafe { self.sp = self.sp.offset(-1 * n) }
    }

    pub fn pop(&mut self) -> OOP {
        unsafe {
            let oop = *self.sp;
            self.sp = self.sp.offset(-1);
            oop
        }
    }

    fn stack_top(&mut self) -> OOP {
        unsafe { *self.sp }
    }

    fn pop_stack_top(&mut self) {
        self.pop();
    }

    fn stack_val(&mut self, offset: isize) -> OOP {
        unsafe { *self.sp.offset(-offset - 1) }
    }

    fn fetch_ptr(&self, obj: OOP, offset: u8) -> OOP {
        // println!("interp.fetch_ptr(obj={}, offset={})", obj, offset);
        self.om.fetch_ptr(obj, offset as isize)
    }

    fn store_ptr(&self, obj: OOP, offset: u8, value: OOP) {
        self.om.store_ptr(obj, offset as isize, value)
    }

    fn temp(&self, offset: u8) -> OOP {
        let hc: &Context = unsafe { mem::transmute(self.om.addr_of_oop(self.home_context)) };

        hc.FTEMP_FRAME[offset as usize]
    }

    fn literal(&self, _literal_offset: u8) -> OOP {
        0 // not-implemented
    }

    fn superclass(&self, class: OOP) -> OOP {
        self.fetch_ptr(class, SUPER_CLASS)
    }

    fn send_selector(&mut self, selector: OOP, argc: u8) {
        let new_receiver = self.stack_val(argc as isize);

        self.msg_selector = selector;
        self.arg_count = argc as usize;

        let cls = self.om.fetch_class(new_receiver);

        self.send_selector_to_class(cls);
    }

    /// process stuff that should live in process.rs
    fn process_first_context(&mut self) -> OOP {
        // self.om.dump_oop(SCHED_ASS_PTR);
        let scheduler = self.fetch_ptr(SCHED_ASS_PTR, VALUE);
        let active_process = self.fetch_ptr(scheduler, ACTIVE_PROCESS);
        self.fetch_ptr(active_process, SUSPENDED_CTX)
    }

    fn get_method_from_cache(&mut self, cls: OOP, selector: OOP) -> (OOP, u8) {
        /* see Bits of History, p.244 for an explanation of this hash function */
        let hash = ((selector ^ cls) as usize) & (CACHE_SIZE - 1);
        {
            let entry = &self.mcache[hash];
            if entry.selector == selector && entry.class == cls {
                /* hit */
                return (entry.method, entry.prim_index);
            }
        }

        let method = self.lookup_method_in_class(cls, selector);
        self.new_method = method;
        self.prim_index = self.primitive_index(self.new_method);

        let save_entry = &mut self.mcache[hash];

        save_entry.selector = selector;
        save_entry.class = cls;
        save_entry.method = self.method;
        save_entry.prim_index = self.prim_index;

        return (self.method, self.prim_index);
    }

    fn send_selector_to_class(&mut self, cls: OOP) {
        let selector = self.msg_selector;
        let (method, prim_index) = self.get_method_from_cache(cls, selector);
        self.new_method = method;
        self.prim_index = prim_index;
        self.execute_new_method();
    }

    fn execute_new_method(&mut self) {
        if self.prim_index == 0 {
            let flag = self.flag_value(self.new_method);

            if flag == 6 {
                // quick instance load
                let obj = self.pop();
                let result = self.fetch_ptr(obj, self.field_index(self.new_method));
                self.push(result);
            } else {
                self.activate_new_method();
            }
        } else {
            let prim = PRIMITIVE_DISPATCH[self.prim_index as usize];
            if !prim(self).is_ok() {
                self.activate_new_method();
            }
        }
    }

    fn activate_new_method(&mut self) {
        let hdr = self.header(self.new_method);
        let ctx_size = if self.hdr_large_ctx(hdr) {
            context::TEMP_FR_START as usize + 32
        } else {
            context::TEMP_FR_START as usize + 12
        };
        let new_ctx = self.om.inst_ptrs(CLASS_METH_CTX, ctx_size);

        let abs_new_ctx: &mut Context = unsafe { mem::transmute(self.om.addr_of_oop(new_ctx)) };

        abs_new_ctx.FSENDER_CALLER = self.active_context;
        abs_new_ctx.FINSTR_PTR = self.om.int_obj(self.hdr_init_ip_of_meth(hdr) as i32);
        abs_new_ctx.FSTACK_PTR = self.om.int_obj(self.hdr_temp_count_of(hdr) as i32);
        abs_new_ctx.FMETHOD_BLOCK_ARGC = self.new_method;

        unsafe {
            // transfer(argCount + 1, sp - (WORD*)ac - argCount,
            //          activeContext, RECEIVER, newCtx);
            ptr::copy(
                self.om.addr_of_oop(self.active_context) as *const u16, // .offset(sp - ...)
                self.om.addr_of_oop(new_ctx) as *mut u16,               // .offset(RECEIVER)
                self.arg_count + 1,
            );
        }

        let arg_count = self.arg_count;

        self.pop_n((arg_count + 1) as isize);

        self.active_context = new_ctx;
        self.fetch_ctx_regs(abs_new_ctx);
        self.om.register_active_context(new_ctx);
    }

    fn lookup_method_in_class(&mut self, cls: OOP, selector: OOP) -> OOP {
        let mut method = self.lookup_method_in_class_(cls, selector);
        if method == NIL_PTR {
            let arg_count = self.arg_count;
            {
                let arg_array = self.om.inst_ptrs(CLASS_ARRAY, self.arg_count);
                let msg = self.om.inst_ptrs(CLASS_MSG, MSG_SIZE);
                self.store_ptr(msg, MSG_SELECTOR, self.msg_selector);
                self.store_ptr(msg, MSG_ARGS, arg_array);

                unsafe {
                    // transfer(self.arg_count, sp - (WORD *)ac - (self.arg_count - 1),
                    //             activeContext, 0, arg_array);
                    ptr::copy(
                        self.om.addr_of_oop(self.active_context) as *const u16,
                        self.om.addr_of_oop(arg_array) as *mut u16,
                        self.arg_count,
                    );
                }

                self.pop_n(arg_count as isize);
                self.push(msg);
                self.arg_count = 1;
            }

            method = self.lookup_method_in_class_(cls, DOES_NOT_UNDERSTAND);
            if method == NIL_PTR {
                panic!("does not understand not found?")
            }
        }

        method
    }

    fn lookup_method_in_class_(&self, cls: OOP, selector: OOP) -> OOP {
        // recursively look up superclass hierarchy for the method

        let mut current: OOP = cls;
        while current != NIL_PTR {
            let dict = self.fetch_ptr(current, MSG_DICT);

            let length = self.om.len(dict);
            let mut wrap = false;

            let (len, _) = length.overflowing_sub(SEL_START as usize - 1);
            let mut index: u8 = ((len & hash(selector)) + (SEL_START as usize)) as u8;

            loop {
                let next_sel = self.fetch_ptr(dict, index);
                if next_sel == NIL_PTR {
                    /* not found */
                    break;
                }

                if next_sel == selector {
                    /* found */
                    return self.fetch_ptr(self.fetch_ptr(dict, METH_ARRAY), index - SEL_START);
                }

                index += 1;
                if index as usize == length {
                    if wrap {
                        /* not found */
                        break;
                    }
                    wrap = true;
                    index = SEL_START;
                }
            }

            current = self.superclass(current);
        }

        NIL_PTR
    }

    fn header(&self, obj: OOP) -> OOP {
        self.fetch_ptr(obj, HEADER)
    }

    fn hdr_ext(&self, obj: OOP) -> OOP {
        self.lit_meth(self.lit_count(obj) - 2, obj)
    }

    fn hdr_large_ctx(&self, hdr: OOP) -> bool {
        hdr & 0x40 != 0
    }

    fn hdr_init_ip_of_meth(&self, hdr: OOP) -> u32 {
        ((self.lit_cnt_hdr(hdr) + LITERAL_START) * mem::size_of::<u32>() as u32 + 1)
    }

    fn hdr_temp_count_of(&self, hdr: OOP) -> u32 {
        (((hdr) & 0x0F80) >> 7)
    }

    fn lit_meth(&self, offset: u32, obj: OOP) -> OOP {
        self.fetch_ptr(obj, (offset + LITERAL_START) as u8)
    }

    fn lit_count(&self, obj: OOP) -> OOP {
        self.lit_cnt_hdr(self.header(obj))
    }

    fn lit_cnt_hdr(&self, header: OOP) -> OOP {
        header & 0x3F
    }

    fn flag_value(&self, m: OOP) -> u8 {
        ((self.header(m) & 0x7000) >> 12) as u8
    }

    fn field_index(&self, m: OOP) -> u8 {
        ((self.header(m) & 0x0F80) >> 7) as u8
    }

    pub fn primitive_index(&self, m: OOP) -> u8 {
        if self.header(m) & 0x7000 == 0x7000 {
            (self.hdr_ext(m) as u8) & 0xFF
        } else {
            0
        }
    }
}
