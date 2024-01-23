#[cfg(test)]
mod test {
    use m3u8_rs;
    use tokio;

    const MASTER_PLAYLIST_FILE: &str = r#"#EXTM3U
        #EXT-X-VERSION:4
        #EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID="Audio1",NAME="mp4a.40.2_96K_Spanish",LANGUAGE="spa",DEFAULT=YES,AUTOSELECT=YES,URI="A1.m3u8"
        A1.m3u8
        #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=395000,CODECS="avc1.4d001f,mp4a.40.2",AUDIO="Audio1",RESOLUTION=320x240
        01.m3u8
        #EXT-X-I-FRAME-STREAM-INF:BANDWIDTH=394000,CODECS="avc1.4d001f,mp4a.40.2",URI="01_iframe_index.m3u8"
        01_iframe_index.m3u8
        #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=963000,CODECS="avc1.4d001f,mp4a.40.2",AUDIO="Audio1",RESOLUTION=448x336
        02.m3u8
        #EXT-X-I-FRAME-STREAM-INF:BANDWIDTH=962000,CODECS="avc1.4d001f,mp4a.40.2",URI="02_iframe_index.m3u8"
        02_iframe_index.m3u8
        #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=1695000,CODECS="avc1.4d001f,mp4a.40.2",AUDIO="Audio1",RESOLUTION=640x480
        03.m3u8
        #EXT-X-I-FRAME-STREAM-INF:BANDWIDTH=1694000,CODECS="avc1.4d001f,mp4a.40.2",URI="03_iframe_index.m3u8"
        03_iframe_index.m3u8
        "#;

    const MEDIA_PLAYLIST_FILE: &str = r#"#EXTM3U
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

    #[tokio::test]
    async fn test_parse_m3u8_playlist_file() {
        let outcome = m3u8_rs::parse_playlist(MASTER_PLAYLIST_FILE.as_bytes());

        assert!(outcome.is_ok(), "outcome is not ok.");

        match outcome {
            Result::Ok((_, m3u8_rs::Playlist::MasterPlaylist(pl))) => {
                println!("Master playlist:\n{:?}", pl)
            }
            Result::Ok((_, m3u8_rs::Playlist::MediaPlaylist(pl))) => {
                println!("Media playlist:\n{:?}", pl)
            }
            _ => {}
        }
    }
}
