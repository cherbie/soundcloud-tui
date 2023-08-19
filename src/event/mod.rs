mod crossterm;
mod event_server;
mod key;

pub enum Event<I> {
    Input(I),
    Tick,
}

pub use self::crossterm::CrosstermEventServer;
pub use event_server::EventServer;
