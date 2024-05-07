use crate::hls::*;

mod tests {
    use futures::StreamExt;

    use super::playlist::*;
    use super::stubs;

    #[tokio::test]
    async fn test_process_master_playlist_parsing_err() {
        // let mock_m3u8_file = Bytes::from(stubs::MASTER_PLAYLIST_FILE);
        let mut fetch_fn = stubs::MockResolveUriFactoryFn::new();
        fetch_fn.set_bytes("invalid m3u8 file");

        let outcome = fetch_playlist(String::from("https://example.com"), fetch_fn.call()).await;
        assert!(outcome.is_err(), "Expected error result, got ok",);
    }

    #[tokio::test]
    async fn test_process_master_playlist() {
        match m3u8_rs::parse_master_playlist(stubs::MASTER_PLAYLIST_FILE.as_bytes()) {
            Result::Ok((_, pl)) => {
                let mut fetch_fn = stubs::MockResolveUriFactoryFn::new();

                // return media playlist to avoid infinite loop
                fetch_fn.set_bytes(stubs::MEDIA_PLAYLIST_FILE);

                match parse_m3u8_master_playlist(pl, fetch_fn.call()).await {
                    Ok(_) => {
                        assert_eq!(fetch_fn.count(), 16, "fetch client was not called the correct number of times. Expected 16, got {}", fetch_fn.count());
                    }
                    Err(e) => {
                        assert!(false, "future has a result error: {}", e)
                    }
                }
            }
            Err(e) => {
                assert!(false, "unexpected testing error: {}", e)
            }
        }
    }

    #[tokio::test]
    async fn test_media_playlist() {
        match m3u8_rs::parse_media_playlist(stubs::MEDIA_PLAYLIST_FILE.as_bytes()) {
            Ok((_, pl)) => {
                let mock_fn = stubs::MockResolveUriFactoryFn::new();
                let stream = media_playlist_stream_factory(&pl, mock_fn.call());
                let stream_len = stream.len();
                assert!(stream_len > 0, "stream is empty");
                stream
                    .for_each(|f| async move { assert!(f.is_ok(), "future is not okay") })
                    .await;
            }
            Err(e) => {
                assert!(false, "unexpected testing error: {}", e)
            }
        }
    }

    #[tokio::test]
    async fn test_media_playlist_call_factory_fn() {
        match m3u8_rs::parse_media_playlist(stubs::MEDIA_PLAYLIST_FILE.as_bytes()) {
            Ok((_, pl)) => {
                let fetch_fn = stubs::MockResolveUriFactoryFn::new();
                let stream = media_playlist_stream_factory(&pl, fetch_fn.call());
                let n_items = stream.count().await;
                assert_eq!(
                    fetch_fn.count() as usize,
                    n_items,
                    "fetch was not called the correct number of times. Expected {}",
                    n_items
                );
            }
            Err(e) => {
                assert!(false, "unexpected testing error: {}", e)
            }
        }
    }
}

mod stubs {
    use super::*;
    use bytes::Bytes;
    use std::sync::{Arc, Mutex};

    pub struct MockResolveUriFactoryFn {
        count: Arc<Mutex<u32>>,
        bytes: Bytes,
    }

    impl MockResolveUriFactoryFn {
        pub fn new() -> Self {
            MockResolveUriFactoryFn {
                count: Arc::new(Mutex::new(0)),
                bytes: Bytes::new(),
            }
        }

        pub fn set_bytes(&mut self, str: &str) {
            self.bytes = Bytes::from(String::from(str));
        }

        pub fn call(&self) -> Box<FetchUriFactoryFn<Bytes>> {
            let count = self.count.clone();
            let bytes = self.bytes.clone();
            Box::new(move |_| {
                let count = count.clone();
                let bytes = bytes.clone();
                Box::pin(async move {
                    let count = count.clone();
                    let mut c = count.lock().unwrap();
                    *c += 1;
                    Ok(bytes)
                })
            })
        }

        pub fn count(&self) -> u32 {
            let count_gaurd = self.count.lock().unwrap();
            let count: u32 = count_gaurd.clone();

            count
        }
    }

    pub const MASTER_PLAYLIST_FILE: &str = r#"#EXTM3U
#EXT-X-VERSION:3
#EXT-X-STREAM-INF:PROGRAM-ID=9,BANDWIDTH=300000,
chunklist-b300000.m3u8
#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=600000
chunklist-b600000.m3u8
#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=850000
chunklist-b850000.m3u8
#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=1000000
chunklist-b1000000.m3u8
#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=1500000
chunklist-b1500000.m3u8"#;

    pub const MEDIA_PLAYLIST_FILE: &str = r#"#EXTM3U
#EXT-X-VERSION:4
#EXT-X-TARGETDURATION:3
#EXT-X-MEDIA-SEQUENCE:338559
#EXT-X-KEY:METHOD=AES-128,URI="https://secure.domain.com",IV=0xb059217aa2649ce170b734
#EXTINF:2.002,338559
20140311T113819-01-338559live.ts
#EXTINF:2.002,338560
20140311T113819-01-338560live.ts
#EXTINF:2.002,338561
20140311T113819-01-338561live.ts
#EXTINF:2.002,338562
20140311T113819-01-338562live.ts
#EXTINF:2.002,338563
20140311T113819-01-338563live.ts
#EXTINF:2.002,338564
20140311T113819-01-338564live.ts
#EXTINF:2.002,338565
20140311T113819-01-338565live.ts
#EXTINF:2.002,338566
20140311T113819-01-338566live.ts
#EXTINF:2.002,338567
20140311T113819-01-338567live.ts
#EXTINF:2.002,338568
20140311T113819-01-338568live.ts
#EXTINF:2.002,338569
20140311T113819-01-338569live.ts
#EXTINF:2.002,338570
20140311T113819-01-338570live.ts
#EXTINF:2.002,338571
20140311T113819-01-338571live.ts
#EXTINF:2.002,338572
20140311T113819-01-338572live.ts
#EXTINF:2.002,338573
20140311T113819-01-338573live.ts"#;
}
