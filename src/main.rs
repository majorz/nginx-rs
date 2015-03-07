#![feature(core)]
#![feature(path)]
#![feature(fs)]

mod downloader;
mod builder;
mod version;

use version::nginx_version_string;

use downloader::Downloader;
use builder::Builder;


fn main() {
   let version = nginx_version_string();

   let nginx_downloader = Downloader::new(version);
   nginx_downloader.download();

   let nginx_builder = Builder::new(nginx_downloader);
   nginx_builder.build();
}
