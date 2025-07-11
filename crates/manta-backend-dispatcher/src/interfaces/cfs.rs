use std::future::Future;

use crate::types::cfs::cfs_configuration_details::LayerDetails;
use crate::types::cfs::cfs_configuration_request::CfsConfigurationRequest;
use crate::types::cfs::cfs_configuration_response::{
  CfsConfigurationResponse, Layer,
};
use crate::types::cfs::component::Component;
use crate::types::cfs::session::CfsSessionPostRequest;
use crate::types::ims::Image;
use crate::types::{bos::session_template::BosSessionTemplate, K8sDetails};
use crate::{error::Error, types::cfs::session::CfsSessionGetResponse};

pub trait CfsTrait {
  type T: futures_io::AsyncBufRead + Send + Sized;

  fn get_session_logs_stream(
    &self,
    _shasta_token: &str,
    _site_name: &str,
    _cfs_session_name: &str,
    _k8s: &K8sDetails,
  ) -> impl Future<Output = Result<Self::T, Error>> {
    async {
      Err(Error::Message(
        "Get session logs stream command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  fn get_session_logs_stream_by_xname(
    &self,
    _auth_token: &str,
    _site_name: &str,
    _xname: &str,
    _k8s: &K8sDetails,
  ) -> impl Future<Output = Result<Self::T, Error>> {
    async {
      Err(Error::Message(
                "Get session logs stream by xname command not implemented for this backend"
                    .to_string(),
            ))
    }
  }

  fn get_cfs_health(&self) -> impl Future<Output = Result<(), Error>> {
    async {
      Err(Error::Message(
        "Get CFS health command not implemented for this backend".to_string(),
      ))
    }
  }

  fn post_session(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _session: &CfsSessionPostRequest,
  ) -> impl Future<Output = Result<CfsSessionGetResponse, Error>> + Send {
    async {
      Err(Error::Message(
        "Post session command not implemented for this backend".to_string(),
      ))
    }
  }

  fn get_sessions(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _session_name_opt: Option<&String>,
    _limit_opt: Option<u8>,
    _after_id_opt: Option<String>,
    _min_age_opt: Option<String>,
    _max_age_opt: Option<String>,
    _status_opt: Option<String>,
    _name_contains_opt: Option<String>,
    _is_succeded_opt: Option<bool>,
    _tags_opt: Option<String>,
  ) -> impl Future<Output = Result<Vec<CfsSessionGetResponse>, Error>> + Send
  {
    async {
      Err(Error::Message(
        "Get sessions command not implemented for this backend".to_string(),
      ))
    }
  }

  fn get_sessions_by_xname(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _xname_vec: &[&str],
    _limit_opt: Option<u8>,
    _after_id_opt: Option<String>,
    _min_age_opt: Option<String>,
    _max_age_opt: Option<String>,
    _status_opt: Option<String>,
    _name_contains_opt: Option<String>,
    _is_succeded_opt: Option<bool>,
    _tags_opt: Option<String>,
  ) -> impl Future<Output = Result<Vec<CfsSessionGetResponse>, Error>> + Send
  {
    async {
      Err(Error::Message(
        "Get sessions by xname command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  fn get_and_filter_sessions(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _hsm_group_name_vec_opt: Option<Vec<String>>,
    _xname_vec_opt: Option<Vec<&str>>,
    _min_age_opt: Option<&String>,
    _max_age_opt: Option<&String>,
    _status_opt: Option<&String>,
    _cfs_session_name_opt: Option<&String>,
    _limit_number_opt: Option<&u8>,
    _is_succeded_opt: Option<bool>,
  ) -> impl Future<Output = Result<Vec<CfsSessionGetResponse>, Error>> + Send
  {
    async {
      Err(Error::Message(
        "Get and filter sessions command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  fn get_configuration(
    &self,
    _auth_token: &str,
    _base_url: &str,
    _root_cert: &[u8],
    _configuration_name_opt: Option<&String>,
  ) -> impl Future<Output = Result<Vec<CfsConfigurationResponse>, Error>> + Send
  {
    async {
      Err(Error::Message(
        "Get configuration command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  fn get_and_filter_configuration(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _configuration_name: Option<&str>,
    _configuration_name_pattern: Option<&str>,
    _hsm_group_name_vec: &[String],
    _limit_number_opt: Option<&u8>,
  ) -> impl Future<Output = Result<Vec<CfsConfigurationResponse>, Error>> + Send
  {
    async {
      Err(Error::Message(
        "Get and filter configuration command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  fn get_configuration_layer_details(
    &self,
    _shasta_root_cert: &[u8],
    _gitea_base_url: &str,
    _gitea_token: &str,
    _layer: Layer,
    _site_name: &str, // FIXME: Should we move 'site_name' as Self.site_name?
  ) -> impl Future<Output = Result<LayerDetails, Error>> + Send {
    async {
      Err(Error::Message(
                "Get configuration layer details command not implemented for this backend"
                    .to_string(),
            ))
    }
  }

  fn create_configuration_from_repos(
    &self,
    _gitea_token: &str,
    _gitea_base_url: &str,
    _shasta_root_cert: &[u8],
    _repo_name_vec: Vec<String>,
    _local_git_commit_vec: Vec<String>,
    _playbook_file_name_opt: Option<&String>,
  ) -> impl Future<Output = Result<CfsConfigurationRequest, Error>> {
    async {
      Err(Error::Message(
                "Create configuration from repos command not implemented for this backend"
                    .to_string(),
            ))
    }
  }

  // This function enforces a new CFS configuration to be created. First, checks if CFS configuration
  // with same name already exists in CSM, if that is the case, it will return an error, otherwise
  // creates a new CFS configuration
  fn put_configuration(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _configuration: &CfsConfigurationRequest,
    _configuration_name: &str,
    _overwrite: bool,
  ) -> impl Future<Output = Result<CfsConfigurationResponse, Error>> + Send {
    async {
      Err(Error::Message(
        "Put configuration command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  fn update_runtime_configuration(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _xnames: Vec<String>,
    _desired_configuration: &str,
    _enabled: bool,
  ) -> impl Future<Output = Result<(), Error>> + Send {
    async {
      Err(Error::Message(
        "Update runtime configuration command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  // Get all CFS sessions, IMS images and BOS sessiontemplates related to a CFS configuration
  fn get_derivatives(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _configuration_name: &str,
  ) -> impl Future<
    Output = Result<
      (
        Option<Vec<CfsSessionGetResponse>>,
        Option<Vec<BosSessionTemplate>>,
        Option<Vec<Image>>,
      ),
      Error,
    >,
  > + Send {
    async {
      Err(Error::Message(
        "Get derivatives command not implemented for this backend".to_string(),
      ))
    }
  }

  fn get_cfs_components(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _configuration_name: Option<&str>,
    _components_ids: Option<&str>,
    _status: Option<&str>,
  ) -> impl Future<Output = Result<Vec<Component>, Error>> + Send {
    async {
      Err(Error::Message(
        "Get CFS conponents command not implemented for this backend"
          .to_string(),
      ))
    }
  }
}
