use bytes::Bytes;
use futures::stream::FuturesOrdered;
use m3u8_rs::{self, MasterPlaylist, MediaPlaylist, Playlist};
use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use std::str::FromStr;
use std::sync;
use std::sync::mpsc;

#[cfg(test)]
use mockall::automock;

use self::track::Track;

use super::*;

#[cfg_attr(test, automock(type Error=mpsc::SendError<Track>;))]
pub trait TrackSender {
    type Error;

    fn send(&self, track: Track) -> Result<(), Self::Error>;
}

impl TrackSender for mpsc::Sender<Track> {
    type Error = mpsc::SendError<Track>;

    fn send(&self, track: Track) -> Result<(), Self::Error> {
        mpsc::Sender::send(self, track)
    }
}

/// 1. source::from_factory
///    a. factory resolves playlist and creates a Track
///    b. a Track consists of samples that have been successfully fetched
///    c. the buffer will contain the entire track then initially.
/// 2. sink::play_sound

pub fn fetch_playlist(uri: String, fetch_uri: Box<FetchUriFactoryFn<Bytes>>) -> PinFuture<Track> {
    Box::pin(async move {
        let playlist_bytes = fetch_uri(uri).await?;

        match m3u8_rs::parse_playlist(&playlist_bytes) {
            Ok((_, Playlist::MasterPlaylist(pl))) => {
                return parse_m3u8_master_playlist(pl, fetch_uri).await;
            }
            Ok((_, Playlist::MediaPlaylist(pl))) => {
                let stream = media_playlist_stream_factory(&pl, fetch_uri);
                let track = Track::from_media_playlist(pl.to_owned(), stream).await;

                return Ok(track);
            }
            Err(e) => return Err(Box::new(e.to_owned())),
        }
    })
}

pub fn parse_m3u8_master_playlist(
    pl: MasterPlaylist,
    fetch_uri: Box<FetchUriFactoryFn<Bytes>>,
) -> PinFuture<Track> {
    Box::pin(async move {
        // take first variant
        // TODO: improve choice of variant
        if let Some(variant) = pl.variants.get(0) {
            return fetch_playlist(variant.uri.clone(), fetch_uri).await;
        }

        Err(Box::new(Error::new(
            std::io::ErrorKind::Other,
            "no variants in master playlist",
        )))
    })
}

// make media playlist stream
pub fn media_playlist_stream_factory(
    pl: &MediaPlaylist,
    fetch_uri: Box<FetchUriFactoryFn<Bytes>>,
) -> PlaylistStream {
    let mut futures = Vec::new();
    for segment in &pl.segments {
        futures.push(fetch_uri(segment.uri.clone()));
    }

    FuturesOrdered::from_iter(futures)
}
