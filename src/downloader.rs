use std::env;
use std::process::Command;

use version::nginx_version_string;


pub struct Downloader {
   version: String,

   http_location: String,

   download_dir: String,
}


impl Downloader {

   pub fn new() -> Self {
      let version = nginx_version_string();

      let filename = format!("nginx-{}.tar.gz", version);

      let http_location = format!("http://nginx.org/download/{}", filename);

      let download_dir = Downloader::archive_download_dir();


      Downloader {
         version: version,

         http_location: http_location,

         download_dir: download_dir,
      }

   }

   pub fn archive_download_dir() -> String {
      let exe_path = env::current_exe().unwrap_or_else(|_| {
         panic!("Cannot retrieve current executable location")
      });

      match exe_path.parent() {
         Some(download_dir) => {
            download_dir.to_str().unwrap().to_string()
         },
         None => {
            panic!("Cannot access download directory")
         }
      }
   }

   pub fn download(&self) {
      println!("Downloading nginx v{}...", self.version);

      let args = ["-s", "-L", "-O", self.http_location.as_slice()];

      Command::new("curl").args(&args).current_dir(self.download_dir.as_slice()).output().unwrap_or_else(|e| {
         panic!("Downloading nginx with curl failed: {}", e)
      });

      println!("Done.");
   }

}



