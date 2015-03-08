use std::process::Command;
use std::rc::Rc;

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

      Command::new(&self.paths.configure_path).current_dir(&self.paths.extract_path).status().unwrap_or_else(|e| {
         panic!("Configuring Nginx build failed: {}.", e)
      });
   }
}

