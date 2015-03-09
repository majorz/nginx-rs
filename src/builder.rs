use std::process::Command;
use std::rc::Rc;
use std::fs::{create_dir_all, copy};

use paths::Paths;
use reporter::{report, report_command};


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
      self.setup_conf_path();

      let prefix = format!("--prefix={}", self.paths.target_path.to_str().unwrap());
      let conf_path = format!("--conf-path={}", self.paths.nginx_conf_path.to_str().unwrap());
      let add_module = format!("--add-module={}", self.paths.module_dir_path.to_str().unwrap());

      let args = vec![
         "--with-http_ssl_module",
         prefix.as_slice(),
         conf_path.as_slice(),
         add_module.as_slice(),
      ];

      report_command("Configuring", "configure", &args);

      Command::new(&self.paths.configure_path).args(&args).current_dir(&self.paths.extract_path).status().unwrap_or_else(|e| {
         panic!("Configuring Nginx build failed: {}.", e)
      });
   }

   fn setup_conf_path(&self) {
      report("Creating", format!("{:?}", &self.paths.nginx_conf_prefix_path));

      create_dir_all(&self.paths.nginx_conf_prefix_path).unwrap_or_else(|e| {
         panic!("Cannot create nginx conf prefix path - {:?}: {}.", self.paths.nginx_conf_prefix_path, e)
      });

      report("Copying", format!("nginx.conf {:?}", &self.paths.nginx_conf_path));

      copy(&self.paths.nginx_conf_source_path, &self.paths.nginx_conf_path).unwrap_or_else(|e| {
         panic!("Cannot copy nginx.conf {:?} -> {:?}: {}.", self.paths.nginx_conf_source_path, self.paths.nginx_conf_path, e)
      });
   }
}
