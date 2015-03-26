use std::process::Command;
use std::fs::remove_dir_all;
use std::rc::Rc;

use paths::Paths;
use reporter::{report, report_command};


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
      if self.paths.already_downloaded() {
         report("Downloading", format!("nginx downloaded already - {}", self.paths.archive));
      } else {
         self.download_with_curl();
      }

      if self.paths.already_extracted() {
         self.remove_existing_extract_path();
      }

      self.extract();
   }

   fn download_with_curl(&self) {
      let args = vec!["-s", "-L", "-O", self.paths.http_location.as_ref()];

      report_command("Downloading", "curl", &args);

      Command::new("curl").args(&args).current_dir(&self.paths.target_path).output().unwrap_or_else(|e| {
         panic!("Downloading nginx with Curl failed: {}.", e)
      });
   }

   fn extract(&self) {
      let args = vec!["xzf", self.paths.archive.as_ref()];

      report_command("Extracting", "tar", &args);

      Command::new("tar").args(&args).current_dir(&self.paths.target_path).output().unwrap_or_else(|e| {
         panic!("Extracting nginx failed: {}.", e)
      });
   }

   fn remove_existing_extract_path(&self) {
      report("Deleting", format!("previously extracted {:?}", &self.paths.extract_path));

      remove_dir_all(&self.paths.extract_path).unwrap_or_else(|e| {
         panic!("Cannot delete nginx extract path - {:?}: {}.", self.paths.extract_path, e)
      });
   }
}
