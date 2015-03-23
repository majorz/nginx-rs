pub mod ffi;

use libc::c_char;


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
}


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
