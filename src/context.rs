use om::{is_int, OOP};

pub struct Context {
    pub FSENDER_CALLER: OOP,
    pub FINSTR_PTR: OOP,
    pub FSTACK_PTR: OOP,
    pub FMETHOD_BLOCK_ARGC: OOP,
    pub FINIT_IP: OOP,
    pub FRECEIVER_HOME: OOP,
    pub FTEMP_FRAME: [OOP; 32],
}

pub const SENDER: isize = 0;
pub const INSTR_PTR: isize = 1;
pub const STACK_PTR: isize = 2;
pub const METHOD: isize = 3;
pub const RECEIVER: isize = 5;
pub const TEMP_FR_START: isize = 6;

pub const CALLER: isize = 0;
pub const BLOCK_ARGC: isize = 3;
pub const INIT_IP: isize = 4;
pub const HOME: isize = 5;

impl Context {
    pub fn is_block_ctx(&self) -> bool {
        is_int(self.FMETHOD_BLOCK_ARGC)
    }
}

pub const MSG_SELECTOR: u8 = 0;
pub const MSG_ARGS: u8 = 1;
pub const MSG_SIZE: usize = 2;
