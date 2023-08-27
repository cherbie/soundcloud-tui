pub mod crossterm;
mod event_server;
mod key;
mod utils;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Event<I> {
    Input(I),
    Tick,
}

pub use self::crossterm::*;
pub use event_server::*;

#[cfg(test)]
mod tests;
