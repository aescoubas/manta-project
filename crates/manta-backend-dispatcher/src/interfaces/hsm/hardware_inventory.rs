use serde_json::Value;

use crate::{error::Error, types::HWInventoryByLocationList};

pub trait HardwareInventory {
  fn post_inventory_hardware(
    &self,
    auth_token: &str,
    hardware: HWInventoryByLocationList,
  ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

  fn get_inventory_hardware(
    &self,
    auth_token: &str,
    xname: &str,
  ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

  fn get_inventory_hardware_query(
    &self,
    auth_token: &str,
    xname: &str,
    r#type: Option<&str>,
    children: Option<bool>,
    parents: Option<bool>,
    partition: Option<&str>,
    format: Option<&str>,
  ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;
}
