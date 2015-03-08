use std::fmt;
use term::{StdTerminal, stdout, color, Attr};
use std::old_io::IoResult;

pub struct Reporter {
   term: Box<StdTerminal>,
}

impl Reporter {
   pub fn new() -> Self {
      Reporter {
         term: stdout().unwrap(),
      }
   }

   pub fn report<T, U>(&mut self, status: T, message: U) -> IoResult<()>
      where T: fmt::Display, U: fmt::Display
   {
      try!(self.term.reset());

      try!(self.term.fg(color::CYAN));
      if self.term.supports_attr(Attr::Bold) {
         try!(self.term.attr(Attr::Bold));
      }
      try!(self.term.write_str(&format!("{:>12}", status)));

      try!(self.term.reset());

      try!(self.term.write_line(&format!(" {}", message)));
      try!(self.term.flush());

      Ok(())
   }
}
