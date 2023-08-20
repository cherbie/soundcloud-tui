use super::super::utils::*;
use std::time;

pub struct CrosstermEventUtils;

impl EventPoll for CrosstermEventUtils {
    fn poll(timeout: time::Duration) -> std::io::Result<bool> {
        crossterm::event::poll(timeout)
    }
}

impl EventRead for CrosstermEventUtils {
    type Event = crossterm::event::Event;
    fn read() -> std::io::Result<crossterm::event::Event> {
        crossterm::event::read()
    }
}
