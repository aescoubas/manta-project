use std::future::Future;

use crate::{error::Error, types::bss::BootParameters};

pub trait BootParametersTrait {
  fn get_all_bootparameters(
    &self,
    auth_token: &str,
  ) -> impl Future<Output = Result<Vec<BootParameters>, Error>> + Send;

  fn get_bootparameters(
    &self,
    auth_token: &str,
    nodes: &[String],
  ) -> impl Future<Output = Result<Vec<BootParameters>, Error>> + Send;

  fn add_bootparameters(
    &self,
    auth_token: &str,
    boot_parameters: &BootParameters,
  ) -> impl Future<Output = Result<(), Error>> + Send;

  fn update_bootparameters(
    &self,
    auth_token: &str,
    boot_parameters: &BootParameters,
  ) -> impl Future<Output = Result<(), Error>> + Send;

  fn delete_bootparameters(
    &self,
    auth_token: &str,
    boot_parameters: &BootParameters,
  ) -> impl Future<Output = Result<String, Error>> + Send;
}
