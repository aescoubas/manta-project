use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Location {
  pub xname: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "deputyKey")]
  pub deputy_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Operation {
  #[serde(rename = "on")]
  On,
  #[serde(rename = "off")]
  Off,
  #[serde(rename = "soft-off")]
  SoftOff,
  #[serde(rename = "soft-restart")]
  SoftRestart,
  #[serde(rename = "hard-restart")]
  HardRestart,
  #[serde(rename = "init")]
  Init,
  #[serde(rename = "force-off")]
  ForceOff,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transition {
  pub operation: Operation,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "taskDeadlineMinutes")]
  pub task_deadline_minutes: Option<usize>,
  pub location: Vec<Location>,
}
