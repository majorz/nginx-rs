#![feature(libc)]
#![feature(plugin)]

#![allow(dead_code)]
#![allow(unused_imports)]

extern crate libc;
#[macro_use] extern crate bitflags;

mod nginx;

use std::ffi::CString;
use std::ptr;
use std::mem;
use std::boxed;
use std::default::Default;

use nginx::ffi::{
   ngx_str_t, ngx_http_request_t, ngx_pcalloc, ngx_palloc, ngx_conf_t, ngx_command_t, ngx_http_core_module,
   ngx_http_conf_ctx_t, ngx_int_t, ngx_http_output_filter, ngx_chain_t, ngx_http_send_header, ngx_buf_t,
   ngx_uint_t, ngx_http_core_loc_conf_t, ngx_log_error_core,
};
use nginx::Status;

use libc::{size_t, c_void, c_uchar, c_char};


const NGX_CONF_OK: *const c_char = 0 as *const c_char;
const NGX_CONF_ERROR: *const c_char = -1 as *const c_char;



macro_rules! ngx_http_conf_get_module_loc_conf {
   ($cf:expr, $module:expr) => ({
      (*
         (
            (*
               (
                  (*$cf).ctx as *mut ngx_http_conf_ctx_t
               )
            ).loc_conf
         ).offset($module.ctx_index as isize)
      ) as *mut ngx_http_core_loc_conf_t
   })
}


macro_rules! ngx_alloc_buf {
   ($pool:expr) => (
      ngx_palloc($pool, mem::size_of::<ngx_buf_t>() as u64)
   )
}

macro_rules! ngx_calloc_buf {
   ($pool:expr) => (
      ngx_pcalloc($pool, mem::size_of::<ngx_buf_t>() as u64)
   )
}


macro_rules! log_error_core {
   ($level:expr, $log:expr, $err:expr, $fmt:expr, $( $arg:expr ),*) => (
      ngx_log_error_core($level, $log.raw(), $err, $fmt, $( $arg ),*)
   )
}


bitflags! {
   flags BufFlags: u32 {
      const FLAG_TEMPORARY        = 0b0000000000000001,
      const FLAG_MEMORY           = 0b0000000000000010,
      const FLAG_MMAP             = 0b0000000000000100,
      const FLAG_RECYCLED         = 0b0000000000001000,
      const FLAG_IN_FILE          = 0b0000000000010000,
      const FLAG_FLUSH            = 0b0000000000100000,
      const FLAG_SYNC             = 0b0000000001000000,
      const FLAG_LAST_BUF         = 0b0000000010000000,
      const FLAG_LAST_IN_CHAIN    = 0b0000000100000000,
      const FLAG_LAST_SHADOW      = 0b0000001000000000,
      const FLAG_TEMP_FILE        = 0b0000010000000000,
   }
}



macro_rules! simple_http_module_command {
   ($command:ident, $handler:ident) => (
      #[no_mangle]
      pub extern fn $command(
         cf: *mut ngx_conf_t,
         _: *mut ngx_command_t,
         _: *mut c_void
      ) -> *const c_char {
         unsafe {
            let clcf = ngx_http_conf_get_module_loc_conf!(cf, ngx_http_core_module);
            (*clcf).handler = Some($handler);
         }

         NGX_CONF_OK
      }
   )
}


simple_http_module_command!(ngx_http_sample_module_command, ngx_http_sample_handler);


#[no_mangle]
pub extern fn ngx_http_sample_handler(r: *mut ngx_http_request_t) -> ngx_int_t
{
   // TODO: return 500 on failures

   let mut request = nginx::HttpRequest::from_raw(r);

   let html = CString::new("<html><head><meta charset=\"utf-8\"></head><body>Здравейте!</body></html>").unwrap();
   let len = html.to_bytes().len();

   let mut headers_out = request.headers_out();

   headers_out.set_status(200);
   headers_out.set_content_length_n(len as i64);

   // TODO: rc == NGX_ERROR || rc > NGX_OK || (*r).header_only)
   let send_status = request.http_send_header();
   match send_status {
      Status::Error | Status::Http(_) => {
         return send_status.rc();
      }
      _ => {}
   }

   let mut buf_t = ngx_buf_t::default();

   let ptr = html.as_ptr() as *mut u8;

   buf_t.start = ptr;
   buf_t.pos = buf_t.start;

   buf_t.end = unsafe { ptr.offset(len as isize) };
   buf_t.last = buf_t.end;

   let buf_flags = FLAG_MEMORY | FLAG_LAST_BUF | FLAG_LAST_IN_CHAIN;
   buf_t._bindgen_bitfield_1_ = buf_flags.bits();

   let mut buf = nginx::Buf::take(buf_t);

   let mut chain = nginx::Chain::new(&mut buf, &mut None);

   let out = chain.raw();

   unsafe {
      ngx_http_output_filter(r, out)
   }
}
