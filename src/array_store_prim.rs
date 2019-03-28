use interp::Interpreter;
use primitive_ops::PrimResult;

pub fn at(_interp: &mut Interpreter) -> PrimResult {
    Err("prAt not implemented")
}

pub fn at_put(_interp: &mut Interpreter) -> PrimResult {
    Err("prAtPut not implemented")
}

pub fn size(_interp: &mut Interpreter) -> PrimResult {
    Err("prAtPut not implemented")
}

pub fn string_at(_interp: &mut Interpreter) -> PrimResult {
    Err("prSAt not implemented")
}

pub fn string_at_put(_interp: &mut Interpreter) -> PrimResult {
    Err("prSAtPut not implemented")
}

pub fn object_at(_interp: &mut Interpreter) -> PrimResult {
    Err("prOAt not implemented")
}

pub fn object_at_put(_interp: &mut Interpreter) -> PrimResult {
    Err("prOAtPut not implemented")
}

pub fn new(_interp: &mut Interpreter) -> PrimResult {
    Err("prNew not implemented")
}

pub fn new_with_arg(_interp: &mut Interpreter) -> PrimResult {
    Err("prNewWithArg not implemented")
}

pub fn r#become(_interp: &mut Interpreter) -> PrimResult {
    Err("prBecome not implemented")
}

pub fn inst_var_at(_interp: &mut Interpreter) -> PrimResult {
    Err("prIVAt not implemented")
}

pub fn inst_var_at_put(_interp: &mut Interpreter) -> PrimResult {
    Err("prIVAtPut not implemented")
}

pub fn as_oop(_interp: &mut Interpreter) -> PrimResult {
    Err("prAsOop not implemented")
}

pub fn as_object(_interp: &mut Interpreter) -> PrimResult {
    Err("prAsObject not implemented")
}

pub fn some_instance(_interp: &mut Interpreter) -> PrimResult {
    Err("prSomeInstance not implemented")
}

pub fn next_instance(_interp: &mut Interpreter) -> PrimResult {
    Err("prNextInstance not implemented")
}

pub fn new_method(_interp: &mut Interpreter) -> PrimResult {
    Err("prNewMethod not implemented")
}
