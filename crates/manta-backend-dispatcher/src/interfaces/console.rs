use std::future::Future;

use tokio::io::{AsyncRead, AsyncWrite};

use crate::{error::Error, types::K8sDetails};

/// This trait returns a tuple like `(stdin, stdout, stderr)`
pub trait ConsoleTrait {
  type T: AsyncWrite + Unpin;
  type U: AsyncRead + Unpin;

  fn attach_to_console(
    &self,
    _shasta_token: &str,
    _site_name: &str,
    _xname: &str,
    _term_width: u16,
    _term_height: u16,
    _k8s: &K8sDetails,
  ) -> impl Future<Output = Result<(Self::T, Self::U), Error>> + Send {
    async {
      Err(Error::Message(
        "Attach to console command not implemented for this backend"
          .to_string(),
      ))
    }
  }
}
