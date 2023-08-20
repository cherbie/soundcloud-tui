use crate::utils;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct EventConfig {
    pub tick_rate: Duration,
}

impl Default for EventConfig {
    fn default() -> EventConfig {
        EventConfig {
            tick_rate: Duration::from_millis(250),
        }
    }
}

pub trait EventServer {
    fn config(&self) -> EventConfig;
    fn listen<U, T>(&mut self)
    where
        U: utils::threads::Spawn,
        T: super::utils::EventPoll + super::utils::EventRead<Event = crossterm::event::Event>;
    fn stop(&mut self);
}
