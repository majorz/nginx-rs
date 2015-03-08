use std::env;
use std::path::PathBuf;
use std::fs::PathExt;


pub struct Paths {
   pub version: String,

   pub archive: String,

   pub http_location: String,

   pub target_path: PathBuf,

   pub archive_path: PathBuf,

   pub extract_path: PathBuf,

   pub configure_path: PathBuf,

   pub nginx_conf_prefix_path: PathBuf,

   pub nginx_conf_path: PathBuf,

   pub nginx_conf_source_path: PathBuf,
}


impl Paths {

   pub fn new(version: String) -> Self {
      let extract_dir = format!("nginx-{}", version);
      let archive = format!("{}.tar.gz", extract_dir);

      let http_location = format!("http://nginx.org/download/{}", archive);

      let target_path = Paths::target_path();

      let archive_path = target_path.join(&archive);

      let extract_path = target_path.join(&extract_dir);

      let configure_path = extract_path.join("configure");

      let nginx_conf_prefix_path = target_path.join("conf");

      let nginx_conf_path = nginx_conf_prefix_path.join("nginx.conf");

      let nginx_conf_source_path = target_path.parent().unwrap().join("conf").join("nginx.conf");

      Paths {
         version: version,
         archive: archive,
         http_location: http_location,
         target_path: target_path,
         archive_path: archive_path,
         extract_path: extract_path,
         configure_path: configure_path,
         nginx_conf_prefix_path: nginx_conf_prefix_path,
         nginx_conf_path: nginx_conf_path,
         nginx_conf_source_path: nginx_conf_source_path,
      }
   }

   pub fn already_downloaded(&self) -> bool {
      self.archive_path.exists() && self.archive_path.is_file()
   }

   pub fn already_extracted(&self) -> bool {
      self.extract_path.exists() && self.extract_path.is_dir()
   }

   fn target_path() -> PathBuf {
      let exe_path = env::current_exe().unwrap_or_else(|_| {
         panic!("Cannot retrieve current executable location.")
      });

      exe_path.parent().unwrap().to_path_buf()
   }
}

