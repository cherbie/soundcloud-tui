mod mock {
    use crate::event::utils::{EventPoll, EventRead};
    use crate::event::utils::{MockEventPoll, MockEventRead};
    use std::any::{Any, TypeId};
    use std::collections::HashMap;
    use std::time;

    struct MockInMemoryEventSource<T> {
        poll_should_panic: bool,
        poll_timeout: time::Duration,
        poll_expect: std::io::Result<bool>,
        read_should_panic: bool,
        read_expect: std::io::Result<T>,
    }

    #[derive(Eq, Debug, PartialEq, Hash)]
    enum MockStaticFnId {
        Poll,
        Read,
    }

    fn closure() -> &'static str {
        "123"
    }

    mod poll_panic {
        use super::MockInMemoryEventSource;
        use crate::event::utils::{EventPoll, MockEventPoll};
        use std::time;
        use unimock::*;

        impl<T> EventPoll for MockInMemoryEventSource<T> {
            fn poll(&self, timeout: time::Duration) -> std::io::Result<bool> {
                let clause = MockEventPoll::poll.each_call(matching!()).returns(Ok(true));
                let mock = Unimock::new(clause);
            }
        }
    }

    impl EventRead for InMemoryEventSource {
        type Event = char;
        fn read() -> std::io::Result<Self::Event> {
            Ok('c')
        }
    }
}

mod poll_for_event {
    use std::sync::mpsc;
    use std::time;

    use crate::event::{utils::poll_for_event, Event};
    use crate::tests::utils::Fixture;

    struct EventUtilsFixture<T> {
        tx: mpsc::Sender<Event<T>>,
        rx: mpsc::Receiver<Event<T>>,
        tick_rate: time::Duration,
    }

    impl<T> EventUtilsFixture<T> {
        fn new(tick_rate: time::Duration) -> Self {
            let (tx, rx) = mpsc::channel();
            EventUtilsFixture { tx, rx, tick_rate }
        }
    }

    impl<T> EventUtilsFixture<T> {
        fn setup(&mut self) {}
    }

    impl<T> Fixture for EventUtilsFixture<T> {
        fn setup(&mut self) {}

        fn before_each(&mut self) {}

        fn after_each(&mut self) {}

        fn cleanup(&mut self) {}
    }

    #[test]
    fn test_poll_event() {
        let mut fixture = EventUtilsFixture::<char>::new(time::Duration::from_millis(10));
        fixture.setup();

        fixture.before_each();

        poll_for_event(&fixture.tx, fixture.tick_rate);

        fixture.after_each();

        fixture.cleanup();
    }

    #[test]
    fn test_tick_event_on_timeout() {}

    #[test]
    fn test_closed_channel_panic() {}

    #[test]
    fn test_poll_panic() {}

    #[test]
    fn test_event_read_panic() {}
}
