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
use std::cell::RefCell;

use version::nginx_version_string;
use paths::Paths;
use downloader::Downloader;
use builder::Builder;
use reporter::Reporter;


fn main() {
   let version = nginx_version_string();

   let reporter = Reporter::new();
   let reporter_rc = Rc::new(RefCell::new(reporter));

   let paths = Paths::new(version);
   let paths_rc = Rc::new(paths);

   let nginx_downloader = Downloader::new(paths_rc.clone(), reporter_rc.clone());
   nginx_downloader.download();

   let nginx_builder = Builder::new(paths_rc.clone(), reporter_rc.clone());
   nginx_builder.build();
}
