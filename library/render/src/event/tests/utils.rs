mod poll_for_event {
    use std::sync::mpsc;
    use std::time;
    use unimock::*;

    use crate::event::{
        utils::{poll_for_event, MockEventPoll, MockEventRead},
        Event,
    };

    #[test]
    fn test_poll_event() {
        let mock_event: char = 'c';
        let mock_event_source = Unimock::new((
            MockEventPoll::poll
                .some_call(matching!(_))
                .returns(Ok(true))
                .once(),
            MockEventRead::read
                .some_call(matching!(_))
                .returns(Ok(mock_event))
                .once(),
        ));
        let (mock_sender, mock_receiver) = mpsc::channel();
        let mock_tick_rate = time::Duration::from_millis(10);

        poll_for_event(&mock_event_source, &mock_sender, mock_tick_rate);

        let received_event = mock_receiver.try_recv();
        assert!(received_event.is_ok(), "channel receiver error");
        assert_eq!(
            received_event,
            Ok(Event::Input(mock_event)),
            "incorrect Event::Input(char) received"
        );
    }

    #[test]
    fn test_tick_event_on_poll_timeout() {
        let mock_event_source = Unimock::new(
            MockEventPoll::poll
                .some_call(matching!(_))
                .returns(Ok(false))
                .once(),
        );
        let (mock_sender, mock_receiver) = mpsc::channel();
        let mock_timeout = time::Duration::from_millis(10);

        poll_for_event(&mock_event_source, &mock_sender, mock_timeout);

        let received_event = mock_receiver.recv_timeout(mock_timeout);
        assert!(received_event.is_ok(), "channel receiver error");
        assert_eq!(received_event, Ok(Event::Tick), "Event::Tick not received");
    }

    #[test]
    #[should_panic]
    fn test_closed_channel_panic() {
        let mock_event_source = Unimock::new((
            MockEventPoll::poll
                .some_call(matching!(_))
                .returns(Ok(true))
                .once(),
            MockEventRead::read
                .some_call(matching!(_))
                .returns(Ok('c'))
                .once(),
        ));
        let (mock_sender, mock_receiver) = mpsc::channel();
        let mock_tick_rate = time::Duration::from_millis(10);

        drop(mock_receiver);
        poll_for_event(&mock_event_source, &mock_sender, mock_tick_rate);
    }

    #[test]
    #[should_panic(expected = "EventPoll has panicked")]
    fn test_poll_panic() {
        let mock_event_source = Unimock::new(
            MockEventPoll::poll
                .some_call(matching!(_))
                .panics("EventPoll has panicked")
                .once(),
        );
        let (mock_sender, _) = mpsc::channel();
        let mock_tick_rate = time::Duration::from_millis(10);

        poll_for_event(&mock_event_source, &mock_sender, mock_tick_rate);
    }

    #[test]
    #[should_panic(expected = "EventRead has panicked")]
    fn test_event_read_panic() {
        let mock_event_source = Unimock::new((
            MockEventPoll::poll
                .some_call(matching!(_))
                .returns(Ok(true))
                .once(),
            MockEventRead::read
                .some_call(matching!(_))
                .panics("EventRead has panicked")
                .once(),
        ));
        let (mock_sender, _) = mpsc::channel();
        let mock_tick_rate = time::Duration::from_millis(10);

        poll_for_event(&mock_event_source, &mock_sender, mock_tick_rate);
    }
}
