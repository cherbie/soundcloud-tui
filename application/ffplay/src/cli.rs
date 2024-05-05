use clap::{arg, value_parser, Command};

pub fn args_parser() -> Command {
    Command::new("ffplay")
        .version("0.1")
        .author("Clayton Herbst <cherbie@github.com>")
        .about("A simple audio player using the underlying HLS client library.")
        .arg(
            arg!(input: -i --input <URI> "Sets the HLS source URI")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .required(true),
        )
        .arg(
            arg!(verbose: -v [LEVEL] "Sets the level of verbosity")
                .value_parser(value_parser!(u8).range(0..=3))
                .default_value("0"),
        )
}
