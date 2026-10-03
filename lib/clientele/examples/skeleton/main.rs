// This is free and unencumbered software released into the public domain.

#![deny(unsafe_code)]
#![allow(unused)]

use clientele::{
    crates::clap::{error::ErrorKind, CommandFactory, FromArgMatches, Parser, Subcommand},
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
///
/// With the `color` feature, `--color` in the expanded arguments controls Clap's
/// help and error output, including missing-subcommand diagnostics.
/// With `tracing`, installs the process-wide stderr subscriber after informational
/// early exits and emits a debug event when running `config`. This standalone
/// entry point owns subscriber initialization; a preinstalled subscriber panics.
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

    // Configure color before parsing, since Clap may print help or errors:
    let mut command = Options::command();
    #[cfg(feature = "color")]
    {
        command = command.color(clientele::color_choice(&args));
    }

    // Parse command-line options, retaining the configured command for errors:
    let matches = command
        .try_get_matches_from_mut(args)
        .unwrap_or_else(|error| error.exit());
    let options = Options::from_arg_matches(&matches)
        .unwrap_or_else(|error| error.format(&mut command).exit());

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

    // This standalone application owns the global subscriber. Embedded callers
    // can use try_init_tracing_subscriber instead to handle an existing one.
    #[cfg(feature = "tracing")]
    clientele::tracing::init_tracing_subscriber(&options.flags);

    match options.command {
        Some(Command::Config {}) => {
            #[cfg(feature = "tracing")]
            tracing::debug!("Running config subcommand");
            println!("This is the implementation of the `config` subcommand.");
            Ok(())
        }
        None => {
            command
                .error(ErrorKind::MissingSubcommand, "a subcommand is required")
                .print()?;
            clientele::exit(SysexitsError::EX_USAGE)
        }
    }
}
