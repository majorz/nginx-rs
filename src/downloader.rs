use std::process::Command;
use std::fs::remove_dir_all;
use std::rc::Rc;
use std::cell::RefCell;

use paths::Paths;
use reporter::Reporter;


pub struct Downloader {
   paths: Rc<Paths>,

   reporter: Rc<RefCell<Reporter>>,
}


impl Downloader {
   pub fn new(paths: Rc<Paths>, reporter: Rc<RefCell<Reporter>>) -> Self {
      Downloader {
         paths: paths,
         reporter: reporter,
      }
   }

   pub fn download(&self) {
      self.reporter.borrow_mut().report("Downloading", format!("nginx v{}", self.paths.version)).unwrap();

      if self.paths.already_downloaded() {
         self.reporter.borrow_mut().report("Downloaded", "already").unwrap();
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
         panic!("Downloading nginx with Curl failed: {}.", e)
      });
   }

   fn extract(&self) {
      self.reporter.borrow_mut().report("Extracting", "nginx").unwrap();

      let args = ["xzf", self.paths.archive.as_slice()];

      Command::new("tar").args(&args).current_dir(&self.paths.download_path).output().unwrap_or_else(|e| {
         panic!("Extracting nginx failed: {}.", e)
      });
   }

   fn remove_existing_extract_path(&self) {
      self.reporter.borrow_mut().report("Removing", "previously extracted archive").unwrap();

      remove_dir_all(&self.paths.extract_path).unwrap_or_else(|e| {
         panic!("Cannot delete nginx extract path - {:?}: {}.", self.paths.extract_path, e)
      });
   }
}
