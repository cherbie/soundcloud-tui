use super::device::*;

#[test]
fn test_device_play_pause() {
    let (sink, _stream) = rodio::Sink::new_idle();
    let device = DeviceFactory::new(sink);
    let enqueue_outcome = device.enqueue(Box::new(rodio::source::Empty::new()));
    assert!(
        enqueue_outcome.is_ok(),
        "Expected sound to be enqueued successfully"
    );
    assert!(device.play().is_ok(), "Expected play to be Ok");
    assert!(device.pause().is_ok(), "Expected pause to be Ok");
}

#[test]
fn test_device_skip_ok() {
    let (sink, _stream) = rodio::Sink::new_idle();
    let device = DeviceFactory::new(sink);
    assert!(
        device
            .enqueue(Box::new(rodio::source::Empty::new()))
            .is_ok(),
        "Expected sound to be enqueued successfully"
    );
    assert!(
        device
            .enqueue(Box::new(rodio::source::Empty::new()))
            .is_ok(),
        "Expected sound to be enqueued successfully"
    );

    assert!(device.skip().is_ok(), "Expected skip to be Ok");
}
