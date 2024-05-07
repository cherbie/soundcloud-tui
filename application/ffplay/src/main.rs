mod cli;
#[cfg(test)]
mod cli_test;

use anyhow::Result;
use player::TrackPlayer;
use rodio::Sink;

#[tokio::main]
async fn main() -> Result<()> {
    let cli_matches = cli::args_parser().get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let uri = cli_matches.get_one::<String>("input").unwrap();
    
    app(uri).await
}

async fn app(uri: &str) -> Result<()> {
    print!("App started with uri: {}", uri);
    let (_stream, stream_handle) = rodio::OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle).unwrap();

    let decoder = 
    stream_handle.play_once();

    sink.sleep_until_end();
    Ok(())
}
