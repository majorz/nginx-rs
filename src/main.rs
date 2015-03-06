#![feature(core)]
#![feature(path)]


use std::process::Command;
use std::env;


const NGINX_VERSION: &'static str = "1.7.10";


fn download_nginx() {
   println!("Downloading nginx v{}...", NGINX_VERSION);

   let http_location = format!("http://nginx.org/download/nginx-{}.tar.gz", NGINX_VERSION);

   let exe_path = env::current_exe().unwrap_or_else(|_| {
      panic!("Cannot retrieve current executable location")
   });

   let download_dir = exe_path.parent().unwrap();

   let args = ["-s", "-L", "-O", http_location.as_slice()];

   Command::new("curl").args(&args).current_dir(download_dir).output().unwrap_or_else(|e| {
      panic!("Downloading nginx: {}", e)
   });
}

fn main() {
   download_nginx();

   println!("Done.");
}
