use std::env;
use std::process::Command;
use std::path::PathBuf;
use std::fs::PathExt;

use version::nginx_version_string;


pub struct Downloader {
   version: String,

   filename: String,

   http_location: String,

   download_dir: PathBuf,
}


impl Downloader {

   pub fn new() -> Self {
      let version = nginx_version_string();

      let filename = format!("nginx-{}.tar.gz", version);

      let http_location = format!("http://nginx.org/download/{}", filename);

      let download_dir = Downloader::archive_download_dir();

      Downloader {
         version: version,

         filename: filename,

         http_location: http_location,

         download_dir: download_dir,
      }

   }

   fn archive_download_dir() -> PathBuf {
      let exe_path = env::current_exe().unwrap_or_else(|_| {
         panic!("Cannot retrieve current executable location.")
      });

      match exe_path.parent() {
         Some(download_dir) => {
            download_dir.to_path_buf()
         },
         None => {
            panic!("Cannot access download directory.")
         }
      }
   }

   pub fn download(&self) {
      println!("Downloading Nginx v{}...", self.version);

      if self.already_downloaded() {
         println!("Nginx archive already downloaded.", );
      } else {
         self.download_with_curl();
      }
   }

   fn download_with_curl(&self) {
      let args = ["-s", "-L", "-O", self.http_location.as_slice()];

      Command::new("curl").args(&args).current_dir(&self.download_dir).output().unwrap_or_else(|e| {
         panic!("Downloading Nginx with Curl failed: {}.", e)
      });

      println!("Nginx downloaded.");
   }

   fn already_downloaded(&self) -> bool {
      let archive_path = self.download_dir.join(&self.filename);

      archive_path.exists() && archive_path.is_file()
   }

}


