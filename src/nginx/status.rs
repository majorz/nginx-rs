use nginx::ffi;


pub enum HttpStatus {
   Continue,
   SwitchingProtocols,
   Processing,
   Ok,
   Created,
   Accepted,
   NoContent,
   PartialContent,
   SpecialResponse,
   MovedPermanently,
   MovedTemporarily,
   SeeOther,
   NotModified,
   TemporaryRedirect,
   BadRequest,
   Unauthorized,
   Forbidden,
   NotFound,
   NotAllowed,
   RequestTimeOut,
   Conflict,
   LengthRequired,
   PreconditionFailed,
   RequestUriTooLarge,
   RequestEntityTooLarge,
   UnsupportedMediaType,
   RangeNotSatisfiable,
   Close,
   RequestHeaderTooLarge,
   CertError,
   NoCert,
   ToHttps,
   ClientClosedRequest,
   InternalServerError,
   NotImplement,
   BadGateway,
   ServiceUnavailable,
   GatewayTimeOut,
   InsufficientStorage,
}

impl HttpStatus {
   pub fn new(rc: ffi::ngx_int_t) -> Self {
      match rc {
         ffi::NGX_HTTP_CONTINUE => {
            HttpStatus::Continue }
         ffi::NGX_HTTP_SWITCHING_PROTOCOLS => {
            HttpStatus::SwitchingProtocols }
         ffi::NGX_HTTP_PROCESSING => {
            HttpStatus::Processing }
         ffi::NGX_HTTP_OK => {
            HttpStatus::Ok }
         ffi::NGX_HTTP_CREATED => {
            HttpStatus::Created }
         ffi::NGX_HTTP_ACCEPTED => {
            HttpStatus::Accepted }
         ffi::NGX_HTTP_NO_CONTENT => {
            HttpStatus::NoContent }
         ffi::NGX_HTTP_PARTIAL_CONTENT => {
            HttpStatus::PartialContent }
         ffi::NGX_HTTP_SPECIAL_RESPONSE => {
            HttpStatus::SpecialResponse }
         ffi::NGX_HTTP_MOVED_PERMANENTLY => {
            HttpStatus::MovedPermanently }
         ffi::NGX_HTTP_MOVED_TEMPORARILY => {
            HttpStatus::MovedTemporarily }
         ffi::NGX_HTTP_SEE_OTHER => {
            HttpStatus::SeeOther }
         ffi::NGX_HTTP_NOT_MODIFIED => {
            HttpStatus::NotModified }
         ffi::NGX_HTTP_TEMPORARY_REDIRECT => {
            HttpStatus::TemporaryRedirect }
         ffi::NGX_HTTP_BAD_REQUEST => {
            HttpStatus::BadRequest }
         ffi::NGX_HTTP_UNAUTHORIZED => {
            HttpStatus::Unauthorized }
         ffi::NGX_HTTP_FORBIDDEN => {
            HttpStatus::Forbidden }
         ffi::NGX_HTTP_NOT_FOUND => {
            HttpStatus::NotFound }
         ffi::NGX_HTTP_NOT_ALLOWED => {
            HttpStatus::NotAllowed }
         ffi::NGX_HTTP_REQUEST_TIME_OUT => {
            HttpStatus::RequestTimeOut }
         ffi::NGX_HTTP_CONFLICT => {
            HttpStatus::Conflict }
         ffi::NGX_HTTP_LENGTH_REQUIRED => {
            HttpStatus::LengthRequired }
         ffi::NGX_HTTP_PRECONDITION_FAILED => {
            HttpStatus::PreconditionFailed }
         ffi::NGX_HTTP_REQUEST_URI_TOO_LARGE => {
            HttpStatus::RequestUriTooLarge }
         ffi::NGX_HTTP_REQUEST_ENTITY_TOO_LARGE => {
            HttpStatus::RequestEntityTooLarge }
         ffi::NGX_HTTP_UNSUPPORTED_MEDIA_TYPE => {
            HttpStatus::UnsupportedMediaType }
         ffi::NGX_HTTP_RANGE_NOT_SATISFIABLE => {
            HttpStatus::RangeNotSatisfiable }
         ffi::NGX_HTTP_CLOSE => {
            HttpStatus::Close }
         ffi::NGX_HTTP_REQUEST_HEADER_TOO_LARGE => {
            HttpStatus::RequestHeaderTooLarge }
         ffi::NGX_HTTPS_CERT_ERROR => {
            HttpStatus::CertError }
         ffi::NGX_HTTPS_NO_CERT => {
            HttpStatus::NoCert }
         ffi::NGX_HTTP_TO_HTTPS => {
            HttpStatus::ToHttps }
         ffi::NGX_HTTP_CLIENT_CLOSED_REQUEST => {
            HttpStatus::ClientClosedRequest }
         ffi::NGX_HTTP_INTERNAL_SERVER_ERROR => {
            HttpStatus::InternalServerError }
         ffi::NGX_HTTP_NOT_IMPLEMENTED => {
            HttpStatus::NotImplement }
         ffi::NGX_HTTP_BAD_GATEWAY => {
            HttpStatus::BadGateway }
         ffi::NGX_HTTP_SERVICE_UNAVAILABLE => {
            HttpStatus::ServiceUnavailable }
         ffi::NGX_HTTP_GATEWAY_TIME_OUT => {
            HttpStatus::GatewayTimeOut }
         ffi::NGX_HTTP_INSUFFICIENT_STORAGE => {
            HttpStatus::InsufficientStorage }
         _ => {
            HttpStatus::InternalServerError }
      }
   }

   pub fn rc(&self) -> ffi::ngx_int_t {
      match *self {
         HttpStatus::Continue => {
            ffi::NGX_HTTP_CONTINUE }
         HttpStatus::SwitchingProtocols => {
            ffi::NGX_HTTP_SWITCHING_PROTOCOLS }
         HttpStatus::Processing => {
            ffi::NGX_HTTP_PROCESSING }
         HttpStatus::Ok => {
            ffi::NGX_HTTP_OK }
         HttpStatus::Created => {
            ffi::NGX_HTTP_CREATED }
         HttpStatus::Accepted => {
            ffi::NGX_HTTP_ACCEPTED }
         HttpStatus::NoContent => {
            ffi::NGX_HTTP_NO_CONTENT }
         HttpStatus::PartialContent => {
            ffi::NGX_HTTP_PARTIAL_CONTENT }
         HttpStatus::SpecialResponse => {
            ffi::NGX_HTTP_SPECIAL_RESPONSE }
         HttpStatus::MovedPermanently => {
            ffi::NGX_HTTP_MOVED_PERMANENTLY }
         HttpStatus::MovedTemporarily => {
            ffi::NGX_HTTP_MOVED_TEMPORARILY }
         HttpStatus::SeeOther => {
            ffi::NGX_HTTP_SEE_OTHER }
         HttpStatus::NotModified => {
            ffi::NGX_HTTP_NOT_MODIFIED }
         HttpStatus::TemporaryRedirect => {
            ffi::NGX_HTTP_TEMPORARY_REDIRECT }
         HttpStatus::BadRequest => {
            ffi::NGX_HTTP_BAD_REQUEST }
         HttpStatus::Unauthorized => {
            ffi::NGX_HTTP_UNAUTHORIZED }
         HttpStatus::Forbidden => {
            ffi::NGX_HTTP_FORBIDDEN }
         HttpStatus::NotFound => {
            ffi::NGX_HTTP_NOT_FOUND }
         HttpStatus::NotAllowed => {
            ffi::NGX_HTTP_NOT_ALLOWED }
         HttpStatus::RequestTimeOut => {
            ffi::NGX_HTTP_REQUEST_TIME_OUT }
         HttpStatus::InternalServerError => {
            ffi::NGX_HTTP_INTERNAL_SERVER_ERROR }
         HttpStatus::Conflict => {
            ffi::NGX_HTTP_CONFLICT }
         HttpStatus::LengthRequired => {
            ffi::NGX_HTTP_LENGTH_REQUIRED }
         HttpStatus::PreconditionFailed => {
            ffi::NGX_HTTP_PRECONDITION_FAILED }
         HttpStatus::RequestUriTooLarge => {
            ffi::NGX_HTTP_REQUEST_URI_TOO_LARGE }
         HttpStatus::RequestEntityTooLarge => {
            ffi::NGX_HTTP_REQUEST_ENTITY_TOO_LARGE }
         HttpStatus::UnsupportedMediaType => {
            ffi::NGX_HTTP_UNSUPPORTED_MEDIA_TYPE }
         HttpStatus::RangeNotSatisfiable => {
            ffi::NGX_HTTP_RANGE_NOT_SATISFIABLE }
         HttpStatus::Close => {
            ffi::NGX_HTTP_CLOSE }
         HttpStatus::RequestHeaderTooLarge => {
            ffi::NGX_HTTP_REQUEST_HEADER_TOO_LARGE }
         HttpStatus::CertError => {
            ffi::NGX_HTTPS_CERT_ERROR }
         HttpStatus::NoCert => {
            ffi::NGX_HTTPS_NO_CERT }
         HttpStatus::ToHttps => {
            ffi::NGX_HTTP_TO_HTTPS }
         HttpStatus::ClientClosedRequest => {
            ffi::NGX_HTTP_CLIENT_CLOSED_REQUEST }
         HttpStatus::NotImplement => {
            ffi::NGX_HTTP_NOT_IMPLEMENTED }
         HttpStatus::BadGateway => {
            ffi::NGX_HTTP_BAD_GATEWAY }
         HttpStatus::ServiceUnavailable => {
            ffi::NGX_HTTP_SERVICE_UNAVAILABLE }
         HttpStatus::GatewayTimeOut => {
            ffi::NGX_HTTP_GATEWAY_TIME_OUT }
         HttpStatus::InsufficientStorage => {
            ffi::NGX_HTTP_INSUFFICIENT_STORAGE }
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
         ffi::NGX_OK => {
            Status::Ok }
         ffi::NGX_ERROR => {
            Status::Error }
         ffi::NGX_AGAIN => {
            Status::Again }
         ffi::NGX_BUSY => {
            Status::Busy }
         ffi::NGX_DONE => {
            Status::Done }
         ffi::NGX_DECLINED => {
            Status::Declined }
         ffi::NGX_ABORT => {
            Status::Abort }
         http_rc if rc > 0 => {
            Status::Http(HttpStatus::new(http_rc)) }
         _ => {
            Status::Error }
      }
   }

   pub fn rc(&self) -> ffi::ngx_int_t {
      match *self {
         Status::Ok => {
            ffi::NGX_OK }
         Status::Error => {
            ffi::NGX_ERROR }
         Status::Again => {
            ffi::NGX_AGAIN }
         Status::Busy => {
            ffi::NGX_BUSY }
         Status::Done => {
            ffi::NGX_DONE }
         Status::Declined => {
            ffi::NGX_DECLINED }
         Status::Abort => {
            ffi::NGX_ABORT }
         Status::Http(ref http_status) => {
            http_status.rc() }
      }
   }
}
