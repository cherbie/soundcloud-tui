use super::track_lookup::*;

#[cfg(test)]
mod tests {
    use super::stubs::*;
    use super::*;
    use mockall::predicate::*;

    #[tokio::test]
    #[ignore = "not implemented yet"]
    async fn test_track_lookup() {
        let mut mock_client = MockReqClient::new();
        mock_client
            .expect_fetch()
            .returning(|_| Box::pin(async { Ok(Default::default()) }));

        let track_lookup = TrackLookupFactory::new(std::sync::Arc::new(mock_client));
        let track = track_lookup
            .get_track("https://www.youtube.com/watch?v=1234")
            .await;

        assert!(track.is_ok(), "Expected track lookup to be Ok");
    }
}

#[cfg(test)]
mod stubs {
    pub use super::super::client::MockReqClient;
}
