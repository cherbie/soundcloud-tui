use crate::event::Key;
use futures::StreamExt;
use std::io::ErrorKind;
use std::pin::pin;
use std::time::Duration;

#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use mockall::*;

#[derive(Debug, Clone, Copy)]
pub struct EventConfig {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for EventConfig {
    fn default() -> EventConfig {
        EventConfig {
            exit_key: Key::Ctrl('c'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Event {
    Input(crossterm::event::Event),
    Tick,
}

#[cfg_attr(test, automock)]
pub trait EventStream<T: StreamExt<Item = Event> + std::marker::Unpin> {
    fn config(&self) -> EventConfig;
    fn stream(&mut self) -> T;
}

pub async fn read_event<S: StreamExt<Item = Event> + std::marker::Unpin, E: EventStream<S>>(
    mut reader: E,
) -> Result<Event, crossterm::ErrorKind> {
    let mut stream = reader.stream();

    match tokio::time::timeout(reader.config().tick_rate, pin!(stream.next())).await {
        Err(_) => Ok(Event::Tick),
        Ok(Some(event)) => Ok(event),
        Ok(None) => Err(crossterm::ErrorKind::new(
            ErrorKind::UnexpectedEof,
            "Unexpected stream end",
        )),
    }
}
