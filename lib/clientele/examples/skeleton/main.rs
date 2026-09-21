// This is free and unencumbered software released into the public domain.

#![deny(unsafe_code)]
#![allow(unused)]

use clientele::{
    crates::clap::{error::ErrorKind, CommandFactory, Parser, Subcommand},
    StandardOptions, SysexitsError,
};
use std::process::ExitCode;

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

/// Runs the CLI, reporting application failures with their sysexits status codes.
pub fn main() -> ExitCode {
    // Returning Result directly would turn every application error into status 1.
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            error.as_exit_code()
        }
    }
}

fn run() -> Result<(), SysexitsError> {
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

    match options.command {
        Some(Command::Config {}) => {
            println!("This is the implementation of the `config` subcommand.");
            Ok(())
        }
        None => {
            Options::command()
                .error(ErrorKind::MissingSubcommand, "a subcommand is required")
                .print()?;
            clientele::exit(SysexitsError::EX_USAGE)
        }
    }
}
