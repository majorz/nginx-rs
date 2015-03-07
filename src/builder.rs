use std::path::PathBuf;

use downloader::Downloader;


pub struct Builder {
   downloader: Downloader,

   configure_path: PathBuf,
}


impl Builder {
   pub fn new(downloader: Downloader) -> Self {
      let configure_path = downloader.extract_path.join("configure");

      Builder {
         downloader: downloader,
         configure_path: configure_path,
      }
   }

   pub fn build(&self) {
      println!("Building Nginx... {:?} / v{}", self.configure_path, self.downloader.version);
   }
}

