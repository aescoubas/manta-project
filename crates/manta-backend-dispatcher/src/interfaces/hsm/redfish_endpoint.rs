use std::future::Future;

use serde_json::Value;

use crate::{
  error::Error,
  types::hsm::inventory::{RedfishEndpoint, RedfishEndpointArray},
};

pub trait RedfishEndpointTrait {
  fn get_all_redfish_endpoints(
    &self,
    auth_token: &str,
  ) -> impl Future<Output = Result<RedfishEndpointArray, Error>> + Send;

  fn get_redfish_endpoints(
    &self,
    auth_token: &str,
    id: Option<&str>,
    fqdn: Option<&str>,
    r#type: Option<&str>,
    uuid: Option<&str>,
    macaddr: Option<&str>,
    ip_address: Option<&str>,
    last_status: Option<&str>,
  ) -> impl Future<Output = Result<RedfishEndpointArray, Error>> + Send;

  fn add_redfish_endpoint(
    &self,
    auth_token: &str,
    redfish_endpoint: &RedfishEndpointArray,
  ) -> impl Future<Output = Result<(), Error>> + Send;

  fn update_redfish_endpoint(
    &self,
    auth_token: &str,
    redfish_endpoint: &RedfishEndpoint,
  ) -> impl Future<Output = Result<(), Error>> + Send;

  fn delete_redfish_endpoint(
    &self,
    auth_token: &str,
    id: &str,
  ) -> impl Future<Output = Result<Value, Error>> + Send;
}
