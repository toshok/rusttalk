use om::OOP;

pub struct Context {
    pub FSENDER_CALLER: OOP,
    pub FINSTR_PTR: OOP,
    pub FSTACK_PTR: OOP,
    pub FMETHOD_BLOCK_ARGC: OOP,
    pub FINIT_IP: OOP,
    pub FRECEIVER_HOME: OOP,
    pub FTEMP_FRAME: [OOP; 32]
}