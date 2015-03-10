use std::fmt;
use std::old_io::IoResult;
use term::{stdout, color, Attr};
use std::str::Str;
use std::path::PathBuf;


pub fn report_command<T, U, S>(status: T, command: U, args: &Vec<S>)
   where T: fmt::Display, U: fmt::Display, S: Str
{
   let arg_string =  args.connect(" ");
   report(status, format!("{} {}", command, arg_string));
}


pub fn report<T, U>(status: T, message: U)
   where T: fmt::Display, U: fmt::Display
{
   report_impl(status, message, color::CYAN).unwrap();
}


pub fn report_path<T>(key: T, path: &PathBuf)
   where T: fmt::Display
{
   let path_string = path.to_str().unwrap();
   let message = format!("{:>14} = \"{}\"", key, path_string);
   report_impl("Path", message, color::CYAN).unwrap();
}


fn report_impl<T, U>(status: T, message: U, status_color: color::Color) -> IoResult<()>
   where T: fmt::Display, U: fmt::Display
{
   let mut term = stdout().unwrap();

   try!(term.reset());

   try!(term.fg(status_color));
   if term.supports_attr(Attr::Bold) {
      try!(term.attr(Attr::Bold));
   }
   try!(term.write_str(&format!("{:>12}", status)));

   try!(term.reset());

   try!(term.write_line(&format!(" {}", message)));
   try!(term.flush());

   Ok(())
}
