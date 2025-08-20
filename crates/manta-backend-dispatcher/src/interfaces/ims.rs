use std::future::Future;

use crate::{error::Error, types::ims::Image};

pub trait ImsTrait {
  fn get_images(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _image_id_opt: Option<&str>,
  ) -> impl Future<Output = Result<Vec<Image>, Error>> + Send {
    async {
      Err(Error::Message(
        "Get images command not implemented for this backend".to_string(),
      ))
    }
  }

  fn get_all_images(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
  ) -> impl Future<Output = Result<Vec<Image>, Error>> + Send {
    async {
      Err(Error::Message(
        "Get all images command not implemented for this backend".to_string(),
      ))
    }
  }

  fn filter_images(&self, _image_vec: &mut Vec<Image>) -> Result<(), Error> {
    Err(Error::Message(
      "Filter images command not implemented for this backend".to_string(),
    ))
  }

  fn delete_image(
    &self,
    _shasta_token: &str,
    _shasta_base_url: &str,
    _shasta_root_cert: &[u8],
    _image_id: &str,
  ) -> impl Future<Output = Result<(), Error>> + Send {
    async {
      Err(Error::Message(
        "Delete image command not implemented for this backend".to_string(),
      ))
    }
  }
}
