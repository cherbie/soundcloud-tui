use bytes::Bytes;
use rodio;

pub trait DeviceService {
    fn play(&self) -> Result<(), rodio::PlayError>;
    fn pause(&self) -> Result<(), rodio::PlayError>;
    fn clear(&self) -> Result<(), rodio::PlayError>;
    fn enqueue(
        &self,
        source: Box<dyn rodio::Source<Item = f32> + Send>,
    ) -> Result<(), rodio::PlayError>;
    fn skip(&self) -> Result<(), rodio::PlayError>;
    fn is_empty(&self) -> bool;
}

pub struct DeviceFactory;

impl DeviceFactory {
    pub fn new(sink: rodio::Sink) -> impl DeviceService {
        DeviceCore::from_sink(sink)
    }
}
struct DeviceCore {
    sink: rodio::Sink,
    _stream: Option<(rodio::OutputStream, rodio::OutputStreamHandle)>,
}

impl DeviceCore {
    fn from_sink(sink: rodio::Sink) -> Self {
        Self {
            sink,
            _stream: None,
        }
    }
}

impl DeviceService for DeviceCore {
    fn clear(&self) -> Result<(), rodio::PlayError> {
        self.sink.clear();

        Ok(())
    }

    fn pause(&self) -> Result<(), rodio::PlayError> {
        self.sink.pause();

        Ok(())
    }

    fn play(&self) -> Result<(), rodio::PlayError> {
        if self.is_empty() {
            return Err(rodio::PlayError::DecoderError(
                rodio::decoder::DecoderError::NoStreams,
            ));
        }
        self.sink.play();

        Ok(())
    }

    fn enqueue(
        &self,
        source: Box<dyn rodio::Source<Item = f32> + Send>,
    ) -> Result<(), rodio::PlayError> {
        self.sink.append(source.into_iter());
        Ok(())
    }

    fn skip(&self) -> Result<(), rodio::PlayError> {
        self.sink.skip_one();

        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.sink.empty()
    }
}
