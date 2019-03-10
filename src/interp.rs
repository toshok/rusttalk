pub struct Interpreter {
    image: Vec<u8>
}

impl Interpreter {
    pub fn new(image: Vec<u8>) -> Interpreter {
        Interpreter { image }
    }

    pub fn run(&self) {
        println!("Interpreter::run not implemented yet");
    }
}