pub mod ffi;

use std::result;

use libc::c_long;


const NGX_HTTP_OK:                      ffi::ngx_int_t = 200;
const NGX_HTTP_INTERNAL_SERVER_ERROR:   ffi::ngx_int_t = 500;


const NGX_OK:        ffi::ngx_int_t =  0;
const NGX_ERROR:     ffi::ngx_int_t = -1;
const NGX_AGAIN:     ffi::ngx_int_t = -2;
const NGX_BUSY:      ffi::ngx_int_t = -3;
const NGX_DONE:      ffi::ngx_int_t = -4;
const NGX_DECLINED:  ffi::ngx_int_t = -5;
const NGX_ABORT:     ffi::ngx_int_t = -6;


pub enum HttpStatus {
   Ok,
   InternalServerError,
}

impl HttpStatus {
   pub fn new(rc: ffi::ngx_int_t) -> Self {
      match rc {
         NGX_HTTP_OK => {
            HttpStatus::Ok }
         NGX_HTTP_INTERNAL_SERVER_ERROR => {
            HttpStatus::InternalServerError }
         _ => {
            HttpStatus::InternalServerError }
      }
   }

   pub fn rc(&self) -> ffi::ngx_int_t {
      match *self {
         HttpStatus::Ok => {
            NGX_HTTP_OK }
         HttpStatus::InternalServerError => {
            NGX_HTTP_INTERNAL_SERVER_ERROR }
      }
   }
}


pub enum Status {
   Ok,
   Error,
   Again,
   Busy,
   Done,
   Declined,
   Abort,
   Http(HttpStatus),
}

impl Status {
   pub fn new(rc: ffi::ngx_int_t) -> Self {
      match rc {
         NGX_OK => {
            Status::Ok }
         http_rc if rc > 0 => {
            Status::Http(HttpStatus::new(http_rc)) }
         _ => {
            Status::Error }
      }
   }

   pub fn rc(&self) -> ffi::ngx_int_t {
      match *self {
         Status::Ok => {
            NGX_OK }
         Status::Http(ref http_status) => {
            http_status.rc() }
         _ => {
            NGX_ERROR }
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
