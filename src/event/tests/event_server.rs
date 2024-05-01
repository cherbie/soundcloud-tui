mod event_server_core {
    use unimock::*;

    use crate::event::{
        utils::{MockEventPoll, MockEventRead},
        Event, EventServer, EventServerCore,
    };

    #[test]
    fn test_should_receive_events() {
        let mut event_server = EventServerCore::<char>::default();
        let mock_event_source = Unimock::new((
            MockEventPoll::poll
                .each_call(matching!())
                .answers(&|_,_| Ok(true)),
            MockEventRead::read
                .each_call(matching!())
                .answers(&|_| Ok('c')),
        ));

        event_server.listen(mock_event_source.clone());
        assert_eq!(
            event_server.next(),
            Some(Event::Input('c')),
            "more than one event received."
        );
        event_server.stop();
    }
}
