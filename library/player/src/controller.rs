use super::player::TrackPlayer;

pub trait Worker {
    fn start(&mut self);
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct PlayerWorker {
    inner: TrackPlayer,
    handle: Option<tokio::task::JoinHandle<()>>,
}

impl PlayerWorker {
    pub fn new(player: TrackPlayer) -> Self {
        Self {
            inner: player,
            handle: None,
        }
    }
}

impl Worker for PlayerWorker {
    fn start(&mut self) {
        self.handle = Some(tokio::spawn(async move { () }));
    }

    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(handle) = self.handle.as_mut() {
            handle.abort();
        }

        Ok(())
    }
}
