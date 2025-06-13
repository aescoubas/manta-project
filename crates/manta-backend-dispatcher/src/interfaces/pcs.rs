use serde_json::Value;

use crate::error::Error;
use crate::types::pcs::power_status::types::PowerStatusAll;

pub trait PCSTrait {
  fn power_on_sync(
    &self,
    _auth_token: &str,
    _nodes: &[String],
  ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

  fn power_off_sync(
    &self,
    _auth_token: &str,
    _nodes: &[String],
    _force: bool,
  ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

  fn power_reset_sync(
    &self,
    _auth_token: &str,
    _nodes: &[String],
    _force: bool,
  ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

  // FIXME: Create a new type PowerStatus and return Result<PowerStatus, Error>
  fn power_status(
    &self,
    _auth_token: &str,
    //_nodes: &[String],
    _nodes: &[String],
    _power_status_filter: Option<&str>,
    _management_state_filter: Option<&str>,
  ) -> impl std::future::Future<Output = Result<PowerStatusAll, Error>> + Send
  {
    async {
      Err(Error::Message(
        "Power status command not implemented for this backend".to_string(),
      ))
    }
  }
}
