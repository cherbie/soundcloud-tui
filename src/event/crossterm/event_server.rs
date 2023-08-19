use super::super::event_server::*;
use super::super::Event;
use ::crossterm;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct CrosstermEventServer {
    config: EventConfig,
    rx: mpsc::Receiver<Event<crossterm::event::Event>>,
    tx: mpsc::Sender<Event<crossterm::event::Event>>,
    handle: Option<thread::JoinHandle<()>>,
    exit_condition: Arc<Mutex<bool>>,
}

impl Default for CrosstermEventServer {
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

fn crossterm_poll_for_event(
    tx: &mpsc::Sender<Event<crossterm::event::Event>>,
    tick_rate: Duration,
) {
    if crossterm::event::poll(tick_rate).unwrap() {
        let event = crossterm::event::read().unwrap();
        tx.send(Event::Input(event)).unwrap();
    } else {
        tx.send(Event::Tick).unwrap();
    }
}

impl EventServer for CrosstermEventServer {
    fn config(&self) -> EventConfig {
        self.config
    }

    fn listen(&mut self) {
        let event_tx = self.tx.clone();
        let tick_rate = self.config.tick_rate;

        let exit_condition = Arc::clone(&self.exit_condition);

        let join_handle = thread::spawn(move || loop {
            {
                if *exit_condition.lock().unwrap() == true {
                    break;
                }
            }
            crossterm_poll_for_event(&event_tx, tick_rate)
        });

        self.handle = Some(join_handle);
    }

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

impl Iterator for CrosstermEventServer {
    type Item = Event<crossterm::event::Event>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rx.recv().ok()
    }
}
