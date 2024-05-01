use crate::player::hls::*;

mod tests {
    use bytes::Bytes;
    use futures::StreamExt;
    use m3u8_rs::Playlist;
    use unimock::mock;

    use super::stubs;
    use crate::player::hls::*;

    #[tokio::test]
    #[ignore = "not implemented yet"]
    async fn test_parse_m3u8_parse_err() {
        let mock_m3u8_file = Bytes::from(stubs::MASTER_PLAYLIST_FILE);
        let mut mock_fn = stubs::MockResolveUriFactoryFn::new();
        mock_fn.set_bytes("invalid m3u8 file");
        let outcome = parse_m3u8(mock_m3u8_file, mock_fn.call()).await;
        assert!(outcome.is_ok(), "should parse m3u8 file");
        // TODO: implement what to do with the streaming results
        // FIXME: FutureStream not awaited on
        assert_eq!(0, mock_fn.count(), "get_m3u8_factory was not called");
    }

    #[tokio::test]
    async fn test_parse_m3u8_master_playlist_file() {
        match m3u8_rs::parse_playlist(stubs::MASTER_PLAYLIST_FILE.as_bytes()) {
            Result::Ok((_, m3u8_rs::Playlist::MasterPlaylist(pl))) => {
                let mut mock_fn = stubs::MockResolveUriFactoryFn::new();

                // return media playlist to avoid infinite loop
                // FIXME: recursive call to master playlists results in stack overflow
                mock_fn.set_bytes(stubs::MEDIA_PLAYLIST_FILE);

                match parse_m3u8_master_playlist(pl, mock_fn.call()).await {
                    Ok(_) => {
                        assert_eq!(mock_fn.count(), 1, "get_m3u8_factory was not called");
                    }
                    Err(e) => {
                        assert!(false, "future has a result error: {}", e)
                    }
                }
            }
            Result::Ok(_) => {
                assert!(false, "Expected master playlist. Received media playlist")
            }
            Err(e) => {
                assert!(false, "error parsing m3u8 master file: {}", e)
            }
        }
    }

    #[tokio::test]
    async fn test_new_media_playlist_stream_parsing_ok() {
        match m3u8_rs::parse_playlist_res(stubs::MEDIA_PLAYLIST_FILE.as_bytes()) {
            Result::Ok(m3u8_rs::Playlist::MediaPlaylist(pl)) => {
                let mock_fn = stubs::MockResolveUriFactoryFn::new();
                let stream = new_media_playlist_stream(pl, mock_fn.call());
                let stream_len = stream.len();
                assert!(stream_len > 0, "stream is empty");
                stream
                    .for_each(|f| async move { assert!(f.is_ok(), "future is not okay") })
                    .await;
            }
            _ => {
                assert!(false, "error parsing m3u8 media file")
            }
        }
    }

    #[tokio::test]
    async fn test_new_media_playlist_stream_call_factory_fn() {
        match m3u8_rs::parse_playlist_res(stubs::MEDIA_PLAYLIST_FILE.as_bytes()) {
            Result::Ok(m3u8_rs::Playlist::MediaPlaylist(pl)) => {
                let mock_fn = stubs::MockResolveUriFactoryFn::new();
                let stream = new_media_playlist_stream(pl, mock_fn.call());
                let n_items = stream.count().await;
                assert_eq!(
                    mock_fn.count() as usize,
                    n_items,
                    "get_m3u8_factory was not called the correct number of times"
                );
            }
            _ => {
                assert!(false, "error parsing m3u8 media file")
            }
        }
    }
}

mod stubs {
    use super::*;
    use bytes::Bytes;
    use std::{cell::Cell, sync::*};

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

        pub fn call(&self) -> Box<ResolveUriFactoryFn<Bytes>> {
            let count = self.count.clone();
            let bytes = self.bytes.clone();
            Box::new(move |url: &str| {
                let url_str = String::from(url);
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
