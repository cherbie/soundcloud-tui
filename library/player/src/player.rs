use crate::hls::{track::Track, AsyncError};

use super::hls::track;
use rodio::{cpal::FromSample, OutputStream, OutputStreamHandle, PlayError, Sample, Sink, Source};
use tokio::{
    sync::{
        self,
        mpsc::{error::SendError, Receiver, Sender},
    },
    task::JoinHandle,
};

trait TrackPlayerService {
    fn enqueue_song(&mut self, song_uri: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn play(&self);
    fn pause(&self);
    fn skip(&self);
}

pub struct TrackPlayer {
    // device
    sink: Sink,
    sink_stream: Option<(OutputStream, OutputStreamHandle)>,

    // buffered songs
    tracks_channel: Option<(Sender<Track>, Receiver<Track>)>,

    song_queue: Vec<String>,
    song_current: Option<String>,

    // transcient services
    track_lookup_service: Box<dyn track::TrackLookupService>,
}

impl Default for TrackPlayer {
    // TODO: deprecate me
    fn default() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.pause();

        TrackPlayer {
            sink,
            sink_stream: Some((stream, stream_handle)),
            tracks_channel: Some(tokio::sync::mpsc::channel(5)),
            song_queue: vec![],
            song_current: None,
            track_lookup_service: Box::new(track::TrackLookup::default()),
        }
    }
}

/// The difficulty lies in trying to call a tokio::spawn function to run async code ... and this listen to the result in a blocking fashion.
///

impl TrackPlayer {
    /// TODO initialize source
    fn init_source(&mut self) -> Result<(), PlayError> {
        let source = rodio::source::Empty::new();

        if let Some((_, stream_handle)) = &mut self.sink_stream {
            return stream_handle.play_raw(source);
        } else {
            return Err(PlayError::NoDevice);
        }
    }

    /// Async background fetch of the next song uri.
    /// The mpsc channel will be populated with the song result
    fn next_track_factory(
        &mut self,
    ) -> Result<JoinHandle<Result<(), SendError<Track>>>, AsyncError> {
        if let Some(uri) = self.song_queue.pop() {
            if let Some((tx, _)) = &self.tracks_channel {
                let tx_child = tx.clone();
                let track_fetch = self.track_lookup_service.fetch_track(&uri);

                // async wait for hls resolution
                // FIXME: task JoinHandle result not handled
                return Ok(tokio::spawn(async move {
                    // TODO: need to reserve capacity
                    match track_fetch.await {
                        Ok(track) => tx_child.send(track).await,
                        Err(e) => {
                            // TODO: need to send error information over channel
                            // FIXME: not hanlding async fetch error
                            print!("error fetching track info: {}", e);
                            Ok(())
                        }
                    }
                }));
            } else {
                // no channel configured
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "no channel configured",
                )));
            }
        } else {
            // empty queue
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "empty song queue",
            )));
        }
    }
}

impl TrackPlayerService for TrackPlayer {
    fn enqueue_song(&mut self, song_uri: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.song_queue.push(song_uri.to_string());

        Ok(())
    }

    fn pause(&self) {
        self.sink.pause();
    }

    fn play(&self) {
        if self.song_current.is_none() {
            self.skip();
        }
        self.sink.play();
    }

    fn skip(&self) {
        self.sink.skip_one();
    }
}
