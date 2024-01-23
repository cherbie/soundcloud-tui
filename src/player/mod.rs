// TODO: implement
#![allow(unused)]

mod hls;

use rodio::{cpal::FromSample, OutputStream, OutputStreamHandle, Sample, Sink, Source};
use std::cell::Cell;

pub struct TrackPlayer {
    sink: Sink,
    output_stream: Option<(OutputStream, OutputStreamHandle)>,
}

impl Default for TrackPlayer {
    fn default() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.pause();

        TrackPlayer {
            sink,
            output_stream: Some((stream, stream_handle)),
        }
    }
}

impl TrackPlayer {
    fn queue<S>(&self, source: S)
    where
        S: Source + Send + 'static,
        f32: FromSample<S::Item>,
        S::Item: Sample + Send,
    {
        self.sink.append(source);
    }

    fn len(&self) -> usize {
        self.sink.len()
    }

    fn play(&self) {
        self.sink.play();
    }

    fn pause(&self) {
        self.sink.pause();
    }

    #[cfg(test)]
    fn block(&self) {
        self.sink.sleep_until_end();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rodio::source::{SineWave, Source};
    use std::time::Duration;

    impl TrackPlayer {
        fn mock() -> Self {
            let (sink, _) = Sink::new_idle();

            Self {
                sink,
                output_stream: None,
            }
        }
    }

    #[tokio::test]
    async fn test_audio_output() {
        let player = TrackPlayer::mock();

        player.queue(
            SineWave::new(440.0)
                .take_duration(Duration::from_secs_f32(2.0))
                .amplify(0.20),
        );
        player.queue(
            SineWave::new(420.0)
                .take_duration(Duration::from_secs_f32(2.0))
                .amplify(0.20),
        );

        assert_eq!(player.len(), 2);
        player.play();
    }
}
