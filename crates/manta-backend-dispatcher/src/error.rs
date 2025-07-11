use serde_json::Value;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("ERROR - Message: {0}")]
  Message(String),
  #[error("ERROR - IO: {0}")]
  IoError(#[from] std::io::Error),
  #[error("ERROR - Serde: {0}")]
  SerdeError(#[from] serde_json::Error),
  #[error("ERROR - http client: {0}")]
  NetError(#[from] reqwest::Error),
  #[error("ERROR - http request:\nresponse: {response}\npayload: {payload}")]
  RequestError {
    response: reqwest::Error,
    payload: String, // NOTE: CSM/OCHAMI Apis either returns plain text or a json therefore, we
                     // will just return a String
  },
  #[error("ERROR - Backend: {0}")]
  CsmError(Value),
  #[error("ERROR - Console: {0}")]
  ConsoleError(String),
  #[error("ERROR - CFS Configuration already exists: {0}")]
  ConfigurationAlreadyExistsError(String),
}
