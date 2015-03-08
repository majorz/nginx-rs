use std::process::Command;
use std::rc::Rc;
use std::cell::RefCell;

use paths::Paths;
use reporter::Reporter;


pub struct Builder {
   paths: Rc<Paths>,

   reporter: Rc<RefCell<Reporter>>,
}


impl Builder {
   pub fn new(paths: Rc<Paths>, reporter: Rc<RefCell<Reporter>>) -> Self {
      Builder {
         paths: paths,
         reporter: reporter,
      }
   }

   pub fn build(&self) {
      self.configure();
   }

   fn configure(&self) {
      self.reporter.borrow_mut().report("Configuring", "build").unwrap();

      Command::new(&self.paths.configure_path).current_dir(&self.paths.extract_path).status().unwrap_or_else(|e| {
         panic!("Configuring Nginx build failed: {}.", e)
      });
   }
}

