#![feature(core)]
#![feature(path_ext)]
#![feature(convert)]

#![allow(deprecated)]

extern crate term;
extern crate itertools;

mod version;
mod paths;
mod downloader;
mod builder;
mod reporter;

use std::rc::Rc;

use paths::Paths;
use downloader::Downloader;
use builder::Builder;


fn main() {
   dump_env();

   let paths = Paths::new();
   let paths_rc = Rc::new(paths);

   let nginx_downloader = Downloader::new(paths_rc.clone());
   nginx_downloader.download();

   let nginx_builder = Builder::new(paths_rc.clone());
   nginx_builder.build();
}


fn dump_env() {
   use std::env;
   use reporter::report;

   report("Dumping", "Environment");

   for (key, value) in env::vars() {
       println!("{:>30}: {}", key, value);
   }
}
