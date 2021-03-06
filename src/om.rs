use std::mem;
use std_ptrs::{
    CLASS_COMPILED_METHOD, CLASS_SMALL_INTEGER, LAST_REF_COUNTED_OOP, NIL_PTR, SYSTEM_DICTIONARY,
    UNUSED_PTR,
};

static MAX_INT: i32 = 0x3FFFFFFF; /* 2^30 - 1 */
static MIN_INT: i32 = -(1 << 30); /* -2^30 */

// these are compile-time defines in st80:
pub const OM_SIZE: usize = 128 * 1024 * 1024;
const OT_SIZE: usize = 128 * 1024;

const FIRST_OOP: usize = 0;
const LAST_OOP: usize = OT_SIZE - 1;

pub type OOP = u32;

const NON_PTR: OOP = 0xFFFFFFFF;

const HEAP_SPACE_STOP: usize = OM_SIZE - 1; /* G/R p.658 */

const BIG_SIZE: usize = 40;

const SATURATED: u8 = 1;

// should be u8?
static HEADER: isize = 0;
static LITERAL_START: u32 = 1;

pub const SUPER_CLASS: u8 = 0;
pub const MSG_DICT: u8 = 1;
pub const INSTANCE_SPEC: u8 = 2;
pub const METH_ARRAY: u8 = 1;
pub const SEL_START: u8 = 2;

#[derive(Copy, Clone)]
pub enum Loc {
    Address { addr: *mut u32 },
    Index { index: usize },
    //notyet    Bitmap { bmap: BITMAP },
}

type SizeField = i32;

/*
enum Size_fld {

}
*/

const REST_PTRS: u8 = 2;
const REST_FREE: u8 = 4;
const REST_BITMAP: u8 = 8;

fn to_word_size(nbytes: usize) -> usize {
    (nbytes + mem::size_of::<OOP>() - 1) / mem::size_of::<OOP>()
}

fn counted(oop: OOP) -> bool {
    oop > LAST_REF_COUNTED_OOP && oop <= LAST_OOP as OOP
}

pub struct OM {
    object_space: *mut OOP,

    ot_loc: [Loc; OT_SIZE],
    ot_rest: [u8; OT_SIZE],      /* bitmap, free, ptr and odd bits */
    ot_count: [u16; OT_SIZE],    /* reference counts */
    sizes: [SizeField; OT_SIZE], /* sizes of the object bodies */
    classes: [OOP; OT_SIZE],     /* classes of the objects */

    fcl_heads: [OOP; BIG_SIZE + 1], /* initialise to NON_PTR */
    free_ptr: OOP,
    oops_left: usize,

    active_context: OOP,
}

impl OM {
    pub fn new() -> OM {
        OM {
            object_space: vec![0; 1].as_mut_ptr(),
            ot_loc: [Loc::Index { index: 0 }; OT_SIZE],
            ot_rest: [REST_FREE; OT_SIZE],
            ot_count: [0; OT_SIZE],
            sizes: [0; OT_SIZE],
            classes: [NIL_PTR; OT_SIZE],

            fcl_heads: [NON_PTR; BIG_SIZE + 1],
            free_ptr: NON_PTR,
            oops_left: 0,

            active_context: NIL_PTR,
        }
    }

    // getters/setters for the ot_* arrays
    fn get_count(&self, oop: OOP) -> u16 {
        self.ot_count[oop as usize]
    }

    pub fn get_class(&self, oop: OOP) -> OOP {
        return self.classes[oop as usize];
    }

    fn get_size(&self, oop: OOP) -> usize {
        self.sizes[oop as usize] as usize
    }

    fn get_loc(&self, oop: OOP) -> Loc {
        self.ot_loc[oop as usize]
    }

    fn get_rest(&self, oop: OOP) -> u8 {
        self.ot_rest[oop as usize]
    }

    pub fn set_count(&mut self, oop: OOP, count: u16) {
        // println!("setting count for oop {} to {}", oop, count);
        self.ot_count[oop as usize] = count;
    }

    pub fn set_size(&mut self, oop: OOP, size: usize) {
        // println!("setting size for oop {} to {}", oop, size);
        self.sizes[oop as usize] = size as i32;
    }

    pub fn set_class(&mut self, oop: OOP, cls: OOP) {
        // println!("setting class for oop {} to {}", oop, cls);
        self.classes[oop as usize] = cls;
    }

    pub fn set_loc(&mut self, oop: OOP, loc: Loc) {
        self.ot_loc[oop as usize] = loc;
    }

    pub fn set_rest(&mut self, oop: OOP, rest: u8) {
        // println!("setting rest for oop {} to {}", oop, rest);
        self.ot_rest[oop as usize] = rest;
    }

    pub fn location_addr(&self, oop: OOP) -> *mut OOP {
        match self.get_loc(oop) {
            Loc::Address { addr } => addr,
            _ => panic!("location not an address"),
        }
    }

    pub fn location_index(&self, oop: OOP) -> usize {
        match self.get_loc(oop) {
            Loc::Index { index } => index,
            _ => panic!("location not an address"),
        }
    }

    pub fn is_free(&self, oop: OOP) -> bool {
        (self.get_rest(oop) & REST_FREE) != 0
    }

    pub fn is_ptrs(&self, oop: OOP) -> bool {
        (self.get_rest(oop) & REST_PTRS) != 0
    }

    pub fn is_bitmap(&self, oop: OOP) -> bool {
        (self.get_rest(oop) & REST_BITMAP) != 0
    }

    pub fn len(&self, oop: OOP) -> usize {
        return self.get_size(oop) / mem::size_of::<OOP>();
    }

    // the type of this function is wrong..
    pub fn is_int_val(&self, val: i32) -> bool {
        val <= MAX_INT && val >= MIN_INT
    }

    pub fn int_obj(&self, i: i32) -> OOP {
        (i & 0x7FFFFFFF) as OOP
    }

    fn header(&self, obj: OOP) -> OOP {
        self.fetch_ptr(obj, HEADER)
    }

    fn lit_count(&self, obj: OOP) -> OOP {
        self.lit_cnt_hdr(self.header(obj))
    }

    fn lit_cnt_hdr(&self, header: OOP) -> OOP {
        header & 0x3F
    }

    pub fn free_list_prepend(&mut self, oop: OOP) {
        let next = self.free_ptr;
        self.set_free_list_next(oop, next);
        self.free_ptr = oop;
    }

    pub fn initialize_object_space(&mut self, object_space: &mut Vec<u32>, offsets: Vec<i32>) {
        self.object_space = object_space.as_mut_ptr();

        unsafe {
            println!(
                "object space initialized to {:p}-{:p}",
                self.object_space,
                self.object_space.offset(object_space.len() as isize)
            );
        }

        mem::forget(object_space);
        for_every_oop(|oop| {
            if self.is_free(oop) {
                return;
            }
            if self.is_bitmap(oop) {
                self.set_loc(
                    oop,
                    Loc::Index {
                        index: offsets[oop as usize] as usize,
                    },
                );
                return;
            }
            unsafe {
                let loc = Loc::Address {
                    addr: self.object_space.offset(offsets[oop as usize] as isize),
                };
                self.set_loc(oop, loc);
            }
        });
    }

    pub fn initialize_free_list(&mut self) {
        self.free_ptr = NON_PTR;
        for_every_oop_reverse(|oop| {
            if self.is_free(oop) {
                self.oops_left += 1;
                self.free_list_prepend(oop);
            }
        });
        if self.free_ptr == NON_PTR {
            panic!("no free OOPs during initialization");
        }
    }

    pub fn initialize_free_chunks(&mut self, object_space_length: usize) {
        let fcp = self.free_ptr;
        self.free_ptr = self.get_free_list_next(fcp);
        self.fcl_heads[BIG_SIZE] = fcp;

        let loc = unsafe {
            self.object_space
                .offset((object_space_length / mem::size_of::<OOP>()) as isize)
        };

        let free_space = OM_SIZE - object_space_length;
        self.set_loc(fcp, Loc::Address { addr: loc });
        let rest = self.get_rest(fcp);
        self.set_rest(fcp, rest & !(REST_FREE | REST_PTRS));
        self.set_size(fcp, free_space / mem::size_of::<OOP>());
        self.set_class(fcp, NON_PTR);
    }

    pub fn fetch_ptr(&self, oop: OOP, i: isize) -> OOP {
        unsafe {
            let lptr = self.location_addr(oop).offset(i);
            // println!(
            //     "fetch_ptr(oop_ptr={:p}, offset={}, ptr={:p}) -> {}",
            //     self.location_addr(oop),
            //     i,
            //     ptr,
            //     *ptr
            // );
            *lptr
        }
    }

    pub fn store_ptr(&mut self, oop: OOP, i: isize, value: OOP) {
        self.inc_ref(value);

        unsafe {
            let ptr = self.location_addr(oop).offset(i);
            self.dec_ref(*ptr);
            *ptr = value;
        }
    }

    pub unsafe fn addr_of_oop(&self, oop: OOP) -> *mut OOP {
        self.location_addr(oop)
    }

    pub fn fetch_class(&self, oop: OOP) -> OOP {
        if is_int(oop) {
            CLASS_SMALL_INTEGER
        } else {
            self.classes[oop as usize]
        }
    }

    pub fn inst_ptrs(&mut self, class: OOP, len: usize) -> OOP {
        self.allocate(len * mem::size_of::<OOP>(), true, class)
    }

    fn deallocate(&mut self, oop: OOP) {
        // println!("deallocating {}", oop);
        if self.is_bitmap(oop) {
            // TODO(toshok) some additional freeing here. not sure how we're supposed to free "native" bitmaps?
            let rest = self.get_rest(oop);
            self.set_rest(oop, (rest | REST_FREE) & !REST_BITMAP);
            self.free_list_prepend(oop);
        } else {
            let mut wsize = to_word_size(self.get_size(oop));
            if wsize > BIG_SIZE {
                wsize = BIG_SIZE;
            }

            let oop_next = self.fcl_heads[wsize];
            self.set_free_chunk_next(oop, oop_next);
            self.fcl_heads[wsize] = oop;
        }
    }

    fn allocate(&mut self, nbytes: usize, ptrs: bool, class: OOP) -> OOP {
        let oop = self.alloc(nbytes);
        self.init(oop, nbytes, ptrs, class);
        oop
    }

    fn alloc(&mut self, nbytes: usize) -> OOP {
        // println!("alloc({})", nbytes);
        let mut oop;

        let wsize = to_word_size(nbytes);

        if wsize < BIG_SIZE {
            oop = self.fcl_heads[wsize];
            if oop != NON_PTR {
                self.fcl_heads[wsize] = self.get_free_chunk_next(oop);
                return oop;
            }
        }

        oop = self.attempt_to_alloc(nbytes);
        if oop == NON_PTR {
            // self.compact();
            oop = self.attempt_to_alloc(nbytes);
            if oop == NON_PTR {
                self.reclaim();
                oop = self.attempt_to_alloc(nbytes);
                if oop == NON_PTR {
                    // self.compact();
                    oop = self.attempt_to_alloc(nbytes);
                    if oop == NON_PTR {
                        panic!("errorNoMem");
                    }
                }
            }
        }
        oop
    }

    fn attempt_to_alloc(&mut self, nbytes: usize) -> OOP {
        let mut prev = NON_PTR;
        let mut oop = self.fcl_heads[BIG_SIZE];

        let wsize = to_word_size(nbytes);

        while oop != NON_PTR {
            let next_oop = self.get_free_chunk_next(oop);
            let available_size = to_word_size(self.get_size(oop));
            // println!(
            //     "attempt_to_alloc {}, available_size = {}",
            //     nbytes, available_size
            // );
            if available_size == wsize {
                // exact match
                // println!("   found exact match");
                if prev == NON_PTR {
                    self.fcl_heads[BIG_SIZE] = next_oop;
                } else {
                    self.set_free_chunk_next(prev, next_oop);
                }
                return oop;
            }

            let excess_words = (available_size as isize) - (wsize as isize);
            if excess_words < 0 {
                // println!("   not enough words, skipping");
                prev = oop;
                oop = next_oop;
                continue;
            }

            // println!("   more than enough words, splitting object");
            // available size is larger than wsize.  split the object
            let oop_addr = unsafe { self.location_addr(oop).offset(wsize as isize) };
            let rest_oop =
                self.obtain_ptr((excess_words as usize) * mem::size_of::<OOP>(), oop_addr);
            if rest_oop == NON_PTR {
                // allocation failed
                return NON_PTR;
            }
            self.set_size(oop, nbytes);

            // unlink oop from whatever list it was in
            if prev == NON_PTR {
                // we're replacing the list head
                self.fcl_heads[BIG_SIZE] = next_oop;
            } else {
                self.set_free_chunk_next(prev, next_oop);
            }

            // link rest_oop into the appropriate free chunk list
            let fcl_idx = if (excess_words as usize) < BIG_SIZE {
                excess_words as usize
            } else {
                BIG_SIZE
            };

            let rest_next = self.fcl_heads[fcl_idx];
            self.set_free_chunk_next(rest_oop, rest_next);
            self.fcl_heads[fcl_idx] = rest_oop;

            // println!(
            //     "put rest_oop (size {}) in free chunk list {}",
            //     excess_words, fcl_idx
            // );
            return oop;
        }

        // no suitable entries in chain
        return NON_PTR;
    }

    fn obtain_ptr(&mut self, nbytes: usize, loc: *mut OOP) -> OOP {
        let oop = self.free_ptr;
        if oop == NON_PTR {
            // no free pointers
            println!("free list empty");
            return NON_PTR;
        }

        self.free_ptr = self.get_free_list_next(oop);

        self.set_count(oop, 0);

        let rest = self.get_rest(oop);
        self.set_rest(oop, rest & !(REST_FREE | REST_PTRS | REST_BITMAP));
        self.set_size(oop, nbytes);

        self.set_loc(oop, Loc::Address { addr: loc });
        oop
    }

    fn init(&mut self, oop: OOP, nbytes: usize, ptrs: bool, class: OOP) {
        self.inc_ref(class);

        self.set_rest(oop, if ptrs { REST_PTRS } else { 0 });
        self.set_class(oop, class);
        self.set_size(oop, nbytes);

        /* place nil/0 in newly allocated object */
        let defaultEntry = if ptrs { NIL_PTR } else { 0 };
        let wsize = ((nbytes + mem::size_of::<OOP>() - 1) / mem::size_of::<OOP>()) as isize;

        unsafe {
            let slots = self.location_addr(oop);
            for i in 0..wsize {
                *slots.offset(i) = defaultEntry;
            }
        }
    }

    pub fn dump_ot(&self) {
        for i in FIRST_OOP..=LAST_OOP {
            self.dump_oop(i as OOP);
        }
    }

    pub fn dump_oop(&self, oop: OOP) {
        print!("{}:", oop as usize);
        print!(" class: {}", self.get_class(oop));
        if self.get_class(oop) <= LAST_REF_COUNTED_OOP {
            match self.get_class(oop) {
                CLASS_SMALL_INTEGER => print!("(smi)"),
                CLASS_STRING => print!("(string)"),
                CLASS_ARRAY => print!("(array)"),
                CLASS_FLOAT => print!("(float)"),
                CLASS_METH_CTX => print!("(meth_ctx)"),
                CLASS_BLOCK_CTX => print!("(block_ctx)"),
                CLASS_POINT => print!("(point)"),
                CLASS_LG_POS_INT => print!("(lg_pos_int)"),
                CLASS_DISPLAY_BITMAP => print!("(display_bitmap)"),
                CLASS_MSG => print!("(message)"),
                CLASS_COMPILED_METHOD => print!("(compiled_method)"),
                CLASS_SEMA => print!("(sema)"),
                CLASS_CHARACTER => print!("(character)"),
                CLASS_FORM => print!("(form)"),
                _ => print!("(unknown)"),
            }
        }
        print!(" size: {}", self.get_size(oop));
        print!(" rest: {}", self.get_rest(oop));
        println!(" count: {}", self.get_count(oop));
    }

    fn set_free_list_next(&mut self, node: OOP, next: OOP) {
        self.set_loc(
            node,
            Loc::Index {
                index: next as usize,
            },
        );
    }

    fn get_free_list_next(&self, node: OOP) -> OOP {
        self.location_index(node) as OOP
    }

    fn set_free_chunk_next(&mut self, node: OOP, next: OOP) {
        self.set_class(node, next);
    }

    fn get_free_chunk_next(&self, node: OOP) -> OOP {
        self.get_class(node)
    }

    //
    // GC related things
    //
    pub fn inc_ref(&mut self, oop: OOP) {
        if !counted(oop) {
            return;
        }
        let cnt = self.ot_count[oop as usize];
        self.ot_count[oop as usize] = cnt + 1;
    }

    pub fn dec_ref(&mut self, oop: OOP) {
        if !counted(oop) {
            return;
        }
        let cnt = self.ot_count[oop as usize];
        self.ot_count[oop as usize] = cnt - 1;
    }

    fn unmark(&mut self, oop: OOP) {
        self.set_count(oop, 0);
    }
    fn mark(&mut self, oop: OOP) {
        self.set_count(oop, 2);
    }
    fn unmarked(&self, oop: OOP) -> bool {
        self.ot_count[oop as usize] == 0
    }

    fn zero_ref_counts(&mut self) {
        for_every_oop(|oop| {
            self.unmark(oop);
        })
    }

    fn mark_rest(&mut self, root: OOP) {
        unsafe {
            // println!(
            //     "marking fields (size = {}, last_pointer_index = {})",
            //     self.get_size(root),
            //     self.last_pointer_index(root)
            // );
            for offset in 0..self.last_pointer_index(root) {
                let val = self.location_addr(root).offset(offset as isize);
                if !is_int(*val) {
                    self.mark_from(*val);
                }
            }
        }

        // println!("marking class");
        let class = self.get_class(root);
        self.mark_from(class);
    }

    fn mark_from(&mut self, root: OOP) {
        if self.unmarked(root) {
            self.mark(root);
            // println!("marked {}, refcount {}", root, self.ot_count[root as usize]);
            self.mark_rest(root);
        }
    }

    fn mark_objects(&mut self) {
        self.mark_from(SYSTEM_DICTIONARY);

        println!("mark_from active_context");
        let active_context = self.active_context;
        self.mark_from(active_context);
    }

    pub fn register_active_context(&mut self, active_context: OOP) {
        self.active_context = active_context;
    }

    fn last_pointer_index(&self, oop: OOP) -> usize {
        if self.is_ptrs(oop) {
            return self.get_size(oop) / mem::size_of::<OOP>();
        }
        if self.classes[oop as usize] == CLASS_COMPILED_METHOD {
            return self.lit_count(oop) as usize + 1;
        }
        return 0;
    }

    fn rectify_ref_counts(&mut self) {
        println!("rectifying refcounts");

        // reset heads of free chunk lists
        for size in 0..=BIG_SIZE {
            self.fcl_heads[size] = NON_PTR;
        }

        // claim oop 0 is used
        self.set_count(UNUSED_PTR, 3);

        for_every_oop(|oop| {
            if !self.is_free(oop) {
                let count = self.get_count(oop);

                if self.unmarked(oop) {
                    self.deallocate(oop);
                } else {
                    // println!("OOP = {}, count = {}", oop, count);
                    self.set_count(oop, count - 2);
                    let oop_class = self.get_class(oop);
                    self.inc_ref(oop_class);

                    unsafe {
                        for offset in 0..self.last_pointer_index(oop) {
                            let val = self.addr_of_oop(oop).offset(offset as isize);
                            if !is_int(*val) {
                                self.inc_ref(*val);
                            }
                        }
                    }
                }
            }
        });

        // make sure the roots don't disappear
        self.inc_ref(SYSTEM_DICTIONARY);
        let active_context = self.active_context;
        self.inc_ref(active_context);

        for i in NIL_PTR..=LAST_REF_COUNTED_OOP {
            self.set_count(i, SATURATED.into());
        }
    }

    fn reclaim(&mut self) {
        println!("collecting garbage");

        self.zero_ref_counts();
        self.mark_objects();
        self.rectify_ref_counts();
    }
}

pub fn is_int(oop: OOP) -> bool {
    (oop & 0x80000000) != 0
}

pub fn int_val(oop: OOP) -> i32 {
    ((oop << 1) as i32) >> 1
}

pub fn hash(oop: OOP) -> usize {
    oop as usize
}

pub fn for_every_oop_reverse<F>(mut every_fn: F)
where
    F: FnMut(OOP),
{
    for oop in (FIRST_OOP..=LAST_OOP).rev() {
        every_fn(oop as OOP);
    }
}

pub fn for_every_oop<F>(mut every_fn: F)
where
    F: FnMut(OOP),
{
    for oop in FIRST_OOP..=LAST_OOP {
        every_fn(oop as OOP);
    }
}
