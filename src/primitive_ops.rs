use interp::Interpreter;
use std::result::Result;

pub type PrimResult = Result<(), &'static str>;

pub type Prim = fn(&mut Interpreter) -> PrimResult;

pub fn not_implemented(_interp: &mut Interpreter) -> PrimResult {
    Err("not implemented")
}
