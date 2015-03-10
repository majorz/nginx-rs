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
      self.make();
      self.make_install();
   }

   fn configure(&self) {
      self.setup_conf_path();

      let prefix = format!("--prefix={}", self.paths.target_path.to_str().unwrap());
      let conf_path = format!("--conf-path={}", self.paths.ngxconf_path.to_str().unwrap());
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
      report("Creating", format!("{:?}", &self.paths.ngxconf_prefix_path));

      create_dir_all(&self.paths.ngxconf_prefix_path).unwrap_or_else(|e| {
         panic!("Cannot create nginx conf prefix path - {:?}: {}.", self.paths.ngxconf_prefix_path, e)
      });

      report("Copying", format!("nginx.conf {:?}", &self.paths.ngxconf_path));

      copy(&self.paths.ngxconf_source_path, &self.paths.ngxconf_path).unwrap_or_else(|e| {
         panic!("Cannot copy nginx.conf {:?} -> {:?}: {}.", self.paths.ngxconf_source_path, self.paths.ngxconf_path, e)
      });
   }

   fn make(&self) {
      report("Building", "make");

      Command::new("make").current_dir(&self.paths.extract_path).status().unwrap_or_else(|e| {
         panic!("Make failed: {}.", e)
      });
   }

   fn make_install(&self) {
      let args = vec!["install"];

      report_command("Installing", "make", &args);

      Command::new("make").args(&args).current_dir(&self.paths.extract_path).status().unwrap_or_else(|e| {
         panic!("Make install failed: {}.", e)
      });
   }
}
