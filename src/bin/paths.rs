use std::env;
use std::path::PathBuf;
use std::fs::PathExt;
use reporter::report_path;


pub struct Paths {
   pub version: String,

   pub archive: String,

   pub http_location: String,

   pub target_path: PathBuf,

   pub archive_path: PathBuf,

   pub extract_path: PathBuf,

   pub configure_path: PathBuf,

   pub ngxconf_prefix_path: PathBuf,

   pub ngxconf_path: PathBuf,

   pub ngxconf_source_path: PathBuf,

   pub module_dir_path: PathBuf,
}


impl Paths {

   pub fn new(version: String) -> Self {
      let extract_dir = format!("nginx-{}", version);
      let archive = format!("{}.tar.gz", extract_dir);

      let http_location = format!("http://nginx.org/download/{}", archive);

      let exe_path = Paths::get_exe_path();

      let target_slice = exe_path.parent().unwrap();
      let root_slice = target_slice.parent().unwrap();

      let root_path = PathBuf::new(root_slice);
      report_path("root", &root_path);

      let target_path = PathBuf::new(target_slice);
      report_path("target", &target_path);

      let archive_path = target_path.join(&archive);
      report_path("archive", &archive_path);

      let extract_path = target_path.join(&extract_dir);
      report_path("extract", &extract_path);

      let configure_path = extract_path.join("configure");
      report_path("configure", &configure_path);

      let ngxconf_prefix_path = target_path.join("conf");
      report_path("ngxconf_prefix", &ngxconf_prefix_path);

      let ngxconf_path = ngxconf_prefix_path.join("nginx.conf");
      report_path("ngxconf", &ngxconf_path);

      let ngxconf_source_path = root_path.join("conf").join("nginx.conf");
      report_path("ngxconf_source", &ngxconf_source_path);

      let module_dir_path = root_path.join("module");
      report_path("module_dir", &module_dir_path);

      Paths {
         version: version,
         archive: archive,
         http_location: http_location,
         target_path: target_path,
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
}
