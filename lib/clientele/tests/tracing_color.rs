// This is free and unencumbered software released into the public domain.

//! Checks captured tracing output with each global subscriber and environment
//! isolated in a child process. This harness-free target accepts ordinary CLI
//! arguments in child mode, following the skeleton CLI fixture.

use clientele::{crates::clap::Parser, tracing::init_tracing_subscriber, StandardOptions};
use std::{env, process::Command};

const CHILD_MODE: &str = "CLIENTELE_TRACING_TEST_CHILD";
const PLAIN: &[&str] = &["error event", "warn event", "info event", "debug event"];
const DEBUG: &[&str] = &[
    "ERROR clientele_test: error event",
    "WARN clientele_test: warn event",
    "INFO clientele_test: info event",
    "DEBUG clientele_test: debug event",
    "TRACE clientele_test: trace event",
];

#[derive(Parser)]
struct Options {
    #[command(flatten)]
    flags: StandardOptions,
}

fn main() {
    if env::var_os(CHILD_MODE).is_some() {
        let options = Options::parse();
        init_tracing_subscriber(&options.flags);
        tracing::error!(target: "clientele_test", "error event");
        tracing::warn!(target: "clientele_test", "warn event");
        tracing::info!(target: "clientele_test", "info event");
        tracing::debug!(target: "clientele_test", "debug event");
        tracing::trace!(target: "clientele_test", "trace event");
        return;
    }

    // Captured stderr is not a terminal, so automatic color must be disabled.
    // These cases also retain plain/debug formatting and level filtering.
    for (args, expected) in [
        (&[][..], &PLAIN[..1]),
        (&["-v"][..], &PLAIN[..2]),
        (&["-vv"][..], &PLAIN[..3]),
        (&["-vvv"][..], PLAIN),
        (&["-vvvv"][..], PLAIN),
        (&["--debug"][..], DEBUG),
        (&["--debug", "-v"][..], DEBUG),
    ] {
        check(args, None, expected, false);
    }

    for no_color in [None, Some(""), Some("1")] {
        check(&["--debug"], no_color, DEBUG, false);

        #[cfg(feature = "color")]
        {
            check(&["--debug", "--color=auto"], no_color, DEBUG, false);
            check(&["--debug", "--color=never"], no_color, DEBUG, false);
            check(&["--debug", "--color=always"], no_color, DEBUG, true);

            // Plain message-only events have no level or target to color.
            check(&["-vvv", "--color=always"], no_color, PLAIN, false);
        }
    }
}

fn check(args: &[&str], no_color: Option<&str>, expected: &[&str], ansi: bool) {
    let mut command = Command::new(env::current_exe().unwrap());
    command
        .args(args)
        .env(CHILD_MODE, "1")
        .env("TERM", "xterm-256color")
        .env("CLICOLOR", "1")
        .env("CLICOLOR_FORCE", "1")
        .env("FORCE_COLOR", "1");
    if let Some(value) = no_color {
        command.env("NO_COLOR", value);
    } else {
        command.env_remove("NO_COLOR");
    }
    let output = command.output().expect("run tracing child process");
    assert!(output.status.success(), "{args:?}: {output:?}");
    assert!(output.stdout.is_empty(), "{args:?}: {output:?}");
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 tracing output");
    assert_eq!(
        stderr.contains("\x1b["),
        ansi,
        "{args:?}, NO_COLOR={no_color:?}: unexpected ANSI color presence in {stderr:?}"
    );
    let plain = clientele::strip_ansi(&stderr);
    let lines: Vec<_> = plain.lines().map(str::trim).collect();
    assert_eq!(
        lines, expected,
        "{args:?}, NO_COLOR={no_color:?}: {stderr:?}"
    );
}
