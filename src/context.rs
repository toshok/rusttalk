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

pub const TEMP_FR_START: isize = 6;

impl Context {
    pub fn is_block_ctx(&self) -> bool {
        is_int(self.FMETHOD_BLOCK_ARGC)
    }
}

pub const MSG_SELECTOR: u8 = 0;
pub const MSG_ARGS: u8 = 1;
pub const MSG_SIZE: usize = 2;
