pub mod ffi;

use libc::c_char;


pub struct HttpRequest {
   pub ptr: *mut ffi::ngx_http_request_t,
}

impl HttpRequest {
   pub fn connection(&self) -> Option<Connection> {
      let connection_ptr = unsafe { (*self.ptr).connection };

      if connection_ptr.is_null() {
         None
      } else {
         Some(Connection {
            ptr: connection_ptr,
         })
      }
   }
}


pub struct Connection {
   pub ptr: *mut ffi::ngx_connection_t,
}

impl Connection {
   pub fn log(&self) -> Option<Log> {
      let log_ptr = unsafe { (*self.ptr).log };

      if log_ptr.is_null() {
         None
      } else {
         Some(Log {
            ptr: log_ptr,
         })
      }
   }
}


pub struct Log {
   pub ptr: *mut ffi::ngx_log_t,
}

impl Log {

}
