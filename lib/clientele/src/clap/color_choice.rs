// This is free and unencumbered software released into the public domain.

use std::ffi::OsString;

/// Scans for `--color <when>` / `--color=<when>` ahead of parsing, so that
/// the choice can be fed back into Clap for its own help/usage output.
pub fn color_choice(args: &[OsString]) -> clap::ColorChoice {
    let mut choice = clap::ColorChoice::Auto;
    let mut args = args.iter().filter_map(|arg| arg.to_str());
    while let Some(arg) = args.next() {
        let value = if arg == "--color" {
            args.next()
        } else {
            arg.strip_prefix("--color=")
        };
        if let Some(value) = value {
            choice = value.parse().unwrap_or(choice);
        }
    }
    choice
}
