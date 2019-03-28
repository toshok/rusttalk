use interp::Interpreter;
use primitive_ops::PrimResult;

pub fn signal(_interp: &mut Interpreter) -> PrimResult {
    Err("prSignal not implemented")
}

pub fn wait(_interp: &mut Interpreter) -> PrimResult {
    Err("prWait not implemented")
}

pub fn resume(_interp: &mut Interpreter) -> PrimResult {
    Err("prResume not implemented")
}

pub fn suspend(_interp: &mut Interpreter) -> PrimResult {
    Err("prSuspend not implemented")
}
