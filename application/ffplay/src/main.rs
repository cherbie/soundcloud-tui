mod cli;
#[cfg(test)]
mod cli_test;

use anyhow::Result;

fn main() -> Result<()> {
    let cli_matches = cli::args_parser().get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let uri = cli_matches.get_one::<String>("input").unwrap();
    println!("Using input uri: {}", uri);

    // TODO: testing
    match cli_matches.get_one::<u8>("verbose") {
        Some(l) => println!("No verbose info: {}", l),
        _ => print!("Some other verbosity"),
    };

    Ok(())
}
