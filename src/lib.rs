#![feature(libc)]

extern crate libc;

mod ffi;

use std::ffi::CString;

use ffi::nginx::{ngx_str_t, ngx_http_request_t, ngx_pool_t};

use libc::{c_void, size_t};


extern {
   pub fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
}


#[no_mangle]
pub extern fn sample_text_from_rust(
   r: *const ngx_http_request_t,
   ngx_pcalloc: extern fn(pool: *mut ngx_pool_t, size: size_t) -> *mut c_void
) -> ngx_str_t
{
   let s = CString::new("hellohihi").unwrap();

   let result = ngx_str_t {
      len: 9,
      data: unsafe { ngx_pcalloc((*r).pool, 10) as *mut u8 },
   };

   unsafe {
      memcpy(result.data as *mut c_void, s.as_ptr() as *const c_void, 9);
   }

   result
}
