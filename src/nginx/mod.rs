pub mod ffi;
pub mod status;

pub use self::status::Status;

use std::result;
use std::mem;

use libc::c_long;


pub struct Connection {
   raw: *mut ffi::ngx_connection_t,
}

impl Connection {
   pub fn new(raw: *mut ffi::ngx_connection_t) -> Self {
      Connection {
         raw: raw
      }
   }

   pub fn log(&self) -> Option<Log> {
      let raw = unsafe { (*self.raw).log };

      if raw.is_null() {
         None
      } else {
         Some(
            Log::new(raw)
         )
      }
   }
}


pub struct Log {
   pub raw: *mut ffi::ngx_log_t,
}

impl Log {
   pub fn new(raw: *mut ffi::ngx_log_t) -> Self {
      Log {
         raw: raw
      }
   }
}


pub struct HttpRequest {
   raw: *mut ffi::ngx_http_request_t,
}

impl HttpRequest {
   pub fn new(raw: *mut ffi::ngx_http_request_t) -> Self {
      HttpRequest {
         raw: raw
      }
   }

   pub fn connection(&self) -> Option<Connection> {
      let raw = unsafe { (*self.raw).connection };

      if raw.is_null() {
         None
      } else {
         Some(
            Connection::new(raw)
         )
      }
   }

   pub fn headers_out(&self) -> HttpHeadersOut {
      let raw: *mut ffi::ngx_http_headers_out_t = unsafe { &mut (*self.raw).headers_out };

      HttpHeadersOut::new(raw)
   }

   pub fn http_send_header(&self) -> Status {
      let rc = unsafe { ffi::ngx_http_send_header(self.raw) };
      Status::new(rc)
   }

   pub fn pool(&self) -> Option<Pool> {
      let raw = unsafe { (*self.raw).pool };

      if raw.is_null() {
         None
      } else {
         Some(
            Pool::new(raw)
         )
      }
   }
}


pub struct HttpHeadersOut {
   raw: *mut ffi::ngx_http_headers_out_t,
}

impl HttpHeadersOut {
   pub fn new(raw: *mut ffi::ngx_http_headers_out_t) -> Self {
      HttpHeadersOut {
         raw: raw
      }
   }

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


pub struct Pool {
   raw: *mut ffi::ngx_pool_t,
}

impl Pool {
   pub fn new(raw: *mut ffi::ngx_pool_t) -> Self {
      Pool {
         raw: raw
      }
   }

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
