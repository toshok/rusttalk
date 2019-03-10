mod interp;

fn main() {
    println!("rusttalk");

    let interp = interp::Interpreter::new(vec![0; 24]);
    interp.run();
}
