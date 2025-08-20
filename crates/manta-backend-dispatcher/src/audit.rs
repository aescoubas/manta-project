use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::{error::Error, types::kafka::Kafka};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auditor {
  pub kafka: Option<Kafka>,
  // pub syslog: Option<Syslog>,
}

pub trait Audit {
  fn produce_message(
    &self,
    data: &[u8],
  ) -> impl Future<Output = Result<(), Error>> + Send;
}
