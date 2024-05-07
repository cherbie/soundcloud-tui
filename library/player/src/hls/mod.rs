pub mod client;
pub mod playlist;
#[cfg(test)]
mod playlist_test;
pub mod track;

use std::pin::Pin;

use bytes::Bytes;
use futures::{stream::FuturesOrdered, Future};

pub type FetchUriFactoryFn<T> = dyn (Fn(String) -> PinFuture<T>) + Send;
pub type AsyncError = Box<dyn std::error::Error + Send + Sync>;
pub type PinFuture<T> = Pin<Box<dyn Future<Output = Result<T, AsyncError>> + Send>>;
pub type PlaylistStream = FuturesOrdered<PinFuture<Bytes>>;
