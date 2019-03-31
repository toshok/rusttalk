#![feature(ptr_offset_from)]

mod array_store_prim;
mod char_scan_prim;
mod context;
mod control_prim;
mod float_prim;
mod int_prim;
mod interp;
mod io_prim;
mod mcache;
mod om;
mod prim_table;
mod primitive_ops;
mod process;
mod process_prim;
mod snapshot;
mod std_ptrs;
mod sys_prim;

fn main() {
    println!("rusttalk!");

    let mut om = om::OM::new();
    let _image = snapshot::load("st80.image", &mut om);

    // om.dump_ot();

    let mut interp = interp::Interpreter::new(&mut om);
    interp.startup();
}
