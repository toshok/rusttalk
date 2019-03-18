use primitive_ops::{Prim, not_implemented};
use array_store_prim;
use control_prim;
use char_scan_prim;
use float_prim;
use io_prim;
use int_prim;
use mcache;
use process_prim;
use sys_prim;

pub const INT_MESSAGES: [Prim; 16] = [
    int_prim::add, int_prim::sub, int_prim::lt, int_prim::gt, int_prim::le, int_prim::ge, int_prim::eq, int_prim::ne,
    int_prim::mult, int_prim::divide, int_prim::modulus, int_prim::mk_pt, int_prim::bit_shift, int_prim::div,
    int_prim::and, int_prim::or
];

pub const PRIMITIVE_DISPATCH: [Prim; 256] = [
    not_implemented,
    int_prim::add, int_prim::sub, int_prim::lt, int_prim::gt, int_prim::le, int_prim::ge, int_prim::eq, int_prim::ne,
    int_prim::mult, int_prim::divide, int_prim::modulus, int_prim::div, int_prim::quo,
    int_prim::and, int_prim::or, int_prim::xor, int_prim::bit_shift, int_prim::mk_pt,
    not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented,
    float_prim::as_float, float_prim::add, float_prim::sub, float_prim::lt, float_prim::gt, float_prim::le, float_prim::ge,
    float_prim::eq, float_prim::ne, float_prim::mul, float_prim::div,
    float_prim::trunc, float_prim::frac, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented, not_implemented,
    array_store_prim::at, array_store_prim::at_put, array_store_prim::size, array_store_prim::string_at, array_store_prim::string_at_put,
    not_implemented, not_implemented, not_implemented,
    array_store_prim::object_at, array_store_prim::object_at_put, array_store_prim::new, array_store_prim::new_with_arg,
    array_store_prim::r#become, array_store_prim::inst_var_at, array_store_prim::inst_var_at_put, array_store_prim::as_oop, array_store_prim::as_object,
    array_store_prim::some_instance, array_store_prim::next_instance, array_store_prim::new_method,
    control_prim::block_copy, control_prim::value, control_prim::value_with_args,
    control_prim::perform, control_prim::perform_with_args, process_prim::signal, process_prim::wait,
    process_prim::resume, process_prim::suspend, mcache::flush,
    io_prim::mouse_pt, io_prim::cursor_loc_put, io_prim::cursor_link, io_prim::input_semaphore,
    io_prim::sample_interval, io_prim::input_word, io_prim::copy_bits,
    sys_prim::snapshot, io_prim::time_words_into, io_prim::tick_words_into, io_prim::signal_at, io_prim::be_cursor,
    io_prim::be_display, char_scan_prim::scan_chars,
    not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    sys_prim::equiv, sys_prim::class, sys_prim::core_left, sys_prim::quit, not_implemented,
    sys_prim::oops_left, sys_prim::signal_at_oops_left,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, not_implemented, not_implemented,
    not_implemented, not_implemented, io_prim::boot            
];