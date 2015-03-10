#![feature(libc)]

extern crate libc;

use std::str;
use std::ffi::CStr;
use libc::{c_char, c_int};


#[no_mangle]
pub extern "C" fn mutual_length(first: *const c_char, second: *const c_char) -> c_int {
   let c_first = unsafe {
      CStr::from_ptr(first)
   };

   let c_second = unsafe {
      CStr::from_ptr(second)
   };

   let first_slice = str::from_utf8(c_first.to_bytes()).unwrap();
   let second_slice = str::from_utf8(c_second.to_bytes()).unwrap();

   (first_slice.len() + second_slice.len()) as c_int
}
