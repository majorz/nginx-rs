pub mod ffi;
pub mod status;

pub use self::status::{Status, HttpStatus};

use std::result;
use std::mem;
use std::ptr;
use std::ffi::{CStr, CString};
use std::convert::From;
use std::default::Default;

use libc::c_long;


macro_rules! getter {
   ( $field:ident, $restype:ident ) => {
      pub fn $field(&mut self) -> Option<$restype> {
         let raw = unsafe { (*self.raw()).$field };

         if raw.is_null() {
            None
         } else {
            Some(
               $restype::from_raw(raw)
            )
         }
      }
   };
}


macro_rules! flag_pack_1 {
   ( $pos:expr, $getter:ident, $setter:ident, $clearer:ident, $toggler:ident ) => {
      flag_methods!($pos, $getter, $setter, $clearer, $toggler, _bindgen_bitfield_1_);
   };
}

macro_rules! flag_pack_2 {
   ( $pos:expr, $getter:ident, $setter:ident, $clearer:ident, $toggler:ident ) => {
      flag_methods!($pos, $getter, $setter, $clearer, $toggler, _bindgen_bitfield_2_);
   };
}

macro_rules! flag_pack_3 {
   ( $pos:expr, $getter:ident, $setter:ident, $clearer:ident, $toggler:ident ) => {
      flag_methods!($pos, $getter, $setter, $clearer, $toggler, _bindgen_bitfield_3_);
   };
}

macro_rules! flag_pack_4 {
   ( $pos:expr, $getter:ident, $setter:ident, $clearer:ident, $toggler:ident ) => {
      flag_methods!($pos, $getter, $setter, $clearer, $toggler, _bindgen_bitfield_4_);
   };
}

macro_rules! flag_pack_5 {
   ( $pos:expr, $getter:ident, $setter:ident, $clearer:ident, $toggler:ident ) => {
      flag_methods!($pos, $getter, $setter, $clearer, $toggler, _bindgen_bitfield_5_);
   };
}

macro_rules! flag_pack_6 {
   ( $pos:expr, $getter:ident, $setter:ident, $clearer:ident, $toggler:ident ) => {
      flag_methods!($pos, $getter, $setter, $clearer, $toggler, _bindgen_bitfield_6_);
   };
}

macro_rules! flag_pack_7 {
   ( $pos:expr, $getter:ident, $setter:ident, $clearer:ident, $toggler:ident ) => {
      flag_methods!($pos, $getter, $setter, $clearer, $toggler, _bindgen_bitfield_7_);
   };
}

macro_rules! flag_methods {
   ( $pos:expr, $getter:ident, $setter:ident, $clearer:ident, $toggler:ident, $pack:ident ) => {
      pub fn $getter(&mut self) -> bool {
         unsafe { (1 << $pos) & (*self.raw()).$pack != 0 }
      }

      pub fn $setter(&mut self) {
         let raw = self.raw();
         unsafe { (*raw).$pack = (1 << $pos) | (*raw).$pack };
      }

      pub fn $clearer(&mut self) {
         let raw = self.raw();
         unsafe { (*raw).$pack = !(1 << $pos) & (*raw).$pack };
      }

      pub fn $toggler(&mut self) {
         let raw = self.raw();
         unsafe { (*raw).$pack = (1 << $pos) ^ (*raw).$pack };
      }
   };
}


enum Value<T> {
   Raw(*mut T),
   Stack(T)
}


struct Wrapper<T> {
   value: Value<T>
}

impl<T> Wrapper<T> {
   pub fn from_raw(raw: *mut T) -> Self {
      Wrapper::<T> {
         value: Value::Raw(raw)
      }
   }

   pub fn take(item: T) -> Self {
      Wrapper::<T> {
         value: Value::Stack(item)
      }
   }

   pub fn raw(&mut self) -> *mut T {
      match self.value {
         Value::Raw(raw) => { raw }
         Value::Stack(ref mut val) => { val }
      }
   }
}


pub type Connection = Wrapper<ffi::ngx_connection_t>;

impl Connection {
   getter!(log, Log);
}


pub type Log = Wrapper<ffi::ngx_log_t>;


pub type HttpRequest = Wrapper<ffi::ngx_http_request_t>;

impl HttpRequest {
   pub fn dump_bitflags(&mut self) {
      unsafe {
         println!("B 1: {:064b}", (*self.raw())._bindgen_bitfield_1_);
         println!("B 2: {:064b}", (*self.raw())._bindgen_bitfield_2_);
         println!("B 3: {:064b}", (*self.raw())._bindgen_bitfield_3_);
      }
   }

   pub fn log_bitflags(&mut self) {
      unsafe {
         let log = (*(*self.raw()).connection).log;

         let template = "BUF: bit1=%s";
         let cstr_template = CString::new(template).unwrap();

         let b1 = CString::new(format!("{:064b}", (*self.raw())._bindgen_bitfield_1_)).unwrap();

         ffi::ngx_log_error_core(
            2, log, 0,
            cstr_template.as_ptr(),
            b1.as_ptr(),
         );
      }
   }

   getter!(connection, Connection);

   getter!(pool, Pool);

   flag_pack_1!(24, aio, set_aio,
      clear_aio, toggle_aio);

   flag_pack_1!(29, complex_uri, set_complex_uri,
      clear_complex_uri, toggle_complex_uri);

   flag_pack_1!(30, quoted_uri, set_quoted_uri,
      clear_quoted_uri, toggle_quoted_uri);

   flag_pack_1!(31, plus_in_uri, set_plus_in_uri,
      clear_plus_in_uri, toggle_plus_in_uri);

   flag_pack_2!(0, space_in_uri, set_space_in_uri,
      clear_space_in_uri, toggle_space_in_uri);

   flag_pack_2!(1, invalid_header, set_invalid_header,
      clear_invalid_header, toggle_invalid_header);

   flag_pack_2!(2, add_uri_to_alias, set_add_uri_to_alias,
      clear_add_uri_to_alias, toggle_add_uri_to_alias);

   flag_pack_2!(3, valid_location, set_valid_location,
      clear_valid_location, toggle_valid_location);

   flag_pack_2!(4, valid_unparsed_uri, set_valid_unparsed_uri,
      clear_valid_unparsed_uri, toggle_valid_unparsed_uri);

   flag_pack_2!(5, uri_changed, set_uri_changed,
      clear_uri_changed, toggle_uri_changed);

   flag_pack_2!(10, request_body_in_single_buf, set_request_body_in_single_buf,
      clear_request_body_in_single_buf, toggle_request_body_in_single_buf);

   flag_pack_2!(11, request_body_in_file_only, set_request_body_in_file_only,
      clear_request_body_in_file_only, toggle_request_body_in_file_only);

   flag_pack_2!(12, request_body_in_persistent_file, set_request_body_in_persistent_file,
      clear_request_body_in_persistent_file, toggle_request_body_in_persistent_file);

   flag_pack_2!(13, request_body_in_clean_file, set_request_body_in_clean_file,
      clear_request_body_in_clean_file, toggle_request_body_in_clean_file);

   flag_pack_2!(14, request_body_file_group_access, set_request_body_file_group_access,
      clear_request_body_file_group_access, toggle_request_body_file_group_access);

   flag_pack_2!(18, request_body_no_buffering, set_request_body_no_buffering,
      clear_request_body_no_buffering, toggle_request_body_no_buffering);

   flag_pack_2!(19, subrequest_in_memory, set_subrequest_in_memory,
      clear_subrequest_in_memory, toggle_subrequest_in_memory);

   flag_pack_2!(20, waited, set_waited,
      clear_waited, toggle_waited);

   flag_pack_2!(21, cached, set_cached,
      clear_cached, toggle_cached);

   flag_pack_2!(22, gzip_tested, set_gzip_tested,
      clear_gzip_tested, toggle_gzip_tested);

   flag_pack_2!(23, gzip_ok, set_gzip_ok,
      clear_gzip_ok, toggle_gzip_ok);

   flag_pack_2!(24, gzip_vary, set_gzip_vary,
      clear_gzip_vary, toggle_gzip_vary);

   flag_pack_2!(25, proxy, set_proxy,
      clear_proxy, toggle_proxy);

   flag_pack_2!(26, bypass_cache, set_bypass_cache,
      clear_bypass_cache, toggle_bypass_cache);

   flag_pack_2!(27, no_cache, set_no_cache,
      clear_no_cache, toggle_no_cache);

   flag_pack_2!(28, limit_conn_set, set_limit_conn_set,
      clear_limit_conn_set, toggle_limit_conn_set);

   flag_pack_2!(29, limit_req_set, set_limit_req_set,
      clear_limit_req_set, toggle_limit_req_set);

   flag_pack_2!(30, pipeline, set_pipeline,
      clear_pipeline, toggle_pipeline);

   flag_pack_2!(31, chunked, set_chunked,
      clear_chunked, toggle_chunked);

   flag_pack_3!(0, header_only, set_header_only,
      clear_header_only, toggle_header_only);

   flag_pack_3!(1, keepalive, set_keepalive,
      clear_keepalive, toggle_keepalive);

   flag_pack_3!(0, lingering_close, set_lingering_close,
      clear_lingering_close, toggle_lingering_close);

   flag_pack_3!(1, discard_body, set_discard_body,
      clear_discard_body, toggle_discard_body);

   flag_pack_3!(2, reading_body, set_reading_body,
      clear_reading_body, toggle_reading_body);

   flag_pack_3!(3, internal, set_internal,
      clear_internal, toggle_internal);

   flag_pack_3!(4, error_page, set_error_page,
      clear_error_page, toggle_error_page);

   flag_pack_3!(5, filter_finalize, set_filter_finalize,
      clear_filter_finalize, toggle_filter_finalize);

   flag_pack_3!(6, post_action, set_post_action,
      clear_post_action, toggle_post_action);

   flag_pack_3!(7, request_complete, set_request_complete,
      clear_request_complete, toggle_request_complete);

   flag_pack_3!(8, request_output, set_request_output,
      clear_request_output, toggle_request_output);

   flag_pack_3!(9, header_sent, set_header_sent,
      clear_header_sent, toggle_header_sent);

   flag_pack_3!(10, expect_tested, set_expect_tested,
      clear_expect_tested, toggle_expect_tested);

   flag_pack_3!(11, root_tested, set_root_tested,
      clear_root_tested, toggle_root_tested);

   flag_pack_3!(12, done, set_done,
      clear_done, toggle_done);

   flag_pack_3!(13, logged, set_logged,
      clear_logged, toggle_logged);

   flag_pack_3!(18, main_filter_need_in_memory, set_main_filter_need_in_memory,
      clear_main_filter_need_in_memory, toggle_main_filter_need_in_memory);

   flag_pack_3!(19, filter_need_in_memory, set_filter_need_in_memory,
      clear_filter_need_in_memory, toggle_filter_need_in_memory);

   flag_pack_3!(20, filter_need_temporary, set_filter_need_temporary,
      clear_filter_need_temporary, toggle_filter_need_temporary);

   flag_pack_3!(21, allow_ranges, set_allow_ranges,
      clear_allow_ranges, toggle_allow_ranges);

   flag_pack_3!(22, single_range, set_single_range,
      clear_single_range, toggle_single_range);

   flag_pack_3!(23, disable_not_modified, set_disable_not_modified,
      clear_disable_not_modified, toggle_disable_not_modified);

   pub fn headers_out(&mut self) -> HttpHeadersOut {
      let raw: *mut ffi::ngx_http_headers_out_t = unsafe { &mut (*self.raw()).headers_out };

      HttpHeadersOut::from_raw(raw)
   }

   pub fn http_send_header(&mut self) -> Status {
      let rc = unsafe { ffi::ngx_http_send_header(self.raw()) };
      Status::new(rc)
   }

   pub fn http_output_filter(&mut self, chain: &mut Chain) -> Status {
      let rc = unsafe { ffi::ngx_http_output_filter(self.raw(), chain.raw()) };
      Status::new(rc)
   }
}


pub type HttpHeadersOut = Wrapper<ffi::ngx_http_headers_out_t>;

impl HttpHeadersOut {
   pub fn set_status(&mut self, http_status: HttpStatus) {
      unsafe {
         (*self.raw()).status = http_status.rc() as ffi::ngx_uint_t;
      };

   }
   pub fn set_content_length_n(&mut self, len: usize) {
      unsafe {
         (*self.raw()).content_length_n = len as ffi::off_t;
      };
   }
}


pub type Pool = Wrapper<ffi::ngx_pool_t>;

impl Pool {
   pub fn alloc_buf(&mut self) -> Option<Buf> {
      match self.raw_palloc::<ffi::ngx_buf_t>() {
         None => { None }
         Some(raw) => {
            Some(
               Buf::from_raw(raw)
            )
         }
      }
   }

   pub fn nalloc_buf(&mut self) -> Option<Buf> {
      match self.raw_pnalloc::<ffi::ngx_buf_t>() {
         None => { None }
         Some(raw) => {
            Some(
               Buf::from_raw(raw)
            )
         }
      }
   }

   pub fn calloc_buf(&mut self) -> Option<Buf> {
      match self.raw_pcalloc::<ffi::ngx_buf_t>() {
         None => { None }
         Some(raw) => {
            Some(
               Buf::from_raw(raw)
            )
         }
      }
   }

   #[inline]
   pub fn raw_palloc<T>(&mut self) -> Option<*mut T> {
      let raw = unsafe {
         ffi::ngx_palloc(self.raw(), mem::size_of::<T>() as ffi::size_t) as *mut T
      };

      if raw.is_null() {
         None
      } else {
         Some(raw)
      }
   }

   #[inline]
   pub fn raw_pnalloc<T>(&mut self) -> Option<*mut T> {
      let raw = unsafe {
         ffi::ngx_pnalloc(self.raw(), mem::size_of::<T>() as ffi::size_t) as *mut T
      };

      if raw.is_null() {
         None
      } else {
         Some(raw)
      }
   }

   #[inline]
   pub fn raw_pcalloc<T>(&mut self) -> Option<*mut T> {
      let raw = unsafe {
         ffi::ngx_pcalloc(self.raw(), mem::size_of::<T>() as ffi::size_t) as *mut T
      };

      if raw.is_null() {
         None
      } else {
         Some(raw)
      }
   }

}

pub type Buf = Wrapper<ffi::ngx_buf_t>;

impl Buf {
   flag_pack_1!(0, temporary, set_temporary, clear_temporary, toggle_temporary);

   flag_pack_1!(1, memory, set_memory, clear_memory, toggle_memory);

   flag_pack_1!(2, mmap, set_mmap, clear_mmap, toggle_mmap);

   flag_pack_1!(3, in_file, set_in_file, clear_in_file, toggle_in_file);

   flag_pack_1!(4, flush, set_flush, clear_flush, toggle_flush);

   flag_pack_1!(5, sync, set_sync, clear_sync, toggle_sync);

   flag_pack_1!(6, last_buf, set_last_buf, clear_last_buf, toggle_last_buf);

   flag_pack_1!(7, last_in_chain, set_last_in_chain, clear_last_in_chain, toggle_last_in_chain);

   flag_pack_1!(8, last_shadow, set_last_shadow, clear_last_shadow, toggle_last_shadow);

   flag_pack_1!(9, temp_file, set_temp_file, clear_temp_file, toggle_temp_file);
}

impl<'a> From<&'a CString> for Buf {
   fn from(s: &CString) -> Self {
      let ptr = s.as_ptr() as *mut u8;
      let len = s.to_bytes().len();

      let mut ngx_buf = ffi::ngx_buf_t::default();

      ngx_buf.start = ptr;
      ngx_buf.pos = ngx_buf.start;

      ngx_buf.end = unsafe { ptr.offset(len as isize) };
      ngx_buf.last = ngx_buf.end;

      let mut buf = Buf::take(ngx_buf);

      buf.set_memory();

      buf
   }
}

pub type Chain = Wrapper<ffi::ngx_chain_t>;

impl Chain {
   pub fn new(buf: &mut Buf, next: &mut Option<Chain>) -> Self {
      let next_raw = match *next {
         None => { ptr::null_mut() }
         Some(ref mut chain) => { chain.raw() }
      };

      let val = ffi::ngx_chain_t {
         buf: buf.raw(),
         next: next_raw
      };

      Chain {
         value: Value::Stack(val)
      }
   }
}

pub type Str = Wrapper<ffi::ngx_str_t>;
