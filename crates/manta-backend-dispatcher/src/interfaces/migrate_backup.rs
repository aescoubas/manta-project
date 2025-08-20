use std::future::Future;

use crate::error::Error;

pub trait MigrateBackupTrait {
  fn migrate_backup(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _bos: Option<&String>,
    _destination: Option<&String>,
  ) -> impl Future<Output = Result<(), Error>> + Send {
    async {
      Err(Error::Message(
        "Migrate/backup command not implemented for this backend".to_string(),
      ))
    }
  }
}
