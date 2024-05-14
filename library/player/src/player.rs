use crate::internal::client;

use super::device;
use super::internal;

use rodio;
use std::sync::Arc;
use tokio;

trait TrackPlayerService {
    fn enqueue(&self, song_uri: &str);
    fn play(&self);
    fn pause(&self);
    fn skip(&self);
}

enum DeviceEvent {
    Play,
    Pause,
    Skip,
}

enum PlayerEvent {
    Device(DeviceEvent),
    Enqueue(String),
}

pub struct TrackPlayer {
    // buffered songs
    event_tx: Box<tokio::sync::mpsc::Sender<PlayerEvent>>,
    event_rx: Option<tokio::sync::mpsc::Receiver<PlayerEvent>>,

    // TODO: error signal for worker? ... should probably be the join handle failure

    // singletons
    client_service: Arc<dyn internal::ReqClient>,
    track_lookup_service: Box<dyn internal::TrackLookupService>,
    device_service: Box<dyn device::DeviceService>,
}

impl Default for TrackPlayer {
    // TODO: deprecate me
    fn default() -> Self {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        let device = device::DeviceFactory::new(sink);

        let (tx, rx) = tokio::sync::mpsc::channel::<PlayerEvent>(5);

        let client_service = Arc::new(internal::ClientFactory::new());

        TrackPlayer {
            event_tx: Box::new(tx),
            event_rx: Some(rx),

            client_service: client_service.clone(),
            track_lookup_service: Box::new(internal::TrackLookupFactory::new(
                client_service.clone(),
            )),
            device_service: Box::new(device),
        }
    }
}

impl TrackPlayer {
    pub async fn start(&mut self) -> Result<(), internal::error::AsyncError> {
        if self.event_rx.is_none() {
            return Ok(());
        }

        loop {
            match self.event_rx.as_mut().unwrap().recv().await {
                Some(PlayerEvent::Enqueue(song)) => {
                    self.handle_enqueue_event(&song).await?;
                }
                Some(PlayerEvent::Device(event)) => {
                    self.handle_device_event(&event).await?;
                }
                None => {
                    break;
                }
            }
        }

        Ok(())
    }

    async fn handle_device_event(
        &self,
        event: &DeviceEvent,
    ) -> Result<(), internal::error::AsyncError> {
        match event {
            DeviceEvent::Play => Ok(self.device_service.play()?),
            DeviceEvent::Pause => Ok(self.device_service.pause()?),
            DeviceEvent::Skip => Ok(self.device_service.skip()?),
        }
    }

    async fn handle_enqueue_event(
        &mut self,
        song: &str,
    ) -> Result<(), internal::error::AsyncError> {
        // TODO: resolve track with this async service
        let track = self.track_lookup_service.get_track(song).await?;
        let decoders = track.to_decoders().map_err(|e| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;

        use rodio::Source;
        let track_sample =
            rodio::source::from_iter(Box::new(decoders.into_iter())).convert_samples();
        let _ = self.device_service.enqueue(Box::new(track_sample));

        Ok(())
    }
}

impl TrackPlayerService for TrackPlayer {
    fn enqueue(&self, song_uri: &str) {
        let tx = self.event_tx.clone();
        let song = song_uri.to_string();
        tokio::spawn(async move { tx.send(PlayerEvent::Enqueue(song)).await });
    }

    fn pause(&self) {
        let tx = self.event_tx.clone();
        tokio::spawn(async move { tx.send(PlayerEvent::Device(DeviceEvent::Pause)).await });
    }

    fn play(&self) {
        let tx = self.event_tx.clone();
        tokio::spawn(async move { tx.send(PlayerEvent::Device(DeviceEvent::Play)).await });
    }

    fn skip(&self) {
        let tx = self.event_tx.clone();
        tokio::spawn(async move { tx.send(PlayerEvent::Device(DeviceEvent::Skip)).await });
    }
}
