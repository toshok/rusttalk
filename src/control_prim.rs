use interp::Interpreter;
use primitive_ops::PrimResult;

pub fn block_copy(_interp: &mut Interpreter) -> PrimResult {
    Err("prBlockCopy not implemented")
}

pub fn value(_interp: &mut Interpreter) -> PrimResult {
    Err("prValue not implemented")
}

pub fn value_with_args(_interp: &mut Interpreter) -> PrimResult {
    Err("prValWithArgs not implemented")
}

pub fn perform(_interp: &mut Interpreter) -> PrimResult {
    Err("prPerform not implemented")
}

pub fn perform_with_args(_interp: &mut Interpreter) -> PrimResult {
    Err("prPerWithArgs not implemented")
}
