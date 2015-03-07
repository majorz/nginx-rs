use std::process::Command;
use std::fs::remove_dir_all;
use std::rc::Rc;

use paths::Paths;


pub struct Downloader {
   paths: Rc<Paths>,
}


impl Downloader {
   pub fn new(paths: Rc<Paths>) -> Self {
      Downloader {
         paths: paths
      }
   }

   pub fn download(&self) {
      println!("Downloading Nginx v{}...", self.paths.version);

      if self.paths.already_downloaded() {
         println!("Nginx archive already downloaded.", );
      } else {
         self.download_with_curl();
      }

      if self.paths.already_extracted() {
         self.remove_existing_extract_path();
      }

      self.extract();
   }

   fn download_with_curl(&self) {
      let args = ["-s", "-L", "-O", self.paths.http_location.as_slice()];

      Command::new("curl").args(&args).current_dir(&self.paths.download_path).output().unwrap_or_else(|e| {
         panic!("Downloading Nginx with Curl failed: {}.", e)
      });

      println!("Nginx downloaded.");
   }

   fn extract(&self) {
      println!("Extracting Nginx...");

      let args = ["xzf", self.paths.archive.as_slice()];

      Command::new("tar").args(&args).current_dir(&self.paths.download_path).output().unwrap_or_else(|e| {
         panic!("Extracting Nginx failed: {}.", e)
      });
   }

   fn remove_existing_extract_path(&self) {
      println!("Removing previously extracted Nginx archive.");

      remove_dir_all(&self.paths.extract_path).unwrap_or_else(|e| {
         panic!("Cannot delete Nginx extract path - {:?}: {}.", self.paths.extract_path, e)
      });
   }
}
