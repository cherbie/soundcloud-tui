use crate::cli::*;

#[test]
fn test_cli_uri() {
    let exp_uri = "http://example.com";
    let cli_matches = args_parser().get_matches_from(vec!["ffplay", "-i", exp_uri]);
    let uri = cli_matches.get_one::<String>("input").unwrap();
    assert_eq!(uri, exp_uri);
}

#[test]
fn test_cli_uri_empty_failure() {
    let parse_err = args_parser()
        .try_get_matches_from(vec!["ffplay", "-i", ""])
        .unwrap_err();
    assert_eq!(parse_err.kind(), clap::error::ErrorKind::InvalidValue);
    assert!(parse_err
        .to_string()
        .contains("value is required for '--input <URI>' but none was supplied"));
}

#[test]
fn test_cli_verbosity_outofrange() {
    let parse_err = args_parser()
        .try_get_matches_from(vec!["ffplay", "-i", "https://example.com", "-v", "4"])
        .unwrap_err();
    assert_eq!(parse_err.kind(), clap::error::ErrorKind::ValueValidation);
    assert!(
        parse_err
            .to_string()
            .contains("invalid value '4' for '-v [<LEVEL>]'"),
        "received error message: {}",
        parse_err.to_string(),
    );
}
