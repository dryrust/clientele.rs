// This is free and unencumbered software released into the public domain.

use clap::ColorChoice;
use std::ffi::OsString;

pub trait ColorChoiceExt {
    /// Converts this color choice to a boolean value, where `true` means color
    /// should be enabled and `false` means color should be disabled.
    ///
    /// This is used to determine whether color should be enabled for the current
    /// terminal session.
    fn to_bool(&self) -> bool {
        use std::{
            env,
            io::{stdout, IsTerminal},
        };
        match self.as_color_choice() {
            ColorChoice::Always => true,
            ColorChoice::Never => false,
            ColorChoice::Auto => {
                stdout().is_terminal()
                    && !env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty())
            }
        }
    }

    fn as_color_choice(&self) -> &ColorChoice;
}

impl ColorChoiceExt for ColorChoice {
    fn as_color_choice(&self) -> &ColorChoice {
        self
    }
}

/// Scans for `--color <when>` / `--color=<when>` ahead of parsing, so that
/// the choice can be fed back into Clap for its own help/usage output.
pub fn color_choice(args: &[OsString]) -> ColorChoice {
    let mut choice = ColorChoice::Auto;
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
