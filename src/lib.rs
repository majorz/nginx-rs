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
   let mut request = nginx::HttpRequest::from_raw(r);

   let mut connection = request.connection().unwrap();

   let mut log = connection.log().unwrap();

   let ngx_http_sample_text: ngx_str_t = sample_text_from_rust(r);

   let mut headers_out = request.headers_out();

   headers_out.set_status(200);
   headers_out.set_content_length_n(ngx_http_sample_text.len as i64);

   match request.http_send_header() {
      Status::Error => {
         return Status::Error.rc();
      }
      Status::Http(http_status) => {
         return Status::Http(http_status).rc();
      }
      _ => {}
   }
   //if rc == NGX_ERROR || rc > NGX_OK { //|| (*r).header_only) {

   let mut pool = request.pool().unwrap();

   let mut buf = pool.calloc_buf().unwrap(); // return 500 on error

   unsafe {
      let mut chain = ngx_chain_t {
         buf: buf.raw(),
         next: ptr::null_mut()
      };

      let out: *mut ngx_chain_t = (&mut chain) as *mut ngx_chain_t;

      let headers_in = (*r).headers_in;

      let cstr_template = CString::new("BITS: \"%s\"").unwrap();
      let b1 = CString::new(format!("{:032b}", headers_in._bindgen_bitfield_1_)).unwrap();

      log_error_core!(
         2, log, 0,
         cstr_template.as_ptr(),
         b1.as_ptr()
      );

      (*buf.raw()).start = ngx_http_sample_text.data;
      (*buf.raw()).pos = (*buf.raw()).start;

      (*buf.raw()).end = ngx_http_sample_text.data.offset(ngx_http_sample_text.len as isize);
      (*buf.raw()).last = (*buf.raw()).end;

      let buf_flags = FLAG_MEMORY | FLAG_LAST_BUF | FLAG_LAST_IN_CHAIN;
      (*buf.raw())._bindgen_bitfield_1_ = buf_flags.bits();

      ngx_http_output_filter(r, out)
   }
}


fn sample_text_from_rust(r: *mut ngx_http_request_t) -> ngx_str_t
{
   let s = CString::new("<html><head><meta charset=\"utf-8\"></head><body>Здравейте!</body></html>".to_string()).unwrap();
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

#[no_mangle]
pub extern fn dump_request(r: *mut ngx_http_request_t) {
   unsafe {
      let log = (*(*r).connection).log;

      let template = "REQ: \
         method=\"%ud\" \
         http_version=\"%ud\" \
         request_line=\"%V\" \
         uri=\"%V\" \
         args=\"%V\" \
         exten=\"%V\" \
         unparsed_uri=\"%V\" \
         method_name=\"%V\" \
         http_protocol=\"%V\" \
         bit1=\"%s\" \
         bit2=\"%s\" \
         bit3=\"%s\" \
         bit4=\"%s\" \
         bits=\"%s\" \
      ";
      let cstr_template = CString::new(template).unwrap();

      let b1 = CString::new(format!("{:032b}", (*r)._bindgen_bitfield_1_)).unwrap();
      let b2 = CString::new(format!("{:032b}", (*r)._bindgen_bitfield_2_)).unwrap();
      let b3 = CString::new(format!("{:032b}", (*r)._bindgen_bitfield_3_)).unwrap();
      let b4 = CString::new(format!("{:032b}", (*r)._bindgen_bitfield_4_)).unwrap();

      let buf_flags = FLAG_MEMORY | FLAG_LAST_BUF | FLAG_LAST_IN_CHAIN;
      let bits = CString::new(format!("{:032b}", buf_flags.bits())).unwrap();

      ngx_log_error_core(
         2, log, 0,
         cstr_template.as_ptr(),
         (*r).method,
         (*r).http_version,
         &((*r).request_line),
         &((*r).uri),
         &((*r).args),
         &((*r).exten),
         &((*r).unparsed_uri),
         &((*r).method_name),
         &((*r).http_protocol),
         b1.as_ptr(),
         b2.as_ptr(),
         b3.as_ptr(),
         b4.as_ptr(),
         bits.as_ptr(),
      );
   }
}


#[no_mangle]
pub extern fn dump_buffer(r: *mut ngx_http_request_t, b: *mut ngx_buf_t) {
   unsafe {
      let log = (*(*r).connection).log;

      let template = "BUF: \
         pos=\"%p\" \
         last=\"%p\" \
         start=\"%p\" \
         end=\"%p\" \
         num=\"%d\" \
         bit1=\"%s\" \
      ";
      let cstr_template = CString::new(template).unwrap();

      let b1 = CString::new(format!("{:064b}", (*b)._bindgen_bitfield_1_)).unwrap();

      ngx_log_error_core(
         2, log, 0,
         cstr_template.as_ptr(),
         (*b).pos,
         (*b).last,
         (*b).start,
         (*b).end,
         (*b).num,
         b1.as_ptr(),
      );
   }
}
