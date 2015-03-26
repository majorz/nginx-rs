pub mod ffi;
pub mod status;

pub use self::status::Status;

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
   pub fn set_status(&mut self, status: ffi::ngx_uint_t) {
      unsafe {
         (*self.raw()).status = status;
      };
   }

   pub fn set_content_length_n(&mut self, content_length_n: ffi::off_t) {
      unsafe {
         (*self.raw()).content_length_n = content_length_n;
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
   pub fn flag_memory(&mut self) {
      self.set_bitflag_1(0b0000000000000010);
   }

   pub fn memory(&mut self) -> bool {
      self.get_bitflag_1(0b0000000000000010)
   }

   pub fn flag_last_buf(&mut self) {
      self.set_bitflag_1(0b0000000010000000);
   }

   pub fn last_buf(&mut self) -> bool {
      self.get_bitflag_1(0b0000000010000000)
   }

   pub fn flag_last_in_chain(&mut self) {
      self.set_bitflag_1(0b0000000100000000);
   }

   pub fn last_in_chain(&mut self) -> bool {
      self.get_bitflag_1(0b0000000100000000)
   }

   fn set_bitflag_1(&mut self, flag: ::libc::c_uint) {
      let raw = self.raw();
      unsafe { (*raw)._bindgen_bitfield_1_ = flag | (*raw)._bindgen_bitfield_1_ };
   }

   fn get_bitflag_1(&mut self, flag: ::libc::c_uint) -> bool {
      unsafe { flag & (*self.raw())._bindgen_bitfield_1_ != 0 }
   }
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

      buf.flag_memory();

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
