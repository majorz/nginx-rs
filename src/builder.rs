use std::process::Command;
use std::rc::Rc;

use paths::Paths;


pub struct Builder {
   paths: Rc<Paths>,
}


impl Builder {
   pub fn new(paths: Rc<Paths>) -> Self {
      Builder {
         paths: paths
      }
   }

   pub fn build(&self) {
      println!("Building Nginx... {:?} / v{}", self.paths.configure_path, self.paths.version);

      self.configure();
   }

   fn configure(&self) {
      println!("Configuring Nginx build...");

      Command::new(&self.paths.configure_path).current_dir(&self.paths.extract_path).status().unwrap_or_else(|e| {
         panic!("Configuring Nginx build failed: {}.", e)
      });
   }
}

