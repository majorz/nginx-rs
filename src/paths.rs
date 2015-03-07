use std::env;
use std::path::PathBuf;
use std::fs::PathExt;


pub struct Paths {
   pub version: String,

   pub archive: String,

   pub http_location: String,

   pub download_path: PathBuf,

   pub archive_path: PathBuf,

   pub extract_path: PathBuf,

   pub configure_path: PathBuf,
}


impl Paths {

   pub fn new(version: String) -> Self {
      let extract_dir = format!("nginx-{}", version);
      let archive = format!("{}.tar.gz", extract_dir);

      let http_location = format!("http://nginx.org/download/{}", archive);

      let download_path = Paths::archive_download_path();

      let archive_path = download_path.join(&archive);

      let extract_path = download_path.join(&extract_dir);

      let configure_path = extract_path.join("configure");

      Paths {
         version: version,
         archive: archive,
         http_location: http_location,
         download_path: download_path,
         archive_path: archive_path,
         extract_path: extract_path,
         configure_path: configure_path,
      }
   }

   pub fn already_downloaded(&self) -> bool {
      self.archive_path.exists() && self.archive_path.is_file()
   }

   pub fn already_extracted(&self) -> bool {
      self.extract_path.exists() && self.extract_path.is_dir()
   }

   fn archive_download_path() -> PathBuf {
      let exe_path = env::current_exe().unwrap_or_else(|_| {
         panic!("Cannot retrieve current executable location.")
      });

      exe_path.parent().unwrap().to_path_buf()
   }
}

