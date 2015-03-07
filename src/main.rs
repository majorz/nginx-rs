#![feature(core)]
#![feature(path)]
#![feature(fs)]

mod downloader;
mod version;

use downloader::Downloader;


fn main() {
   let nginx_downloader = Downloader::new();
   nginx_downloader.download();
}
