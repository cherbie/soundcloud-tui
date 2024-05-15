use std::io::Error;

use bytes::Bytes;
#[cfg(test)]
use mockall::automock;

use super::hls;

#[cfg_attr(test, automock)]
pub trait ReqClient {
    fn fetch(&self, uri: String) -> hls::PinFuture<Bytes>;
}

pub struct ClientFactory;

impl ClientFactory {
    pub fn new() -> impl ReqClient {
        ReqClientInner {}
    }
}

struct ReqClientInner;

impl ReqClient for ReqClientInner {
    fn fetch(&self, uri: String) -> hls::PinFuture<Bytes> {
        Box::pin(async move {
            match reqwest::get(uri).await {
                Ok(response) => {
                    if let Ok(bytes) = response.bytes().await {
                        return Ok(Bytes::from(bytes));
                    }
                }
                Err(e) => {
                    return Err(
                        Box::new(Error::new(std::io::ErrorKind::Other, e.to_string())).into(),
                    );
                }
            }

            Err(Box::new(Error::new(std::io::ErrorKind::Other, "failed to fetch")).into())
        })
    }
}
