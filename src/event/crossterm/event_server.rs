use super::super::event_server::*;
use super::super::utils::*;
use super::super::Event;
use crate::utils;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct CrosstermEventServer<T> {
    config: EventConfig,
    rx: mpsc::Receiver<Event<T>>,
    tx: mpsc::Sender<Event<T>>,
    handle: Option<thread::JoinHandle<()>>,
    exit_condition: Arc<Mutex<bool>>,
}

impl Default for CrosstermEventServer<crossterm::event::Event> {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();

        CrosstermEventServer {
            config: EventConfig::default(),
            rx,
            tx,
            handle: None,
            exit_condition: Arc::new(Mutex::new(false)),
        }
    }
}

impl EventServer for CrosstermEventServer<crossterm::event::Event> {
    fn config(&self) -> EventConfig {
        self.config
    }

    fn listen<T, U>(&mut self)
    where
        T: utils::threads::Spawn,
        U: super::super::utils::EventPoll
            + super::super::utils::EventRead<Event = crossterm::event::Event>,
    {
        let event_tx = self.tx.clone();
        let tick_rate = self.config.tick_rate;

        let exit_condition = Arc::clone(&self.exit_condition);

        let join_handle = T::spawn(move || loop {
            {
                if *exit_condition.lock().unwrap() == true {
                    break;
                }
            }
            poll_for_event::<U, crossterm::event::Event>(&event_tx, tick_rate)
        });

        self.handle = Some(join_handle);
    }

    fn stop(&mut self) {
        self.stop()
    }
}

impl<T> CrosstermEventServer<T> {
    fn stop(&mut self) {
        {
            let mut exit_condition = self.exit_condition.lock().unwrap();
            *exit_condition = true;
        }

        if self.handle.is_some() {
            self.handle
                .take()
                .unwrap()
                .join()
                .expect("crossterm event thread join panic.");
        }
    }
}

impl<T> Drop for CrosstermEventServer<T> {
    fn drop(&mut self) {
        self.stop();
    }
}

impl Iterator for CrosstermEventServer<crossterm::event::Event> {
    type Item = Event<crossterm::event::Event>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rx.recv().ok()
    }
}
