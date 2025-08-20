use std::future::Future;

use crate::error::Error;

pub trait MigrateRestoreTrait {
  fn migrate_restore(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _bos_file: Option<&String>,
    _cfs_file: Option<&String>,
    _hsm_file: Option<&String>,
    _ims_file: Option<&String>,
    _image_dir: Option<&String>,
  ) -> impl Future<Output = Result<(), Error>> + Send {
    async {
      Err(Error::Message(
        "Migrate/restore command not implemented for this backend".to_string(),
      ))
    }
  }
}
