use cstr_core::c_char;

#[no_mangle] pub unsafe extern fn printk(_fmt : *const c_char, mut _args: ...) -> libc::c_int {
    // WIP
    return 0;
}


#[no_mangle] pub extern fn test() {}