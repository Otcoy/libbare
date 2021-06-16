use std::os::raw::c_char;
use std::os::raw::c_int;

#[no_mangle] pub unsafe extern "C" fn printk(_fmt : *const c_char, mut _args: ...) -> c_int {
    // WIP
    return 0;
}
