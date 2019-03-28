use om::OOP;

pub const MINUS_ONE_PTR: OOP = 0xFFFFFFFF;
pub const ZERO_PTR: OOP = 0x80000000;
pub const ONE_PTR: OOP = 0x80000001;
pub const TWO_PTR: OOP = 0x80000002;

pub const UNUSED_PTR: OOP = 0;
pub const NIL_PTR: OOP = 1;
pub const FALSE_PTR: OOP = 2;
pub const TRUE_PTR: OOP = 3;

pub const SCHED_ASS_PTR: OOP = 4;

pub const SPECIAL_SELECTORS: OOP = 24;
pub const CHAR_TABLE: OOP = 25;
pub const SMALLTALK: OOP = 9; /* the system dictionary, not defd in G&R */

pub const CLASS_SMALL_INTEGER: OOP = 6; /* p. 687 */
pub const CLASS_STRING: OOP = 7;
pub const CLASS_ARRAY: OOP = 8;
pub const CLASS_FLOAT: OOP = 10; /* not defined in G/R */
pub const CLASS_METH_CTX: OOP = 11;
pub const CLASS_BLOCK_CTX: OOP = 12;
pub const CLASS_POINT: OOP = 13;
pub const CLASS_LG_POS_INT: OOP = 14;
pub const CLASS_DISPLAY_BITMAP: OOP = 15;
pub const CLASS_MSG: OOP = 16;
pub const CLASS_COMPILED_METHOD: OOP = 17; /* p. 686 */
pub const CLASS_SEMA: OOP = 19; /* not defined in G/R */
pub const CLASS_CHARACTER: OOP = 20;
pub const CLASS_FORM: OOP = 0x629; /* required for BitBlt */
