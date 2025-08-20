use std::collections::HashMap;

use serde_json::Value;

use crate::{error::Error, types::Group};

pub trait GroupTrait {
  fn get_group_available(
    &self,
    auth_token: &str,
  ) -> impl std::future::Future<Output = Result<Vec<Group>, Error>> + Send;

  fn get_group_name_available(
    &self,
    _jwt_token: &str,
  ) -> impl std::future::Future<Output = Result<Vec<String>, Error>> + Send;

  fn add_group(
    &self,
    auth_token: &str,
    hsm_name: Group,
  ) -> impl std::future::Future<Output = Result<Group, Error>> + Send;

  fn get_member_vec_from_group_name_vec(
    &self,
    auth_token: &str,
    hsm_group_name_vec: Vec<String>,
  ) -> impl std::future::Future<Output = Result<Vec<String>, Error>> + Send;

  fn get_group_map_and_filter_by_group_vec(
    &self,
    _auth_token: &str,
    _hsm_name_vec: Vec<&str>,
  ) -> impl std::future::Future<Output = Result<HashMap<String, Vec<String>>, Error>>
       + Send;

  fn get_group_map_and_filter_by_member_vec(
    &self,
    _auth_token: &str,
    _member_vec: &[&str],
  ) -> impl std::future::Future<Output = Result<HashMap<String, Vec<String>>, Error>>
       + Send;

  fn get_all_groups(
    &self,
    _auth_token: &str,
  ) -> impl std::future::Future<Output = Result<Vec<Group>, Error>> + Send;

  fn get_group(
    &self,
    _auth_token: &str,
    _hsm_name: &str,
  ) -> impl std::future::Future<Output = Result<Group, Error>> + Send;

  fn get_groups(
    &self,
    _auth_token: &str,
    _hsm_name_vec: Option<&[&str]>,
  ) -> impl std::future::Future<Output = Result<Vec<Group>, Error>> + Send;

  fn delete_group(
    &self,
    _auth_token: &str,
    _hsm_group_label: &str,
  ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

  fn get_hsm_map_and_filter_by_hsm_name_vec(
    &self,
    auth_token: &str,
    hsm_name_vec: Vec<&str>,
  ) -> impl std::future::Future<Output = Result<HashMap<String, Vec<String>>, Error>>
       + Send;

  fn post_member(
    &self,
    auth_token: &str,
    group_label: &str,
    members: &str,
  ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

  fn add_members_to_group(
    &self,
    auth_token: &str,
    group_label: &str,
    members: Vec<&str>,
  ) -> impl std::future::Future<Output = Result<Vec<String>, Error>> + Send;

  fn delete_member_from_group(
    &self,
    auth_token: &str,
    group_label: &str,
    xname: &str,
  ) -> impl std::future::Future<Output = Result<(), Error>> + Send;

  fn update_group_members(
    &self,
    auth_token: &str,
    group_name: &str,
    members_to_remove: &Vec<String>,
    members_to_add: &Vec<String>,
  ) -> impl std::future::Future<Output = Result<(), Error>> + Send;

  fn migrate_group_members(
    &self,
    shasta_token: &str,
    target_hsm_group_name: &str,
    parent_hsm_group_name: &str,
    new_target_hsm_members: Vec<&str>,
  ) -> impl std::future::Future<Output = Result<(Vec<String>, Vec<String>), Error>>
       + Send;
}
