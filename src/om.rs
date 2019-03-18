use std_ptrs::NIL_PTR;

// these are compile-time defines in st80:
const OM_SIZE: usize = 1024 * 1024;
const OT_SIZE: usize = 128 * 1024;

const FIRST_OOP: usize = 0;
const LAST_OOP: usize = OT_SIZE-1;

pub type OOP = u32;

const NON_PTR: OOP = 0xFFFFFFFF;

const HEAP_SPACE_STOP: usize = OM_SIZE - 1;     /* G/R p.658 */


#[derive(Copy, Clone)]
pub enum Loc {
//notyet    Address { addr: ADDR },
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

pub struct OM {
    otLoc:   [Loc;       OT_SIZE],
    otRest:  [u8;        OT_SIZE], /* bitmap, free, ptr and odd bits */
    otCount: [u8;        OT_SIZE], /* reference counts */
    sizes:   [SizeField; OT_SIZE], /* sizes of the object bodies */
    classes: [OOP;       OT_SIZE], /* classes of the objects */

//OOP     FCLhd[BIG_SIZE + 1];        /* initialise to NON_PTR */

    free_ptr: OOP,
    oops_left: usize,
}

impl OM {
    pub fn new() -> OM {
        OM{
            otLoc:   [Loc::Index{index: 0}; OT_SIZE],
            otRest:  [0; OT_SIZE],
            otCount: [0; OT_SIZE],
            sizes:   [0; OT_SIZE],
            classes: [NIL_PTR; OT_SIZE],

            free_ptr: NON_PTR,
            oops_left: 0,
        }
    }

    pub fn is_free(&self, oop: OOP) -> bool {
        (self.otRest[oop as usize]&REST_FREE) != 0
    }

    pub fn class(&self, oop: OOP) -> OOP {
        return self.classes[oop as usize]
    }

    pub fn is_int(&self, oop: OOP) -> bool {
        (oop & 0x80000000) != 0
    }

    pub fn int_val(&self, oop: OOP) -> i32 {
        ((oop<<1) as i32) >> 1
    }

    // the type of this function is wrong..
    pub fn is_int_val(&self, i: i32) -> bool {
        false // XXX
    }

    pub fn int_obj(&self, i: i32) -> OOP {
        (i & 0x7FFFFFFF) as OOP
    }

    pub fn free_list_prepend(&mut self, oop: OOP) {
        self.otLoc[oop as usize] = Loc::Index{index: oop as usize};
        self.free_ptr = oop;
    }

    pub fn initialize_free_list(&mut self) {
        self.free_ptr = NON_PTR;
        for_every_oop_reverse(|oop| {
            if self.is_free(oop) {
                self.oops_left += 1;
                self.free_list_prepend(oop);
            }        
        })
    }

    // methods exposed for snapshot loading
    pub fn set_ot_count(&mut self, oop: OOP, count: u8) {
        self.otCount[oop as usize] = count;
    }

    pub fn set_ot_rest(&mut self, oop: OOP, rest: u8) {
        self.otRest[oop as usize] = rest;
    }

    pub fn set_ot_size(&mut self, oop: OOP, size: i32) {
        self.sizes[oop as usize] = size;
    }

    pub fn set_ot_class(&mut self, oop: OOP, cls: OOP) {
        self.classes[oop as usize] = cls;
    }
}

    pub fn for_every_oop_reverse<F>(mut every_fn: F) where F: FnMut(OOP) {
        for oop in LAST_OOP..=FIRST_OOP {
            every_fn(oop as OOP);
        }
    }

    pub fn for_every_oop<F>(mut every_fn: F) where F: FnMut(OOP) {
        for oop in FIRST_OOP..=LAST_OOP {
            every_fn(oop as OOP);
        }
    }