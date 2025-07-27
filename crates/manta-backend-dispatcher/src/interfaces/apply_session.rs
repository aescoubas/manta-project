use std::future::Future;

use crate::error::Error;

pub trait ApplySessionTrait {
  fn apply_session(
    &self,
    _gitea_token: &str,
    _gitea_base_url: &str,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    // _k8s_api_url: &str,
    _cfs_conf_sess_name: Option<&String>,
    _playbook_yaml_file_name_opt: Option<&String>,
    _hsm_group: Option<&String>,
    _repo_name_vec: Vec<String>,
    _repo_last_commit_id_vec: Vec<String>,
    _ansible_limit: Option<String>,
    _ansible_verbosity: Option<String>,
    _ansible_passthrough: Option<String>,
    // _watch_logs: bool,
  ) -> impl Future<Output = Result<(String, String), Error>> + Send {
    async {
      Err(Error::Message(
        "Apply session command not implemented for this backend".to_string(),
      ))
    }
  }
}
