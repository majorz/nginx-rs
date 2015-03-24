pub mod ffi;

use libc::c_char;


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
      HttpHeadersOut::new(self.raw)
   }
}


pub struct HttpHeadersOut {
   request_raw: *mut ffi::ngx_http_request_t,
}

impl HttpHeadersOut {
   pub fn new(request_raw: *mut ffi::ngx_http_request_t) -> Self {
      HttpHeadersOut {
         request_raw: request_raw
      }
   }

   pub fn set_status(&mut self, status: ffi::ngx_uint_t) {
      unsafe {
         (*self.request_raw).headers_out.status = status;
      };
   }

   pub fn set_content_length_n(&mut self, content_length_n: ffi::off_t) {
      unsafe {
         (*self.request_raw).headers_out.content_length_n = content_length_n;
      };
   }
}
