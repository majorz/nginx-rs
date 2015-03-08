use std::process::Command;
use std::fs::remove_dir_all;
use std::rc::Rc;

use paths::Paths;
use reporter::report;


pub struct Downloader {
   paths: Rc<Paths>,
}


impl Downloader {
   pub fn new(paths: Rc<Paths>) -> Self {
      Downloader {
         paths: paths,
      }
   }

   pub fn download(&self) {
      report("Downloading", format!("nginx v{}", self.paths.version));

      if self.paths.already_downloaded() {
         report("Downloaded", "already");
      } else {
         self.download_with_curl();
      }

      if self.paths.already_extracted() {
         self.remove_existing_extract_path();
      }

      self.extract();
   }

   fn download_with_curl(&self) {
      report("Curl", self.paths.http_location.as_slice());

      let args = ["-s", "-L", "-O", self.paths.http_location.as_slice()];

      Command::new("curl").args(&args).current_dir(&self.paths.target_path).output().unwrap_or_else(|e| {
         panic!("Downloading nginx with Curl failed: {}.", e)
      });
   }

   fn extract(&self) {
      report("Extracting", "nginx");

      let args = ["xzf", self.paths.archive.as_slice()];

      Command::new("tar").args(&args).current_dir(&self.paths.target_path).output().unwrap_or_else(|e| {
         panic!("Extracting nginx failed: {}.", e)
      });
   }

   fn remove_existing_extract_path(&self) {
      report("Removing", "previously extracted archive");

      remove_dir_all(&self.paths.extract_path).unwrap_or_else(|e| {
         panic!("Cannot delete nginx extract path - {:?}: {}.", self.paths.extract_path, e)
      });
   }
}
