use super::client;
use super::hls;

pub trait TrackLookupService {
    fn get_track(&self, uri: &str) -> hls::PinFuture<hls::track::Track>;
}

pub struct TrackLookupFactory;

impl TrackLookupFactory {
    pub fn new<C>(client: std::sync::Arc<C>) -> impl TrackLookupService 
    where C: client::ReqClient + Send + Sync + 'static
    {
        TrackLookupCore::new(client)
    }
}

struct TrackLookupCore {
    client: std::sync::Arc<dyn client::ReqClient + Send + Sync>,
}

impl TrackLookupCore {
    pub fn new(client: std::sync::Arc<dyn client::ReqClient + Send + Sync>) -> Self {
        Self { client }
    }
}

impl TrackLookupService for TrackLookupCore {
    fn get_track(&self, uri: &str) -> hls::PinFuture<hls::track::Track> {
        let client = self.client.clone();
        let uri_str = uri.to_string();

        Box::pin(async move {
            let track_outcome =
                hls::playlist::fetch_playlist(uri_str, Box::new(move |uri| client.fetch(uri)))
                    .await;

            Ok(track_outcome?)
        })
    }
}
