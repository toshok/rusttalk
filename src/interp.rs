use om::{OM, OOP};
use prim_table::{INT_MESSAGES, PRIMITIVE_DISPATCH};
use process::{ACTIVE_PROCESS, SUSPENDED_CTX};
use std_ptrs::{NIL_PTR, TRUE_PTR, FALSE_PTR, SCHED_ASS_PTR, MINUS_ONE_PTR, ZERO_PTR, ONE_PTR, TWO_PTR};
use mcache::{MethodCacheEntry, CACHE_SIZE};

static VALUE: u8 = 1;

// built-in selector?
static MUST_BE_BOOLEAN: OOP = 7;

struct Context {
    fsender_caller: OOP,
    finstr_ptr: OOP,
    fstack_ptr: OOP,
    fmethod_block_argc: OOP,
    finit_ip: OOP,
    freceiver_home: OOP,
    ftemp_frame: [OOP; 32],
}

pub struct Interpreter<'a> {
    pub om: &'a OM,
    ip: usize,

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
    stack: Vec<OOP>,
    pub mcache: Vec<MethodCacheEntry>,
}

    fn lookup_method_in_class(_cls: OOP, _selector: OOP) {
        panic!("not implemented yet")
        /*
        OOP current;

#ifdef SIM
        extern void trace();

        trace('l',Class,fetchClass(stackVal(argCount)));
#endif

 start:

        for (current= Class;  current != NIL_PTR;  current= superclass(current)) {
                OOP dict= fetchPtr(MSG_DICT, current);

                                        /*ST!Interpreter!lookupMethodInDictionary:!*/

                /* lookup method in dictionary */
                WORD length= Wlen(dict);
                WORD index= ((length - SEL_START - 1) & hash(msgSelector))
                                + SEL_START;
                BOOL wrap= FALSE;

                for (;;) {
                        OOP nextSel= fetchPtr(index, dict);
                        if (nextSel == NIL_PTR)         /* not found */
                                break;
                        if (nextSel == msgSelector) {   /* found */
                                newMethod= fetchPtr(index - SEL_START,
                                                    fetchPtr(METH_ARRAY, dict));
                                primIndex= primitiveIndex(newMethod);
#ifdef SIM
                                trace('L',newMethod,stackVal(argCount));
#endif
                                return;
                        }
                        if (++index == length) {
                                if (wrap) break;        /* not found */
                                wrap= TRUE;
                                index= SEL_START;
                        }
                }
        }
        /* message not found in current dictionary */
        if (msgSelector == DOES_NOT_UNDERSTAND) {
                extern void errorDoesNotUnderstand();
                errorDoesNotUnderstand();
        }

        log("Message not understood");

        /* create message */
                                        /*ST!Interpreter!createActualMessage!*/
        {
                OOP argArray= instPtrs(CLASS_ARRAY, argCount);
                OOP msg= instPtrs(CLASS_MSG, MSG_SIZE);
                storePtr(MSG_SELECTOR, msg, msgSelector);
                storePtr(MSG_ARGS, msg, argArray);
                transfer(argCount, sp - (WORD *)ac - (argCount - 1),
                         activeContext, 0, argArray);
                pop(argCount);
                push(msg);
                argCount= 1;
        }

        msgSelector= DOES_NOT_UNDERSTAND;
        /* lookupMethodInClass(Class);  */
        goto start;
        */
    }

impl<'a> Interpreter<'a> {
    pub fn new(om: &OM) -> Interpreter {
        Interpreter {
            om,
            ip: 0,
            active_context: NIL_PTR,
            home_context: NIL_PTR,
            receiver: NIL_PTR,
            method: NIL_PTR,
            msg_selector: NIL_PTR,
            arg_count: 0,
            new_method: NIL_PTR,
            prim_index: 0,
            stack: vec![],
            mcache: vec![
                MethodCacheEntry{
                    selector: NIL_PTR,
                    class: NIL_PTR,
                    method: NIL_PTR,
                    prim_index: 0
                };
                CACHE_SIZE],
        }
    }

//    fn fetch_ctx_regs 

    pub fn run(&mut self) {
        self.active_context = self.process_first_context();
        // cacheActiveContext
        // fetchCtxRegs

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

                _ => panic!("illegal bytecode")
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
        let oop = self.fetch_ptr(bytecode & 15, self.receiver);
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
        let oop = self.fetch_ptr(VALUE, self.literal(bytecode & 31));
        self.push(oop);
    }

    fn pop_and_store_receiver_var(&mut self, bytecode: u8) {
        let val = self.pop();
        self.store_ptr(bytecode & 7, self.receiver, val);
    }

    fn pop_and_store_temp_var(&mut self, bytecode: u8) {
        let val = self.pop();
        self.store_ptr(/* TEMP_FR_START + */ bytecode & 7, self.home_context, val);
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
            0 => self.fetch_ptr(desc & 0x3f, self.receiver),
            0x40 => self.temp(desc & 0x3f),
            0x80 => self.literal(desc & 0x3f),
            0xc0 => self.fetch_ptr(VALUE, self.literal(desc & 0x3f)),
            _ => panic!("invalid extended_push desc")
        };
        self.push(oop);
    }

    fn extended_store(&mut self) {
        let desc = self.next_byte();

        match desc & 0xc0 {
            0 => {
                let val = self.stack_top();
                self.store_ptr(desc & 0x3f, self.receiver, val)
            },
            0x40 => {
                let val = self.stack_top();
                self.store_ptr(/*(*/desc & 0x3F/*) + TEMP_FR_START*/, self.home_context, val)
            },
            0x80 => panic!("invalid extended_store desc"),
            0xc0 => {
                let val = self.stack_top();
                self.store_ptr(VALUE, self.literal(desc & 0x3f), val)
            },
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
        self.send_selector(literal, desc>>5);
    }

    fn extended_send_of_extended_literal(&mut self) {
        let count = self.next_byte();
        let offset = self.next_byte();
        let literal = self.literal(offset);
        self.send_selector(literal, count);
    }

    fn pop_stack_top(&mut self) {
        self.pop();
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
        let sel_idx = (bytecode - 176) as usize;
        if !INT_MESSAGES[sel_idx](self).is_ok() {
            panic!("unhandled int primitive")
        }
    }

    fn send_literal_with_argc(&mut self, bytecode: u8, argc: u8) {
        let literal = self.literal(bytecode & 0x15);
        self.send_selector(literal, argc);
    }

    fn short_unconditional_jump(&mut self, bytecode: u8) {
        self.jump(((bytecode & 7) + 1) as usize);
    }

    fn jump_if_false(&mut self, bytecode: u8) {
        self.jumplf(FALSE_PTR, TRUE_PTR, ((bytecode & 7) + 1) as usize);
    }

    fn extended_unconditional_jump(&mut self, bytecode: u8) {
        let next = self.next_byte() as usize;
        let offset = 256 * (((bytecode as usize) & 7) - 4) + next;
        self.jump(offset);
    }

    fn extended_jump_on_true(&mut self, bytecode: u8) {
        let next = self.next_byte() as usize;
        self.jumplf(TRUE_PTR, FALSE_PTR, 256*((bytecode as usize)&3) + next);
    }

    fn extended_jump_on_false(&mut self, bytecode: u8) {
        let next = self.next_byte() as usize;
        self.jumplf(FALSE_PTR, TRUE_PTR, 256*((bytecode as usize)&3) + next);
    }

    // primitives
    fn next_byte(&mut self) -> u8 {
        let byte = 0;
        /*
        let byte = self.image[self.ip];
        self.ip += 1;
        */
        byte
    }

    fn jump(&mut self, offset: usize) {
        self.ip += offset;
    }

    /* assume that cond is one of TRUE_PTR, FALSE_PTR, and notcond is its inverse */
    // XXX(toshok) that's a terrible assumption.  why?
    fn jumplf(&mut self, cond: OOP, notcond: OOP, offset: usize) {
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
        self.stack.push(obj)
    }

    pub fn pop(&mut self) -> OOP {
        self.stack.pop().unwrap()
    }

    fn stack_top(&mut self) -> OOP {
        *self.stack.last().unwrap()
    }

    fn stack_val(&mut self, offset: usize) -> OOP {
        *self.stack.get(self.stack.len()-offset-1).unwrap()
    }

    fn fetch_ptr(&self, _offset: u8, _obj: OOP) -> OOP {
        0 // not-implemented
    }

    fn store_ptr(&self, _offset: u8, _obj: OOP, _value: OOP) {
        // not-implemented
    }

    fn temp(&self, _temp_offset: u8) -> OOP {
        0 // not-implemented
    }

    fn literal(&self, _literal_offset: u8) -> OOP {
        0 // not-implemented
    }

    fn send_selector(&mut self, selector: OOP, argc: u8) {
        let _new_receiver = self.stack_val(argc as usize);

        self.msg_selector = selector;
        self.arg_count = argc as usize;

        /*
        let cls = self.fetch_class(new_receiver);

        self.send_selector_to_class(cls);
        */

        // not-implemented
    }

    /// process stuff that should live in process.rs
    fn process_first_context(&mut self) -> OOP {
        let scheduler = self.fetch_ptr(VALUE, SCHED_ASS_PTR);
        let active_process = self.fetch_ptr(ACTIVE_PROCESS, scheduler);
        self.fetch_ptr(SUSPENDED_CTX, active_process)
    }

    fn get_method_from_cache(&mut self, cls: OOP, selector: OOP) -> (OOP, u8) {
        /* see Bits of History, p.244 for an explanation of this hash function */
        let hash = ((selector ^ cls) as usize) & (CACHE_SIZE - 1);
        let entry = &mut self.mcache[hash];

        if entry.selector == selector && entry.class == cls {
            /* hit */
            return (entry.method, entry.prim_index);
        }

        lookup_method_in_class(cls, selector);

        entry.selector = selector;
        entry.class = cls;
        entry.method = self.method;
        entry.prim_index = self.prim_index;

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
            panic!("not implemented yet")
        } else {
            let prim = PRIMITIVE_DISPATCH[self.prim_index as usize];
            if !prim(self).is_ok() {
                self.activate_new_method();
            }
        }
    }

    fn activate_new_method(&mut self) {
        panic!("not implemented yet")
        /*
        extern void trace_send();
        OOP hdr= header(newMethod);
        WORD ctxSize= HdrLargeCtx(hdr) ?
                        (TEMP_FR_START + 32)
                          : (TEMP_FR_START + 12);
        OOP newCtx= instPtrs(CLASS_METH_CTX, ctxSize);
        CONTEXT *ABSnewCtx= (CONTEXT *)addrOfOop(newCtx);
        CtxStorePtr(FSENDER_CALLER, ABSnewCtx, activeContext);
        AstrIPval(HdrInitIPofMeth(hdr), ABSnewCtx);
        AstrSPval(HdrTempCntOf(hdr), ABSnewCtx);
        CtxStorePtr(FMETHOD_BLOCK_ARGC, ABSnewCtx, newMethod);
        transfer(argCount + 1, sp - (WORD*)ac - argCount,
                 activeContext, RECEIVER, newCtx);
        pop(argCount + 1);

        newActiveCtx(newCtx);
        */
    }
}