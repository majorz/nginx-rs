pub mod ffi;
pub mod status;

pub use self::status::Status;

use std::result;
use std::mem;

use libc::c_long;


macro_rules! getter {
   ( $field:ident, $restype:ident ) => {
      pub fn $field(&self) -> Option<$restype> {
         let raw = unsafe { (*self.raw).$field };

         if raw.is_null() {
            None
         } else {
            Some(
               $restype::new(raw)
            )
         }
      }
   };
}


struct Wrapper<R> {
   pub raw: *mut R,
}

impl<R> Wrapper<R> {
   pub fn new(raw: *mut R) -> Self {
      Wrapper::<R> {
         raw: raw
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

   pub fn headers_out(&self) -> HttpHeadersOut {
      let raw: *mut ffi::ngx_http_headers_out_t = unsafe { &mut (*self.raw).headers_out };

      HttpHeadersOut::new(raw)
   }

   pub fn http_send_header(&self) -> Status {
      let rc = unsafe { ffi::ngx_http_send_header(self.raw) };
      Status::new(rc)
   }
}


pub type HttpHeadersOut = Wrapper<ffi::ngx_http_headers_out_t>;

impl HttpHeadersOut {
   pub fn set_status(&mut self, status: ffi::ngx_uint_t) {
      unsafe {
         (*self.raw).status = status;
      };
   }

   pub fn set_content_length_n(&mut self, content_length_n: ffi::off_t) {
      unsafe {
         (*self.raw).content_length_n = content_length_n;
      };
   }
}


pub type Pool = Wrapper<ffi::ngx_pool_t>;

impl Pool {
   #[inline]
   pub fn raw_palloc<T>(&self) -> *mut T {
      unsafe {
         ffi::ngx_palloc(self.raw, mem::size_of::<T>() as ffi::size_t) as *mut T
      }
   }

   #[inline]
   pub fn raw_pnalloc<T>(&self) -> *mut T {
      unsafe {
         ffi::ngx_pnalloc(self.raw, mem::size_of::<T>() as ffi::size_t) as *mut T
      }
   }

   #[inline]
   pub fn raw_pcalloc<T>(&self) -> *mut T {
      unsafe {
         ffi::ngx_pcalloc(self.raw, mem::size_of::<T>() as ffi::size_t) as *mut T
      }
   }

}
