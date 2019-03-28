use interp::Interpreter;
use primitive_ops::PrimResult;
use std_ptrs::{FALSE_PTR, TRUE_PTR};

pub fn equiv(interp: &mut Interpreter) -> PrimResult {
    let top = interp.pop();
    let next = interp.pop();

    interp.push(if top == next { TRUE_PTR } else { FALSE_PTR });
    Ok(())
}

pub fn class(_interp: &mut Interpreter) -> PrimResult {
    /*
    let rcvr = interp.pop();
    interp.push(interp.fetch_class(rcvr));
    Ok(())
    */
    Err("prClass not implemented")
}

pub fn core_left(_interp: &mut Interpreter) -> PrimResult {
    Err("prCoreLeft not implemented")
}

pub fn quit(_interp: &mut Interpreter) -> PrimResult {
    Err("prQuit not implemented")
}

pub fn oops_left(_interp: &mut Interpreter) -> PrimResult {
    Err("prOopsLeft not implemented")
}

pub fn signal_at_oops_left(_interp: &mut Interpreter) -> PrimResult {
    Err("prSAtOopsLeft not implemented")
}

pub fn snapshot(_interp: &mut Interpreter) -> PrimResult {
    Err("prSnapshot not implemented")
}
