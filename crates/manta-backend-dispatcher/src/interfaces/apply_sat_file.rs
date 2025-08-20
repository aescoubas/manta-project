use std::future::Future;

use crate::error::Error;

pub trait SatTrait {
  fn apply_sat_file(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _vault_base_url: &str,
    _vault_secret_path: &str,
    _k8s_api_url: &str,
    _shasta_k8s_secrets: serde_json::Value,
    _sat_template_file_yaml: serde_yaml::Value,
    _hsm_group_available_vec: &Vec<String>,
    _ansible_verbosity_opt: Option<u8>,
    _ansible_passthrough_opt: Option<&String>,
    _gitea_base_url: &str,
    _gitea_token: &str,
    _do_not_reboot: bool,
    _watch_logs: bool,
    _debug_on_failure: bool,
    _overwrite: bool,
    _dry_run: bool,
  ) -> impl Future<Output = Result<(), Error>> + Send {
    async {
      Err(Error::Message(
        "Apply SAT file command not implemented for this backend".to_string(),
      ))
    }
  }
}
