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


use std::num::Int;

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
         unsafe { 2.pow($pos) & (*self.raw()).$pack != 0 }
      }

      pub fn $setter(&mut self) {
         let raw = self.raw();
         unsafe { (*raw).$pack = 2.pow($pos) | (*raw).$pack };
      }

      pub fn $clearer(&mut self) {
         let raw = self.raw();
         unsafe { (*raw).$pack = 2.pow($pos) | (*raw).$pack };
      }

      pub fn $toggler(&mut self) {
         let raw = self.raw();
         unsafe { (*raw).$pack = 2.pow($pos) | (*raw).$pack };
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
   getter!(connection, Connection);
   getter!(pool, Pool);

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
