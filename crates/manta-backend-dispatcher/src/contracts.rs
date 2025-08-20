use crate::error::Error;

pub trait BackendTrait {
  fn test_backend_trait(&self) -> String;

  // AUTHORIZATION
  fn get_api_token(
    &self,
    _site_name: &str,
  ) -> impl std::future::Future<Output = Result<String, Error>> + Send;

  /// Get list of xnames from NIDs
  /// The list of NIDs can be:
  ///     - comma separated list of NIDs (eg: nid000001,nid000002,nid000003)
  ///     - regex (eg: nid00000.*)
  ///     - hostlist (eg: nid0000[01-15])
  fn nid_to_xname(
    &self,
    auth_token: &str,
    user_input_nid: &str,
    is_regex: bool,
  ) -> impl std::future::Future<Output = Result<Vec<String>, Error>> + Send;
}
