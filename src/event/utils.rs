use super::Event;
use std::sync;
use std::time;

pub trait EventPoll {
    fn poll(timeout: time::Duration) -> std::io::Result<bool>;
}

pub trait EventRead {
    type Event;
    fn read() -> std::io::Result<Self::Event>;
}

pub fn poll_for_event<U, E>(tx: &sync::mpsc::Sender<Event<E>>, tick_rate: time::Duration)
where
    U: EventPoll + EventRead<Event = E>,
{
    if U::poll(tick_rate).unwrap() {
        let event = U::read().unwrap();
        tx.send(Event::Input(event)).unwrap();
    } else {
        tx.send(Event::Tick).unwrap();
    }
}
