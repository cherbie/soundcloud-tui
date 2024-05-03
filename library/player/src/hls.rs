use bytes::Bytes;
use futures::stream::{self, FuturesOrdered};
use futures::{SinkExt, StreamExt};
use m3u8_rs::{self, MasterPlaylist, MediaPlaylist, Playlist};
use std::future::Future;
use std::pin::Pin;

pub type ResolveUriFactoryFn<T> = dyn Fn(&str) -> PinFuture<T>;
pub type AsyncError = Box<dyn std::error::Error + Send>;
pub type PinFuture<T> = Pin<Box<dyn Future<Output = Result<T, AsyncError>> + Send>>;
pub type PlaylistStream<T> = FuturesOrdered<PinFuture<T>>;

pub async fn parse_m3u8(
    bytes: Bytes,
    get_m3u8_factory: Box<ResolveUriFactoryFn<Bytes>>,
) -> Result<(), AsyncError> {
    match m3u8_rs::parse_playlist_res(&bytes) {
        Ok(Playlist::MasterPlaylist(pl)) => {
            Box::pin(parse_m3u8_master_playlist(pl, get_m3u8_factory)).await?;
        }
        Ok(Playlist::MediaPlaylist(pl)) => {
            let stream = new_media_playlist_stream(pl, get_m3u8_factory);
            // TODO: send the stream results to the tx
        }
        Err(e) => return Err(Box::new(e.to_owned())),
    }

    Ok(())
}

pub fn parse_m3u8_master_playlist(
    pl: MasterPlaylist,
    get_m3u8_factory: Box<ResolveUriFactoryFn<Bytes>>,
) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send>>>>> {
    Box::pin(async move {
        if let Some(variant) = pl.variants.get(0) {
            let bytes = Box::pin(get_m3u8_factory(&variant.uri)).await?;
            Box::pin(parse_m3u8(bytes, get_m3u8_factory)).await?;
        }

        Ok(())
    })
}

pub fn new_media_playlist_stream(
    pl: MediaPlaylist,
    get_m3u8_factory: Box<ResolveUriFactoryFn<Bytes>>,
) -> PlaylistStream<Bytes> {
    let mut futures = Vec::new();
    for segment in &pl.segments {
        futures.push(get_m3u8_factory(&segment.uri));
    }

    FuturesOrdered::from_iter(futures)
}
