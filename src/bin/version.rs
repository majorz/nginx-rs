use std::env;

const DEFAULT_NGINX_VERSION: (u16, u16, u16) = (1, 7, 11);


pub fn nginx_version_string() -> String {
   let key = "NGINX_VERSION";

   match env::var(key) {
      Ok(val) => {
         val
      },
      Err(_) => {
         let (major, minor, patch) = DEFAULT_NGINX_VERSION;

         format!("{}.{}.{}", major, minor, patch)
      },
   }
}
