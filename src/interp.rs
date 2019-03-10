type OOP = usize;

pub struct Interpreter {
    image: Vec<u8>,
    ip: usize,

    // "registers" in the smalltalk formal spec
    active_context: OOP,
    home_context: OOP,
    receiver: OOP,
    method: OOP,
    msg_selector: OOP,
    new_method: OOP,

    //
    stack: Vec<OOP>,
}

    /*
        ppsTVar, ppsTVar, ppsTVar, ppsTVar, ppsTVar, ppsTVar, ppsTVar, ppsTVar,
        pRcvr, pTrue, pFalse, pNil, pM1, p0, p1, p2,
        rRcvr, rTrue, rFalse, rNil,
        rSTMessage, rSTBlock,
        BCFault, BCFault,
        ePush,
        eStore,
        ePopStore,
        esLit, eseLit, eSsLit, eSseLit,    
                popST, dupST,
        pushActCtx,
        BCFault, BCFault, BCFault, BCFault, BCFault, BCFault,
        SUJump, SUJump, SUJump, SUJump, SUJump, SUJump, SUJump, SUJump,
        pJumpFalse, pJumpFalse, pJumpFalse, pJumpFalse,
        pJumpFalse, pJumpFalse, pJumpFalse, pJumpFalse,
        eJump, eJump, eJump, eJump, eJump, eJump, eJump, eJump,
        eJumpTrue, eJumpTrue, eJumpTrue, eJumpTrue,
        eJumpFalse, eJumpFalse, eJumpFalse, eJumpFalse,
        sArithMsg, sArithMsg, sArithMsg, sArithMsg,
        sArithMsg, sArithMsg, sArithMsg, sArithMsg,
        sArithMsg, sArithMsg, sArithMsg, sArithMsg,
        sArithMsg, sArithMsg, sArithMsg, sArithMsg,
        sCommonMsg, sCommonMsg, sCommonMsg, sCommonMsg,
        sCommonMsg, sCommonMsg, sCommonMsg, sCommonMsg,
        sCommonMsg, sCommonMsg, sCommonMsg, sCommonMsg,
        sCommonMsg, sCommonMsg, sCommonMsg, sCommonMsg,
        sLit0Args, sLit0Args, sLit0Args, sLit0Args,
        sLit0Args, sLit0Args, sLit0Args, sLit0Args,
        sLit0Args, sLit0Args, sLit0Args, sLit0Args,
        sLit0Args, sLit0Args, sLit0Args, sLit0Args,
        sLit1Arg, sLit1Arg, sLit1Arg, sLit1Arg,
                sLit1Arg, sLit1Arg, sLit1Arg, sLit1Arg,
        sLit1Arg, sLit1Arg, sLit1Arg, sLit1Arg,
        sLit1Arg, sLit1Arg, sLit1Arg, sLit1Arg,
        sLit2Args, sLit2Args, sLit2Args, sLit2Args,
        sLit2Args, sLit2Args, sLit2Args, sLit2Args,
        sLit2Args, sLit2Args, sLit2Args, sLit2Args,
        sLit2Args, sLit2Args, sLit2Args, sLit2Args
        */

impl Interpreter {
    pub fn new(image: Vec<u8>) -> Interpreter {
        Interpreter {
            image,
            ip: 0,
            active_context: 0,
            home_context: 0,
            receiver: 0,
            method: 0,
            msg_selector: 0,
            new_method: 0,
            stack: vec![],
        }
    }

    pub fn run(&mut self) {
        loop {
            let bytecode = self.image[self.ip];
            self.ip+=1;

            match bytecode {
                0...15 => self.push_receiver_var(bytecode),
                16...31 => self.push_temp_var(bytecode),
                32...63 => self.push_literal_const(bytecode),
                64...95 => self.push_literal_var(bytecode),
                96...103 => self.pop_and_store_receiver_var(bytecode),
                104...111 => self.pop_and_store_temp_var(bytecode),
                _ => println!("unrecognized bytecode {}", bytecode),
            }
        }
    }

    fn push_receiver_var(&mut self, bytecode: u8) {
        let oop = self.fetch_ptr(self.receiver, bytecode & 15);
        self.push(oop)
    }

    fn push_temp_var(&mut self, bytecode: u8) {
        let oop = self.temp(bytecode & 15);
        self.push(oop);
    }

    fn push_literal_const(&mut self, bytecode: u8) {
        let oop = self.literal(bytecode & 31);
        self.push(oop);
    }

    fn push_literal_var(&mut self, bytecode: u8) {
        let oop = self.fetch_ptr(self.literal(bytecode & 31), 1);
        self.push(oop);
    }

    fn pop_and_store_receiver_var(&mut self, bytecode: u8) {
        let oop = self.pop();
        self.store_ptr(self.receiver, bytecode & 7, oop);
    }

    fn pop_and_store_temp_var(&mut self, bytecode: u8) {
        let oop = self.pop();
        self.store_ptr(self.home_context, /* TEMP_FR_START + */ bytecode & 7, oop);
    }

    fn push(&mut self, obj: OOP) {
        self.stack.push(obj)
    }

    fn pop(&mut self) -> OOP {
        self.stack.pop().unwrap()
    }

    fn fetch_ptr(&self, _obj: OOP, _offset: u8) -> OOP {
        0 // not-implemented
    }

    fn store_ptr(&self, _obj: OOP, offset: u8, value: OOP) {
        // not-implemented
    }

    fn temp(&self, _temp_offset: u8) -> OOP {
        0 // not-implemented
    }

    fn literal(&self, _literal_offset: u8) -> OOP {
        0 // not-implemented
    }
}