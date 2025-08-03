use std::future::Future;

use chrono::NaiveDateTime;

use crate::{
  error::Error,
  types::{
    bss::BootParameters,
    cfs::{component::Component, session::CfsSessionGetResponse},
  },
};

pub trait CommandsTrait {
  fn i_delete_data_related_to_cfs_configuration(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _hsm_name_available_vec: Vec<String>,
    _configuration_name_opt: Option<&String>,
    _configuration_name_pattern: Option<&String>,
    _since_opt: Option<NaiveDateTime>,
    _until_opt: Option<NaiveDateTime>,
    _assume_yes: bool,
  ) -> impl Future<Output = Result<(), Error>> + Send {
    async {
      Err(Error::Message(
                "Delete data related to CFS configuration command not implemented for this backend"
                    .to_string(),
            ))
    }
  }
}
