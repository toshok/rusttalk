use interp::Interpreter;
use om;
use primitive_ops::PrimResult;
use std_ptrs::{FALSE_PTR, TRUE_PTR};

pub fn add(_interp: &mut Interpreter) -> PrimResult {
    return Err("not implemented, see prAddSI");
}

pub fn sub(_interp: &mut Interpreter) -> PrimResult {
    return Err("not implemented, see prSubSI");
}

fn si_logical_op(interp: &mut Interpreter, op: fn(i32, i32) -> bool) -> PrimResult {
    let right_ptr = interp.pop();
    if !om::is_int(right_ptr) {
        interp.push(right_ptr);
        return Err("rhs not an int");
    }

    let left_ptr = interp.pop();
    if !om::is_int(right_ptr) {
        interp.push(left_ptr);
        interp.push(right_ptr);
        return Err("lhs not an int");
    }

    interp.push(if op(om::int_val(left_ptr), om::int_val(right_ptr)) {
        TRUE_PTR
    } else {
        FALSE_PTR
    });

    Ok(())
}

pub fn lt(interp: &mut Interpreter) -> PrimResult {
    si_logical_op(interp, |lhs, rhs| lhs < rhs)
}
pub fn gt(interp: &mut Interpreter) -> PrimResult {
    si_logical_op(interp, |lhs, rhs| lhs > rhs)
}
pub fn le(interp: &mut Interpreter) -> PrimResult {
    si_logical_op(interp, |lhs, rhs| lhs <= rhs)
}
pub fn ge(interp: &mut Interpreter) -> PrimResult {
    si_logical_op(interp, |lhs, rhs| lhs >= rhs)
}
pub fn eq(interp: &mut Interpreter) -> PrimResult {
    si_logical_op(interp, |lhs, rhs| lhs == rhs)
}
pub fn ne(interp: &mut Interpreter) -> PrimResult {
    si_logical_op(interp, |lhs, rhs| lhs != rhs)
}

pub fn mult(_interp: &mut Interpreter) -> PrimResult {
    return Err("not implemented, see prMultSI");
}

pub fn divide(interp: &mut Interpreter) -> PrimResult {
    let divisor_ptr = interp.pop();

    if om::is_int(divisor_ptr) {
        let divisor_int = om::int_val(divisor_ptr);
        let dividend_ptr = interp.pop();
        if divisor_int != 0 && om::is_int(dividend_ptr) {
            let dividend_int = om::int_val(dividend_ptr);
            let res_int = dividend_int / divisor_int;
            if (dividend_int % divisor_int == 0) && interp.om.is_int_val(res_int) {
                interp.push(interp.om.int_obj(res_int));
                return Ok(());
            }
        }

        interp.push(dividend_ptr);
        interp.push(divisor_ptr);
        return Err("si division failed");
    }

    interp.push(divisor_ptr);
    Err("divisor not an integer")
}

pub fn modulus(_interp: &mut Interpreter) -> PrimResult {
    return Err("not implemented, see prModSI");
}

pub fn mk_pt(_interp: &mut Interpreter) -> PrimResult {
    return Err("not implemented, see mtPkSI");
}

pub fn bit_shift(_interp: &mut Interpreter) -> PrimResult {
    return Err("not implemented, see prBitShSI");
}

pub fn div(_interp: &mut Interpreter) -> PrimResult {
    return Err("not implemented, see prDivSI");
}

pub fn quo(_interp: &mut Interpreter) -> PrimResult {
    return Err("not implemented, see prQuoSI");
}

pub fn and(_interp: &mut Interpreter) -> PrimResult {
    return Err("not implemented, see prAndSI");
}

pub fn or(_interp: &mut Interpreter) -> PrimResult {
    return Err("not implemented, see prOrSI");
}

pub fn xor(_interp: &mut Interpreter) -> PrimResult {
    return Err("not implemented, see prXorSI");
}
