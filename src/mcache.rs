use interp::Interpreter;
use om::OOP;
use primitive_ops::PrimResult;
use std_ptrs::NIL_PTR;

pub const CACHE_SIZE: usize = 1024;

#[derive(Clone)]
pub struct MethodCacheEntry {
    pub selector: OOP,
    pub class: OOP,
    pub method: OOP,
    pub prim_index: u8,
}

pub fn flush(interp: &mut Interpreter) -> PrimResult {
    for i in 0..CACHE_SIZE {
        interp.mcache[i].selector = NIL_PTR;
    }
    Ok(())
}
