use std::{
    io::{Read, Seek},
    pin::Pin,
    sync::mpsc,
};

use bytes::Bytes;
use futures::{FutureExt, StreamExt};
use m3u8_rs::{MediaPlaylist, MediaSegment};
use rodio::{source, Decoder, Sample, Source};
use std::io;
use std::sync;

use super::*;

#[derive(Debug)]
pub struct TrackSample {
    sample: Bytes,
}

impl TrackSample {
    fn new(bytes: Bytes) -> Self {
        Self { sample: bytes }
    }
}

impl TryInto<Decoder<io::Cursor<Bytes>>> for TrackSample {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> Result<Decoder<io::Cursor<Bytes>>, Self::Error> {
        let cursor = io::Cursor::new(self.sample);
        let decoder = Decoder::new(cursor)?;
        Ok(decoder)
    }
}

impl From<Bytes> for TrackSample {
    fn from(bytes: Bytes) -> Self {
        TrackSample::new(bytes)
    }
}

impl Into<Bytes> for TrackSample {
    fn into(self) -> Bytes {
        self.sample
    }
}

#[derive(Debug)]
pub struct Track {
    samples: Vec<TrackSample>,
    media: MediaPlaylist,
}

impl Track {
    pub fn to_decoders(
        self,
    ) -> Result<Vec<Decoder<io::Cursor<Bytes>>>, Box<dyn std::error::Error>> {
        let decoders = self
            .samples
            .into_iter()
            .map(|sample| sample.try_into())
            .collect::<Result<Vec<Decoder<io::Cursor<Bytes>>>, Box<dyn std::error::Error>>>()?;

        Ok(decoders)
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
