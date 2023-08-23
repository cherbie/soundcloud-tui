pub use api::*;
pub use lib::poll_for_event;

mod api {
    use std::time;
    #[cfg(test)]
    use unimock::unimock;

    #[cfg_attr(test, unimock(api=MockEventPoll))]
    pub trait EventPoll {
        fn poll(&self, timeout: time::Duration) -> std::io::Result<bool>;
    }

    #[cfg_attr(test, unimock(api=MockEventRead, type Event = char;))]
    pub trait EventRead {
        type Event;
        fn read(&self) -> std::io::Result<Self::Event>;
    }
}

mod lib {
    use super::super::Event;
    use super::api;
    use std::sync;
    use std::time;

    pub fn poll_for_event<U, E>(
        event_source: &U,
        tx: &sync::mpsc::Sender<Event<E>>,
        tick_rate: time::Duration,
    ) where
        U: api::EventPoll + api::EventRead<Event = E>,
    {
        if event_source.poll(tick_rate).unwrap() {
            let event = event_source.read().unwrap();
            tx.send(Event::Input(event)).unwrap();
        } else {
            tx.send(Event::Tick).unwrap();
        }
    }
}
