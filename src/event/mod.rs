mod crossterm;
mod event_server;
mod key;
mod utils;

pub enum Event<I> {
    Input(I),
    Tick,
}

pub use self::crossterm::CrosstermEventServer;
pub use self::crossterm::CrosstermEventUtils;
pub use event_server::EventServer;
