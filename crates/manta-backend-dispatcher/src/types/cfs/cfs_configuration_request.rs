use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Layer {
  #[serde(skip_serializing_if = "Option::is_none")]
  // Either commit or branch is passed
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  // Either commit or branch is passed
  pub clone_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  // Either commit or branch is passed
  pub source: Option<String>,
  pub playbook: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  // Either commit or branch is passed
  pub commit: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  // Either commit or branch is passed
  pub branch: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub special_parameters: Option<Vec<SpecialParameter>>,
}

impl Layer {
  pub fn new(
    name: Option<String>,
    clone_url: Option<String>,
    source: Option<String>,
    playbook: String,
    commit: Option<String>,
    branch: Option<String>,
    special_parameters: Option<Vec<SpecialParameter>>,
  ) -> Self {
    Self {
      clone_url,
      commit,
      name,
      playbook,
      branch,
      special_parameters,
      source,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpecialParameter {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ims_required_dkms: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AdditionalInventory {
  pub name: Option<String>,
  pub clone_url: String,
  pub source: Option<String>,
  pub commit: Option<String>,
  pub branch: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CfsConfigurationRequest {
  pub description: Option<String>,
  pub layers: Option<Vec<Layer>>,
  pub additional_inventory: Option<AdditionalInventory>,
}

impl Default for CfsConfigurationRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl CfsConfigurationRequest {
  pub fn new() -> Self {
    Self {
      description: None,
      layers: Some(Vec::default()),
      additional_inventory: None,
    }
  }

  pub fn add_layer(&mut self, layer: Layer) {
    if let Some(ref mut layers) = self.layers.as_mut() {
      layers.push(layer);
    }
  }
}
