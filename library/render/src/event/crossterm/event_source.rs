pub use lib::*;

mod lib {
    use crate::event::utils::*;
    use std::io::Result;
    use std::time;

    #[derive(Clone, Copy, Debug)]
    pub struct CrosstermEventSource;

    impl EventPoll for CrosstermEventSource {
        fn poll(&self, timeout: time::Duration) -> Result<bool> {
            crossterm::event::poll(timeout)
        }
    }

    impl EventRead for CrosstermEventSource {
        type Event = crossterm::event::Event;

        fn read(&self) -> Result<crossterm::event::Event> {
            crossterm::event::read()
        }
    }
}
