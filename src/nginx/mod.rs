pub mod ffi;

use std::result;

use libc::c_char;


const NGX_OK:        ffi::ngx_int_t =  0;
const NGX_ERROR:     ffi::ngx_int_t = -1;
const NGX_AGAIN:     ffi::ngx_int_t = -2;
const NGX_BUSY:      ffi::ngx_int_t = -3;
const NGX_DONE:      ffi::ngx_int_t = -4;
const NGX_DECLINED:  ffi::ngx_int_t = -5;
const NGX_ABORT:     ffi::ngx_int_t = -6;


pub type Result = result::Result<(), ffi::ngx_int_t>;


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
      let raw: *mut ffi::ngx_http_headers_out_t = &mut unsafe { (*self.raw).headers_out };

      HttpHeadersOut::new(raw)
   }

   pub fn http_send_header(&self) -> Result {
      let rc = unsafe { ffi::ngx_http_send_header(self.raw) };

      if rc == NGX_OK {
         Ok(())
      } else {
         Err(rc)
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
