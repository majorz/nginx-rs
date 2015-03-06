#![feature(core)]
#![feature(path)]

mod downloader;
mod version;

use downloader::Downloader;


fn main() {
   let nginx_downloader = Downloader::new();

   nginx_downloader.download();
}
