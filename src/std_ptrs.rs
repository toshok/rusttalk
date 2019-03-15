use om::OOP;

pub const MINUS_ONE_PTR: OOP = 0xFFFFFFFF;
pub const ZERO_PTR: OOP =      0x80000000;
pub const ONE_PTR: OOP =       0x80000001;
pub const TWO_PTR: OOP =       0x80000002;

pub const UNUSED_PTR: OOP =      0;
pub const NIL_PTR: OOP =         1;
pub const FALSE_PTR: OOP =       2;
pub const TRUE_PTR: OOP =        3;

pub const SCHED_ASS_PTR: OOP =           4;

pub const SPECIAL_SELECTORS: OOP =       24;
pub const CHAR_TABLE: OOP =              25;
pub const SMALLTALK: OOP =               9;      /* the system dictionary, not defd in G&R */