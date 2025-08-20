use serde::{Deserialize, Serialize};

use crate::types::pcs::transitions::types::Operation;

#[derive(Debug, Serialize, Deserialize)]
pub enum PowerState {
  #[serde(rename = "on")]
  On,
  #[serde(rename = "off")]
  Off,
  #[serde(rename = "undefined")]
  Undefined,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ManagementState {
  Unavailable,
  Available,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerStatus {
  pub xname: String,
//  #[serde(skip_serializing_if = "Option::is_none")]
//  pub power_state_filter: Option<PowerState>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "powerState")]
  pub power_state: Option<PowerState>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "management_state")]
  pub management_state: Option<ManagementState>,
//  #[serde(skip_serializing_if = "Option::is_none")]
//  pub management_state_filter: Option<ManagementState>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub error: Option<String>,
  //#[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "supportedPowerTransitions")]
  pub supported_power_transitions: Vec<Operation>,
  pub last_updated: String,
}

//TODO: define a PowerStatusAll,
// This is actually what is returned from the pcs endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct PowerStatusAll {
  pub status: Vec<PowerStatus>
}
