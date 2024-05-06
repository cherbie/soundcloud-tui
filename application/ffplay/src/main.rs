mod cli;
#[cfg(test)]
mod cli_test;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli_matches = cli::args_parser().get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let uri = cli_matches.get_one::<String>("input").unwrap();

    app(uri).await
}

async fn app(uri: &str) -> Result<()> {
    print!("App started with uri: {}", uri);
    Ok(())
}
