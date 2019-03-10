mod interp;

fn main() {
    println!("rusttalk");

    let mut interp = interp::Interpreter::new(vec![0; 24]);
    interp.run();
}
