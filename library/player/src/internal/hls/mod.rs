pub mod playlist;
#[cfg(test)]
mod playlist_test;
pub mod track;
#[cfg(test)]
mod track_test;
mod ts;
#[cfg(test)]
pub mod ts_test;

use std::pin::Pin;

use bytes::Bytes;
use futures::{stream::FuturesOrdered, Future};

pub type FetchUriFactoryFn<T> = dyn (Fn(String) -> PinFuture<T>) + Send;
type AsyncError = super::error::AsyncError;
pub type PinFuture<T> = Pin<Box<dyn Future<Output = Result<T, AsyncError>> + Send>>;
pub type PlaylistStream = FuturesOrdered<PinFuture<Bytes>>;
