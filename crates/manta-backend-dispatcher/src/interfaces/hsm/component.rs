use crate::{
  error::Error,
  types::{Component, ComponentArrayPostArray, NodeMetadataArray},
};
use serde_json::Value;

pub trait ComponentTrait {
  fn get_all_nodes(
    &self,
    auth_token: &str,
    nid_only: Option<&str>,
  ) -> impl std::future::Future<Output = Result<NodeMetadataArray, Error>> + Send;

  fn get(
    &self,
    auth_token: &str,
    id: Option<&str>,
    r#type: Option<&str>,
    state: Option<&str>,
    flag: Option<&str>,
    role: Option<&str>,
    subrole: Option<&str>,
    enabled: Option<&str>,
    software_status: Option<&str>,
    subtype: Option<&str>,
    arch: Option<&str>,
    class: Option<&str>,
    nid: Option<&str>,
    nid_start: Option<&str>,
    nid_end: Option<&str>,
    partition: Option<&str>,
    group: Option<&str>,
    state_only: Option<&str>,
    flag_only: Option<&str>,
    role_only: Option<&str>,
    nid_only: Option<&str>,
  ) -> impl std::future::Future<Output = Result<NodeMetadataArray, Error>> + Send;

  fn get_node_metadata_available(
    &self,
    auth_token: &str,
  ) -> impl std::future::Future<Output = Result<Vec<Component>, Error>> + Send;

  // HSM/COMPONENT
  fn post_nodes(
    &self,
    auth_token: &str,
    component: ComponentArrayPostArray,
  ) -> impl std::future::Future<Output = Result<(), Error>> + Send;

  // HSM/COMPONENT
  fn delete_node(
    &self,
    auth_token: &str,
    id: &str,
  ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;
}
