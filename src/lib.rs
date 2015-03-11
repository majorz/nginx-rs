#![feature(libc)]

extern crate libc;

mod ffi;

use std::ffi::CString;

use ffi::nginx::{ngx_str_t, ngx_http_request_t, ngx_pool_t, size_t};


#[no_mangle]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8,
                            n: usize) -> *mut u8 {
   let mut i = 0;
   while i < n {
      *dest.offset(i as isize) = *src.offset(i as isize);
      i += 1;
   }
   return dest;
}


#[no_mangle]
pub extern fn sample_text_from_rust(
   r: *const ngx_http_request_t,
   ngx_pcalloc: extern fn(pool: *mut ngx_pool_t, size: size_t) -> *mut ::libc::c_void
) -> ngx_str_t
{
   let s = CString::new("hellohihi").unwrap();

   let result = ngx_str_t {
      len: 9,
      data: unsafe { ngx_pcalloc((*r).pool, 10) as *mut u8 },
   };

   unsafe {
      memcpy(result.data, s.as_ptr() as *const u8, 9);
   }

   result
}
