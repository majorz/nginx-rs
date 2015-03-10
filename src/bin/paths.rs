use std::env;
use std::path::PathBuf;
use std::fs::PathExt;

use reporter::report_path;
use version::nginx_version_string;


pub struct Paths {
   pub archive: String,

   pub http_location: String,

   pub target_path: PathBuf,

   pub dist_path: PathBuf,

   pub archive_path: PathBuf,

   pub extract_path: PathBuf,

   pub configure_path: PathBuf,

   pub ngxconf_prefix_path: PathBuf,

   pub ngxconf_path: PathBuf,

   pub ngxconf_source_path: PathBuf,

   pub module_dir_path: PathBuf,
}


impl Paths {

   pub fn new() -> Self {
      let version = nginx_version_string();

      let extract_dir = format!("nginx-{}", version);
      let archive = format!("{}.tar.gz", extract_dir);

      let http_location = format!("http://nginx.org/download/{}", archive);

      let exe_path = Paths::get_exe_path();

      let target_slice = exe_path.parent().unwrap();

      let root_path = Paths::get_root_path();
      report_path("root", &root_path);

      let target_path = PathBuf::new(target_slice);
      report_path("target", &target_path);

      let dist_path = target_path.join("dist");
      report_path("dist", &dist_path);

      let archive_path = target_path.join(&archive);
      report_path("archive", &archive_path);

      let extract_path = target_path.join(&extract_dir);
      report_path("extract", &extract_path);

      let configure_path = extract_path.join("configure");
      report_path("configure", &configure_path);

      let ngxconf_prefix_path = dist_path.join("conf");
      report_path("ngxconf_prefix", &ngxconf_prefix_path);

      let ngxconf_path = ngxconf_prefix_path.join("nginx.conf");
      report_path("ngxconf", &ngxconf_path);

      let ngxconf_source_path = root_path.join("conf").join("nginx.conf");
      report_path("ngxconf_source", &ngxconf_source_path);

      let module_dir_path = root_path.join("module");
      report_path("module_dir", &module_dir_path);

      Paths {
         archive: archive,
         http_location: http_location,
         target_path: target_path,
         dist_path: dist_path,
         archive_path: archive_path,
         extract_path: extract_path,
         configure_path: configure_path,
         ngxconf_prefix_path: ngxconf_prefix_path,
         ngxconf_path: ngxconf_path,
         ngxconf_source_path: ngxconf_source_path,
         module_dir_path: module_dir_path,
      }
   }

   pub fn already_downloaded(&self) -> bool {
      self.archive_path.exists() && self.archive_path.is_file()
   }

   pub fn already_extracted(&self) -> bool {
      self.extract_path.exists() && self.extract_path.is_dir()
   }

   fn get_exe_path() -> PathBuf {
      let exe_path = env::current_exe().unwrap_or_else(|_| {
         panic!("Cannot retrieve current executable location.")
      });

      exe_path
   }

   fn get_root_path() -> PathBuf {
      let root_path = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| {
         panic!("Cannot retrieve root dir – CARGO_MANIFEST_DIR.")
      });

      PathBuf::new(root_path.as_slice())
   }
}
