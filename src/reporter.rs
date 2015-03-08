use std::fmt;
use std::old_io::IoResult;
use term::{stdout, color, Attr};
use std::str::Str;


pub fn report_command<T, U, S>(status: T, command: U, args: &Vec<S>)
   where T: fmt::Display, U: fmt::Display, S: Str
{
   let arg_string =  args.connect(" ");
   report(status, format!("{} {}", command, arg_string));
}


pub fn report<T, U>(status: T, message: U)
   where T: fmt::Display, U: fmt::Display
{
   report_impl(status, message).unwrap();
}


fn report_impl<T, U>(status: T, message: U) -> IoResult<()>
   where T: fmt::Display, U: fmt::Display
{
   let mut term = stdout().unwrap();

   try!(term.reset());

   try!(term.fg(color::CYAN));
   if term.supports_attr(Attr::Bold) {
      try!(term.attr(Attr::Bold));
   }
   try!(term.write_str(&format!("{:>12}", status)));

   try!(term.reset());

   try!(term.write_line(&format!(" {}", message)));
   try!(term.flush());

   Ok(())
}
