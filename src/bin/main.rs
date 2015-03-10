#![feature(core)]
#![feature(old_io)]
#![feature(path)]
#![feature(path_ext)]

extern crate term;

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
   let paths = Paths::new();
   let paths_rc = Rc::new(paths);

   let nginx_downloader = Downloader::new(paths_rc.clone());
   nginx_downloader.download();

   let nginx_builder = Builder::new(paths_rc.clone());
   nginx_builder.build();
}
