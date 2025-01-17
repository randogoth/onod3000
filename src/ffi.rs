use std::ffi::{c_char, c_double, c_uchar, CStr};

use crate::Onod;

#[no_mangle]
pub extern "C" fn onod_run(
    test: *const c_char,
    samples: *const c_uchar,
    len: usize,
    result: *mut c_double,
) -> bool {
    // Check for null pointers
    if test.is_null() || samples.is_null() || result.is_null() {
        eprintln!("Error: Null pointer passed to onod_run.");
        return false;
    }

    // Convert C string to Rust string
    let test = unsafe {
        match CStr::from_ptr(test).to_str() {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Error: Invalid UTF-8 string passed to onod_run.");
                return false;
            }
        }
    };

    // Convert samples to a Rust slice
    let samples = unsafe { std::slice::from_raw_parts(samples, len as usize) };

    // Call the Rust `run` function
    let (obs, z, p) = Onod::run(test, samples);

    // Write results to the output buffer
    unsafe {
        *result.offset(0) = obs;
        *result.offset(1) = z;
        *result.offset(2) = p;
    }

    true
}