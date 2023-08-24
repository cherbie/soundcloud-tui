pub mod crossterm;
mod event_server;
mod key;
mod utils;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Event<I> {
    Input(I),
    Tick,
}

pub use self::crossterm::CrosstermEventServer;
pub use event_server::EventServer;

#[cfg(test)]
mod tests;
