mod array_store_prim;
mod char_scan_prim;
mod control_prim;
mod int_prim;
mod io_prim;
mod float_prim;
mod interp;
mod mcache;
mod om;
mod primitive_ops;
mod prim_table;
mod process;
mod process_prim;
mod snapshot;
mod std_ptrs;
mod sys_prim;

fn main() {
    println!("rusttalk!");

    let mut om = om::OM::new();
    let image = snapshot::load("st80.image", &mut om);
    let mut interp = interp::Interpreter::new(&mut om);
    interp.run();
}
