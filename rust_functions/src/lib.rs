extern crate libc;

use std::ffi::{CString, CStr};
use std::str;
use std::os::raw::c_char;
use std::slice;

#[no_mangle]
pub extern fn count(string : *const c_char, sub_string : *const c_char) -> i32 {
    let c_string = unsafe { CStr::from_ptr(string).to_bytes() };
    let c_sub_string = unsafe { CStr::from_ptr(sub_string).to_bytes() };

    let rust_string = str::from_utf8(c_string).unwrap();
    let rust_sub_string = str::from_utf8(c_sub_string).unwrap();

    rust_string.matches(rust_sub_string).count() as i32
}

#[no_mangle]
pub extern fn append(string : *const c_char, sub_string :  *const c_char) -> *const c_char {
    let c_string = unsafe { CStr::from_ptr(string).to_bytes() };
    let c_sub_string = unsafe { CStr::from_ptr(sub_string).to_bytes() };

    let rust_string = str::from_utf8(c_string).unwrap();
    let rust_sub_string = str::from_utf8(c_sub_string).unwrap();

    CString::new(rust_string.to_owned() + rust_sub_string).unwrap().into_raw()
}

#[no_mangle]
pub extern fn cube(input : i32) -> i32 {
    input * input * input
}

#[no_mangle]
pub extern fn float_multiply(a : f32, b : f32) -> f32 {
    a * b
}

#[repr(C)]
pub struct Tuple {
    l : *const i32,
    size : i32,
}

#[no_mangle]
pub extern fn increment_list(c_array : *mut i32, len : i32) ->  Tuple {
    let rust_list = unsafe { slice::from_raw_parts_mut(c_array, len as usize) };

    for i in 0..rust_list.len() {
        rust_list[i] += 1;
    }

    Tuple{l     : rust_list.as_ptr(), 
          size  : rust_list.len() as i32}
}
