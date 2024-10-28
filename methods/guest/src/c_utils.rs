use std::os::raw::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn interop_print(text: *const c_char) {
    let str = unsafe { CStr::from_ptr(text).to_string_lossy().into_owned() };
    println!("print_from_c: {}", str);
}

#[no_mangle]
pub extern "C" fn interop_abort_with_msg(msg: *const c_char) {
    let str = unsafe { CStr::from_ptr(msg).to_string_lossy().into_owned() };
    panic!("abort_with_msg: {}", str);
}
