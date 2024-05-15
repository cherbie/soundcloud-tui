use super::track::*;

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use bytes::Bytes;
    use rodio::Decoder;

    use super::*;

    #[test]
    fn test_track_sample_from_bytes() {
        let bytes = Bytes::from_static(b"test");
        let sample = TrackSample::from(bytes.clone());

        let derived_bytes: Bytes = sample.into();
        assert_eq!(bytes, derived_bytes);
    }

    #[test]
    fn test_track_sample_into_decoder_err() {
        let bytes = Bytes::from_static(b"test");
        let sample = TrackSample::from(bytes.clone());

        let conversion_outcome: Result<Decoder<Cursor<Bytes>>, _> = sample.try_into();
        assert!(conversion_outcome.is_err(), "Expected error result, got ok");
        assert_eq!(
            "Unrecognized format",
            conversion_outcome.err().unwrap().to_string()
        )
    }

    #[test]
    #[ignore = "The .ts file is invalid as rodio does not support non-audio files"]
    fn test_track_sample_into_decoder() {
        let file = std::fs::read("testdata/segment_12.ts");
        assert!(
            file.is_ok(),
            "Error reading sample ts file, {:?}",
            file.err()
        );

        let bytes = Bytes::from(file.unwrap());
        let sample = TrackSample::from(bytes);

        let conversion_outcome: Result<Decoder<Cursor<Bytes>>, _> = sample.try_into();
        assert!(
            conversion_outcome.is_ok(),
            "Expected ok result, got error {:?}",
            conversion_outcome.err()
        );
    }
}

mod stubs {}
