// This is free and unencumbered software released into the public domain.

#![deny(unsafe_code)]
#![allow(unused)]

use clientele::{
    crates::clap::{self, error::ErrorKind, CommandFactory, FromArgMatches, Parser, Subcommand},
    StandardOptions, SysexitsError,
};
use std::{
    io::{self, Write},
    process::ExitCode,
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

/// Runs the CLI, reporting application failures with their sysexits status codes.
///
/// I/O errors retain their operation, filename, and source details for display;
/// conversion to a sysexits status happens only at this process boundary.
/// A broken stdout pipe is a quiet success. Other stdout failures use their
/// sysexits status; failed stderr diagnostics retain the original error status.
///
/// With the `color` feature, `--color` in the expanded arguments controls Clap's
/// help and error output, including missing-subcommand diagnostics.
/// With `tracing`, installs the process-wide stderr subscriber after informational
/// early exits and emits a debug event when running `config`. This standalone
/// entry point owns subscriber initialization; a preinstalled subscriber panics.
pub fn main() -> ExitCode {
    // Returning Result directly would turn every application error into status 1.
    match run() {
        Ok(status) => status,
        Err(error) => {
            // A secondary diagnostic failure must not hide the original status.
            let _ = writeln!(io::stderr().lock(), "Error: {error}");
            SysexitsError::from(&error).as_exit_code()
        }
    }
}

fn run() -> io::Result<ExitCode> {
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
    let matches = match command.try_get_matches_from_mut(args) {
        Ok(matches) => matches,
        Err(error) => return print_clap_error(error),
    };
    let options = match Options::from_arg_matches(&matches) {
        Ok(options) => options,
        Err(error) => return print_clap_error(error.format(&mut command)),
    };

    // Print the program version, if requested:
    if options.flags.version {
        write_stdout(format_args!("skeleton {}", env!("CARGO_PKG_VERSION")))?;
        return Ok(ExitCode::SUCCESS);
    }

    // Print the program license, if requested:
    if options.flags.license {
        write_stdout("This is free and unencumbered software released into the public domain.")?;
        return Ok(ExitCode::SUCCESS);
    }

    // This standalone application owns the global subscriber. Embedded callers
    // can use try_init_tracing_subscriber instead to handle an existing one.
    #[cfg(feature = "tracing")]
    clientele::tracing::init_tracing_subscriber(&options.flags);

    match options.command {
        Some(Command::Config {}) => {
            #[cfg(feature = "tracing")]
            tracing::debug!("Running config subcommand");
            write_stdout("This is the implementation of the `config` subcommand.")?;
            Ok(ExitCode::SUCCESS)
        }
        None => {
            let _ = command
                .error(ErrorKind::MissingSubcommand, "a subcommand is required")
                .print();
            Ok(SysexitsError::EX_USAGE.as_exit_code())
        }
    }
}

fn print_clap_error(error: clap::Error) -> io::Result<ExitCode> {
    if error.use_stderr() {
        // Preserve the parse error's status even when its diagnostic cannot be written.
        let _ = error.print();
    } else {
        // Help/version output follows the same pipe policy as application output.
        stdout_result(error.print())?;
    }
    Ok(ExitCode::from(error.exit_code() as u8))
}

fn write_stdout(message: impl std::fmt::Display) -> io::Result<()> {
    let mut output = io::stdout().lock();
    stdout_result(writeln!(output, "{message}").and_then(|()| output.flush()))
}

fn stdout_result(result: io::Result<()>) -> io::Result<()> {
    match result {
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        result => result,
    }
}
