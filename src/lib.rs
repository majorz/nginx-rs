#![feature(libc)]
#![feature(plugin)]

#![plugin(maud_macros)]

extern crate libc;
extern crate maud;

mod ffi;

use std::ffi::CString;
use std::ptr;

use ffi::nginx::{ngx_str_t, ngx_http_request_t, ngx_pool_t};

use libc::{c_void, size_t};


pub type NgxPcalloc = extern fn(pool: *mut ngx_pool_t, size: size_t) -> *mut c_void;
static mut ngx_pcalloc: Option<NgxPcalloc> = None;


#[no_mangle]
pub extern fn sample_text_from_rust(r: *const ngx_http_request_t, pcalloc: NgxPcalloc) -> ngx_str_t
{
   unsafe {
      ngx_pcalloc = Some(pcalloc);
   }

   let name = "Nginx-Rust";
   let markup = html! {
      html {
         head meta charset="utf-8"
         body {
            p { "Здравейте, " $name "!" }
         }
      }
   };

   let s = CString::new(markup.to_string()).unwrap();
   let len = s.to_bytes().len() as size_t;

   let result = ngx_str_t {
      len: len,
      data: unsafe { ngx_pcalloc.unwrap()((*r).pool, len) as *mut u8 },
   };

   unsafe {
      ptr::copy_nonoverlapping(result.data, s.as_ptr() as *const u8, len as usize);
   }

   result
}
