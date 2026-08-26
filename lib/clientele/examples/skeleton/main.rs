// This is free and unencumbered software released into the public domain.

#![deny(unsafe_code)]
#![allow(unused)]

use clientele::{
    crates::clap::{Parser, Subcommand},
    StandardOptions, SysexitsError,
};

/// Skeleton command-line interface (CLI)
#[derive(Debug, Parser)]
#[command(name = "Skeleton")]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Show the current configuration
    Config {},
}

pub fn main() -> Result<(), SysexitsError> {
    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Print the program version, if requested:
    if options.flags.version {
        println!("skeleton {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // Print the program license, if requested:
    if options.flags.license {
        println!("This is free and unencumbered software released into the public domain.");
        return Ok(());
    }

    match options.command.unwrap() {
        Command::Config {} => {
            println!("This is the implementation of the `config` subcommand.");
            Ok(())
        }
    }
}
