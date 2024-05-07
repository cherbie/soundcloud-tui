use std::{
    io::{Read, Seek},
    pin::Pin,
    sync::mpsc,
};

use bytes::{buf::Chain, Buf, Bytes};
use futures::StreamExt;
use m3u8_rs::{MediaPlaylist, MediaSegment};
use rodio::{source, Decoder, Sample, Source};
use std::io;
use std::sync;

use super::{client::*, PinFuture, PlaylistStream};

#[derive(Debug)]
struct TrackSample {
    sample: Bytes,
}

impl TrackSample {
    pub fn new(sample: Bytes) -> Self {
        Self { sample }
    }

    fn to_decoder(&self) -> Result<Decoder<io::Cursor<Bytes>>, Box<dyn std::error::Error>> {
        let cursor = io::Cursor::new(self.sample.clone());
        let decoder = Decoder::new(cursor)?;
        Ok(decoder)
    }
}

#[derive(Debug)]
pub struct Track {
    samples: Vec<TrackSample>,
    media: MediaPlaylist,
}

impl Track {
    pub fn to_decoders(
        &self,
    ) -> Result<
        Box<
            dyn IntoIterator<
                Item = Decoder<io::Cursor<Bytes>>,
                IntoIter = std::vec::IntoIter<Decoder<io::Cursor<Bytes>>>,
            >,
        >,
        Box<dyn std::error::Error>,
    > {
        let mut decoders: Vec<Decoder<io::Cursor<Bytes>>> = Vec::new();
        for sample in self.samples.iter() {
            let decoder = sample.to_decoder()?;
            decoders.push(decoder);
        }

        Ok(Box::new(decoders.into_iter()))
    }

    pub fn get_sequence(&self) -> u64 {
        self.media.media_sequence
    }
}

impl Track {
    pub async fn from_media_playlist(
        media: MediaPlaylist,
        mut segment_stream: PlaylistStream,
    ) -> Self {
        let mut samples = Vec::new();
        while let Some(segment) = segment_stream.next().await {
            let bytes = match segment {
                Ok(bytes) => bytes,
                Err(e) => panic!("error fetching segment: {}", e),
            };
            samples.push(TrackSample::new(bytes));
        }

        Track { samples, media }
    }
}

pub trait TrackLookupService {
    fn fetch_track(&self, uri: &str) -> PinFuture<Track>;
}

pub struct TrackLookup {
    client: sync::Arc<dyn ReqClient + Send + Sync>,
}

impl Default for TrackLookup {
    fn default() -> Self {
        Self {
            client: sync::Arc::new(ClientFactory::new()),
        }
    }
}

impl TrackLookupService for TrackLookup {
    fn fetch_track(&self, uri: &str) -> PinFuture<Track> {
        let client = self.client.clone();

        super::playlist::fetch_playlist(uri.to_string(), Box::new(move |uri| client.fetch(uri)))
    }
}
