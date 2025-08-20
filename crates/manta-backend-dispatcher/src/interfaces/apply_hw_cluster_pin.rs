use std::future::Future;

use crate::error::Error;

pub trait ApplyHwClusterPin {
  fn apply_hw_cluster_pin(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _target_hsm_group_name: &str,
    _parent_hsm_group_name: &str,
    _pattern: &str,
    _nodryrun: bool,
    _create_target_hsm_group: bool,
    _delete_empty_parent_hsm_group: bool,
  ) -> impl Future<Output = Result<(), Error>> + Send {
    async {
      Err(Error::Message(
        "Apply hardware cluster pin command not implemented for this backend"
          .to_string(),
      ))
    }
  }
}
