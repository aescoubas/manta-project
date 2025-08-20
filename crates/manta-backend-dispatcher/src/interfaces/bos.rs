use std::future::Future;

use serde_json::Value;

use crate::{
  error::Error,
  types::bos::{session::BosSession, session_template::BosSessionTemplate},
};

pub trait ClusterTemplateTrait {
  fn get_template(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _bos_session_template_id_opt: Option<&str>,
  ) -> impl Future<Output = Result<Vec<BosSessionTemplate>, Error>> + Send {
    async {
      Err(Error::Message(
        "Get template command not implemented for this backend".to_string(),
      ))
    }
  }

  fn get_and_filter_templates(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _hsm_group_name_vec: &Vec<String>,
    _hsm_member_vec: &[String],
    _bos_sessiontemplate_name_opt: Option<&String>,
    _limit_number_opt: Option<&u8>,
  ) -> impl Future<Output = Result<Vec<BosSessionTemplate>, Error>> + Send {
    async {
      Err(Error::Message(
        "Get templates command not implemented for this backend".to_string(),
      ))
    }
  }

  fn get_all_templates(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
  ) -> impl Future<Output = Result<Vec<BosSessionTemplate>, Error>> + Send {
    async {
      Err(Error::Message(
        "Get all templates command not implemented for this backend"
          .to_string(),
      ))
    }
  }

  fn put_template(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _bos_template: &BosSessionTemplate,
    _bos_template_name: &str,
  ) -> impl Future<Output = Result<BosSessionTemplate, Error>> + Send {
    async {
      Err(Error::Message(
        "Put template command not implemented for this backend".to_string(),
      ))
    }
  }

  fn delete_template(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _bos_template_id: &str,
  ) -> impl Future<Output = Result<(), Error>> + Send {
    async {
      Err(Error::Message(
        "Delete template command not implemented for this backend".to_string(),
      ))
    }
  }
}

pub trait ClusterSessionTrait {
  fn post_template_session(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _bos_session: BosSession,
  ) -> impl Future<Output = Result<BosSession, Error>> + Send {
    async {
      Err(Error::Message(
        "Create session template command not implemented for this backend"
          .to_string(),
      ))
    }
  }
}
