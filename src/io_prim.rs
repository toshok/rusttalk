use primitive_ops::{PrimResult};
use interp::Interpreter;

pub fn boot(_interp: &mut Interpreter) -> PrimResult {
    Err("prBoot not implemented")
}

pub fn mouse_pt(_interp: &mut Interpreter) -> PrimResult {
    Err("prMousePt not implemented")
}

pub fn cursor_loc_put(_interp: &mut Interpreter) -> PrimResult {
    Err("prCuLocPut not implemented")
}

pub fn cursor_link(_interp: &mut Interpreter) -> PrimResult {
    Err("prCuLink not implemented")
}

pub fn input_semaphore(_interp: &mut Interpreter) -> PrimResult {
    Err("prInpSema not implemented")
}

pub fn sample_interval(_interp: &mut Interpreter) -> PrimResult {
    Err("prSampleIntvl not implemented")
}

pub fn input_word(_interp: &mut Interpreter) -> PrimResult {
    Err("prInpWord not implemented")
}

pub fn copy_bits(_interp: &mut Interpreter) -> PrimResult {
    Err("prCopyBits not implemented")
}

pub fn time_words_into(_interp: &mut Interpreter) -> PrimResult {
    Err("prTime not implemented")
}

pub fn tick_words_into(_interp: &mut Interpreter) -> PrimResult {
    Err("prTick not implemented")
}

pub fn signal_at(_interp: &mut Interpreter) -> PrimResult {
    Err("prSigAt not implemented")
}

pub fn be_cursor(_interp: &mut Interpreter) -> PrimResult {
    Err("prBeCursor not implemented")
}

pub fn be_display(_interp: &mut Interpreter) -> PrimResult {
    Err("prBeDisplay not implemented")
}
