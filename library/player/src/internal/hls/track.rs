use std::{
    io::{Read, Seek},
    pin::Pin,
    sync::mpsc,
};

use bytes::{buf::Chain, Buf, Bytes};
use futures::{FutureExt, StreamExt};
use m3u8_rs::{MediaPlaylist, MediaSegment};
use rodio::{source, Decoder, Sample, Source};
use std::io;
use std::sync;

use super::*;

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
    ) -> Result<Vec<Decoder<io::Cursor<Bytes>>>, Box<dyn std::error::Error>> {
        let mut decoders: Vec<Decoder<io::Cursor<Bytes>>> = Vec::new();
        for sample in self.samples.iter() {
            let decoder = sample.to_decoder()?;
            decoders.push(decoder);
        }

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
