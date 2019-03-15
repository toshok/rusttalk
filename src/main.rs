mod interp;
mod om;
mod process;
mod snapshot;
mod std_ptrs;

fn main() {
    println!("rusttalk!");

    let image = snapshot::load("st80.image");
    let mut interp = interp::Interpreter::new(vec![0; 24]);
    interp.run();
}
