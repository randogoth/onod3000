// MIT License

// Copyright (c) 2025 Tobias Raayoni Last (@randogoth)

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// ---

// This project is a Rust port of the original Java implementation by Paul Uszak.
// Original work is licensed under the terms specified in the respective file header.


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