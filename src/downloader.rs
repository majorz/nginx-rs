use std::env;
use std::process::Command;
use std::path::PathBuf;
use std::fs::{PathExt, remove_dir_all};


pub struct Downloader {
   pub version: String,

   archive: String,

   http_location: String,

   download_path: PathBuf,

   archive_path: PathBuf,

   pub extract_path: PathBuf,
}


impl Downloader {

   pub fn new(version: String) -> Self {
      let extract_dir = format!("nginx-{}", version);
      let archive = format!("{}.tar.gz", extract_dir);

      let http_location = format!("http://nginx.org/download/{}", archive);

      let download_path = archive_download_path();

      let archive_path = download_path.join(&archive);

      let extract_path = download_path.join(&extract_dir);

      Downloader {
         version: version,
         archive: archive,
         http_location: http_location,
         download_path: download_path,
         archive_path: archive_path,
         extract_path: extract_path,
      }
   }

   pub fn download(&self) {
      println!("Downloading Nginx v{}...", self.version);

      if self.already_downloaded() {
         println!("Nginx archive already downloaded.", );
      } else {
         self.download_with_curl();
      }

      if self.already_extracted() {
         self.remove_existing_extract_path();
      }

      self.extract();
   }

   fn download_with_curl(&self) {
      let args = ["-s", "-L", "-O", self.http_location.as_slice()];

      Command::new("curl").args(&args).current_dir(&self.download_path).output().unwrap_or_else(|e| {
         panic!("Downloading Nginx with Curl failed: {}.", e)
      });

      println!("Nginx downloaded.");
   }

   fn extract(&self) {
      println!("Extracting Nginx...");

      let args = ["xzf", self.archive.as_slice()];

      Command::new("tar").args(&args).current_dir(&self.download_path).output().unwrap_or_else(|e| {
         panic!("Extracting Nginx failed: {}.", e)
      });
   }

   fn remove_existing_extract_path(&self) {
      println!("Removing previously extracted Nginx archive.");

      remove_dir_all(&self.extract_path).unwrap_or_else(|e| {
         panic!("Cannot delete Nginx extract path - {:?}: {}.", self.extract_path, e)
      });
   }

   fn already_downloaded(&self) -> bool {
      self.archive_path.exists() && self.archive_path.is_file()
   }

   fn already_extracted(&self) -> bool {
      self.extract_path.exists() && self.extract_path.is_dir()
   }
}


fn archive_download_path() -> PathBuf {
   let exe_path = env::current_exe().unwrap_or_else(|_| {
      panic!("Cannot retrieve current executable location.")
   });

   exe_path.parent().unwrap().to_path_buf()
}

