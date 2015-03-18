#![feature(libc)]
#![feature(plugin)]

#![plugin(maud_macros)]

#![allow(dead_code)]
#![allow(unused_imports)]

extern crate libc;
extern crate maud;
#[macro_use] #[no_link] extern crate rustc_bitflags;

mod ffi;

use std::ffi::CString;
use std::ptr;
use std::ptr::PtrExt;
use std::mem;

use ffi::nginx::{
   ngx_str_t, ngx_http_request_t, ngx_pcalloc, ngx_palloc, ngx_conf_t, ngx_command_t, ngx_http_core_module,
   ngx_http_conf_ctx_t, ngx_int_t, ngx_http_output_filter, ngx_chain_t, ngx_http_send_header, ngx_buf_t,
   ngx_uint_t, ngx_http_core_loc_conf_t, ngx_conf_log_error,
};

use libc::{size_t, c_void, c_uchar, c_char};


const NGX_CONF_OK: *const c_void = 0 as *const c_void;
const NGX_CONF_ERROR: *const c_void = -1 as *const c_void;

const NGX_OK:        ngx_int_t =  0;
const NGX_ERROR:     ngx_int_t = -1;
const NGX_AGAIN:     ngx_int_t = -2;
const NGX_BUSY:      ngx_int_t = -3;
const NGX_DONE:      ngx_int_t = -4;
const NGX_DECLINED:  ngx_int_t = -5;
const NGX_ABORT:     ngx_int_t = -6;


const NGX_HTTP_OK:                      ngx_uint_t = 200;
const NGX_HTTP_INTERNAL_SERVER_ERROR:   ngx_uint_t = 500;


// #define ngx_http_conf_get_module_loc_conf(cf, module)                         \
//    ((ngx_http_conf_ctx_t *) cf->ctx)->loc_conf[module.ctx_index]
macro_rules! ngx_http_conf_get_module_loc_conf {
   ($cf:ident, $module:ident) => (
      unsafe {
         (*
            (
               (*
                  (
                     (*cf).ctx as *mut ngx_http_conf_ctx_t
                  )
               ).loc_conf
            ).offset(ngx_http_core_module.ctx_index as isize)
         )
      }
   )
}


macro_rules! ngx_alloc_buf {
   ($pool:expr) => (
      ngx_palloc($pool, mem::size_of::<ngx_buf_t>())
   )
}

macro_rules! ngx_calloc_buf {
   ($pool:expr) => (
      ngx_pcalloc($pool, mem::size_of::<ngx_buf_t>())
   )
}



#[no_mangle]
pub extern fn ngx_http_sample_module_command(
   cf: *mut ngx_conf_t,
   cmd: *mut ngx_command_t,
   conf: *mut c_void
) -> *mut c_char {
   //ngx_http_core_loc_conf_t  *clcf;

   unsafe {
      let clcf: *mut ngx_http_core_loc_conf_t = (*((*((*cf).ctx as *mut ngx_http_conf_ctx_t)).loc_conf).offset(ngx_http_core_module.ctx_index as isize)) as *mut ngx_http_core_loc_conf_t;
      (*clcf).handler = Some(ngx_http_sample_handler_wrapper);

      let s = CString::new("DUMP  name: %V").unwrap();

      ngx_conf_log_error(
         1, cf, 0,
         s.as_ptr(),
         &(*clcf).name);
   }

   //return NGX_CONF_OK as *mut c_char;
   NGX_CONF_OK as *mut c_char
}

extern {
   fn ngx_http_sample_handler(r: *mut ngx_http_request_t) -> ngx_int_t;
}


#[no_mangle]
pub extern fn ngx_http_sample_handler_wrapper(r: *mut ngx_http_request_t) -> ngx_int_t
{
   unsafe {
      ngx_http_sample_handler(r)
   }
}

#[no_mangle]
pub extern fn sample_text_from_rust(r: *const ngx_http_request_t) -> ngx_str_t
{
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
      data: unsafe {
         ngx_pcalloc((*r).pool, len) as *mut u8
      },
   };

   unsafe {
      ptr::copy_nonoverlapping(result.data, s.as_ptr() as *const u8, len as usize);
   }

   result
}
