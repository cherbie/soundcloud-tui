use crossterm::event::{KeyCode, KeyModifiers};
use futures::future::Future;
use std::pin::pin;

use super::input_reader::{read_event, Event, EventConfig, MockEventStream};

#[tokio::test]
async fn test_stream_error() {
    let mut mock_event_reader: MockEventStream<futures::stream::Empty<Event>> =
        MockEventStream::new();
    mock_event_reader
        .expect_config()
        .return_const(EventConfig::default());
    mock_event_reader
        .expect_stream()
        .return_const(futures::stream::empty::<Event>());

    let event = read_event(mock_event_reader).await;
    assert!(event.is_err(), "should error on stream EOF");
}

#[tokio::test]
#[ignore]
async fn test_single_keyboard_event() {
    // TODO
}

#[tokio::test]
async fn test_tick_event() {
    let mut mock_event_reader: MockEventStream<futures::stream::Pending<Event>> =
        MockEventStream::new();
    mock_event_reader
        .expect_config()
        .return_const(EventConfig::default());
    mock_event_reader
        .expect_stream()
        .return_const(futures::stream::pending::<Event>());

    let event = read_event(mock_event_reader).await;
    assert!(!event.is_err(), "should error on stream EOF");
    assert_eq!(event.unwrap(), Event::Tick);
}
