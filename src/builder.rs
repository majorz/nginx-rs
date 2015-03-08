use std::process::Command;
use std::rc::Rc;
use std::fs::{create_dir_all, copy};

use paths::Paths;
use reporter::report;


pub struct Builder {
   paths: Rc<Paths>,
}


impl Builder {
   pub fn new(paths: Rc<Paths>) -> Self {
      Builder {
         paths: paths,
      }
   }

   pub fn build(&self) {
      self.configure();
   }

   fn configure(&self) {
      report("Configuring", "build");

      self.setup_conf_path();

      let prefix = format!("--prefix={}", self.paths.target_path.to_str().unwrap());
      let conf_path = format!("--conf-path={}", self.paths.nginx_conf_path.to_str().unwrap());

      let args = [
         "--with-http_ssl_module",
         prefix.as_slice(),
         conf_path.as_slice(),
      ];

      Command::new(&self.paths.configure_path).args(&args).current_dir(&self.paths.extract_path).status().unwrap_or_else(|e| {
         panic!("Configuring Nginx build failed: {}.", e)
      });
   }

   fn setup_conf_path(&self) {
      report("Setup", "nginx conf prefix path");

      create_dir_all(&self.paths.nginx_conf_prefix_path).unwrap_or_else(|e| {
         panic!("Cannot create nginx conf prefix path - {:?}: {}.", self.paths.nginx_conf_prefix_path, e)
      });

      copy(&self.paths.nginx_conf_source_path, &self.paths.nginx_conf_path).unwrap_or_else(|e| {
         panic!("Cannot copy nginx.conf {:?} -> {:?}: {}.", self.paths.nginx_conf_source_path, self.paths.nginx_conf_path, e)
      });
   }
}

