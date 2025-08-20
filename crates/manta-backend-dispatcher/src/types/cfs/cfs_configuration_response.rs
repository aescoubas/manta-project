use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)] // TODO: investigate why serde can Deserialize dynamically syzed structs `Vec<Layer>`
pub struct Next {
  pub limit: Option<u8>,
  pub after_id: Option<String>,
  pub in_use: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Layer {
  pub name: String,
  // #[serde(rename = "cloneUrl")]
  pub clone_url: String,
  pub source: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  // Either commit or branch is passed
  pub commit: Option<String>,
  pub playbook: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  // Either commit or branch is passed
  pub branch: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AdditionalInventory {
  pub name: String,
  pub clone_url: String,
  pub commit: Option<String>,
  pub branch: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpecialParameter {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ims_required_dkms: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CfsConfigurationResponse {
  pub name: String,
  pub last_updated: String,
  pub layers: Vec<Layer>,
  #[serde(skip_serializing_if = "Option::is_none")]
  // Either commit or branch is passed
  pub additional_inventory: Option<AdditionalInventory>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CfsConfigurationVecResponse {
  pub configurations: Vec<CfsConfigurationResponse>,
  pub next: Option<Next>,
}
