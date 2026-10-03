// This is free and unencumbered software released into the public domain.

//! Subprocess regression checks for the skeleton's actual entry point.
//!
//! A harness-free target lets child invocations accept ordinary CLI arguments
//! and return the example's exit status. Including the example ensures it is
//! rebuilt with this target's features, without invoking Cargo inside tests.

use std::{
    env,
    process::{Command, ExitCode, Output, Termination},
};
use temp_dir::TempDir;

#[path = "../examples/skeleton/main.rs"]
mod skeleton;

const CHILD_MODE: &str = "CLIENTELE_SKELETON_TEST_CHILD";
const NO_COLOR_ENV: &[(&str, Option<&str>)] = &[
    ("NO_COLOR", Some("1")),
    ("CLICOLOR_FORCE", None),
    ("FORCE_COLOR", None),
];
#[cfg(feature = "color")]
const FORCE_COLOR_ENV: &[(&str, Option<&str>)] = &[
    ("NO_COLOR", None),
    ("CLICOLOR_FORCE", Some("1")),
    ("FORCE_COLOR", Some("1")),
];

fn main() -> ExitCode {
    if env::var_os(CHILD_MODE).is_some() {
        return skeleton::main().report();
    }

    let dir = TempDir::new().expect("create isolated CLI directory");
    // Stop dotenv from searching ancestor directories for the developer's .env.
    std::fs::write(dir.child(".env"), "").unwrap();

    for args in [&[][..], &["--unknown-option"], &["unknown-command"]] {
        check(&dir, args, 2, "", "Usage:", NO_COLOR_ENV);
    }
    for args in [&["--debug"][..], &["-vv"], &["--debug", "--verbose"]] {
        check(&dir, args, 64, "", "Usage:", NO_COLOR_ENV);
    }

    for args in [&["--help"][..], &["-h"]] {
        check(&dir, args, 0, "Usage:", "", NO_COLOR_ENV);
    }
    let version = format!("skeleton {}", env!("CARGO_PKG_VERSION"));
    for args in [&["--version"][..], &["-V"], &["--debug", "--version"]] {
        check(&dir, args, 0, &version, "", NO_COLOR_ENV);
    }
    for args in [&["--license"][..], &["--verbose", "--license"]] {
        check(
            &dir,
            args,
            0,
            "released into the public domain",
            "",
            NO_COLOR_ENV,
        );
    }
    for args in [
        &["config"][..],
        &["config", "--verbose"],
        &["-vv", "config"],
    ] {
        check(
            &dir,
            args,
            0,
            "implementation of the `config` subcommand",
            "",
            NO_COLOR_ENV,
        );
    }

    for args in [&["--debug", "config"][..], &["-vvv", "config"]] {
        let output = check(
            &dir,
            args,
            0,
            "implementation of the `config` subcommand",
            if cfg!(feature = "tracing") {
                "Running config subcommand"
            } else {
                ""
            },
            NO_COLOR_ENV,
        );
        assert!(!output.stderr.contains(&0x1b), "{output:?}");
        #[cfg(feature = "tracing")]
        assert_eq!(
            String::from_utf8_lossy(&output.stderr).contains("DEBUG"),
            args[0] == "--debug",
            "debug mode should include event metadata: {output:?}"
        );
    }

    #[cfg(all(feature = "color", feature = "tracing"))]
    for (color, environment, ansi) in [
        ("always", NO_COLOR_ENV, true),
        ("never", FORCE_COLOR_ENV, false),
        ("auto", NO_COLOR_ENV, false),
    ] {
        let output = check(
            &dir,
            &["--debug", "--color", color, "config"],
            0,
            "implementation of the `config` subcommand",
            "Running config subcommand",
            environment,
        );
        assert_eq!(output.stderr.contains(&0x1b), ansi, "{output:?}");
        assert!(!output.stdout.contains(&0x1b), "{output:?}");
    }

    #[cfg(feature = "color")]
    for (color_args, color_env, ansi) in [
        (&["--color=always"][..], NO_COLOR_ENV, true),
        (&["--color", "always"][..], NO_COLOR_ENV, true),
        (&["--color=never"][..], FORCE_COLOR_ENV, false),
        (&["--color", "never"][..], FORCE_COLOR_ENV, false),
    ] {
        for (suffix, code, stdout, stderr) in [
            (&["--help"][..], 0, "Usage:", ""),
            (&["--unknown-option"][..], 2, "", "unexpected argument"),
            (&[][..], 64, "", "a subcommand is required"),
            (&["config", "--help"][..], 0, "Usage:", ""),
            (
                &["config", "--unknown-option"][..],
                2,
                "",
                "unexpected argument",
            ),
        ] {
            let args = [color_args, suffix].concat();
            let output = check(&dir, &args, code, stdout, stderr, color_env);
            assert_color(&args, &output, ansi);

            #[cfg(feature = "argfile")]
            {
                std::fs::write(dir.child("color-args.txt"), args.join("\n")).unwrap();
                let output = check(&dir, &["@color-args.txt"], code, stdout, stderr, color_env);
                assert_color(&args, &output, ansi);
            }
        }
    }

    #[cfg(feature = "argfile")]
    {
        std::fs::write(dir.child("invalid-args.txt"), [0xff]).unwrap();
        for file in ["missing-args.txt", "invalid-args.txt"] {
            let source = std::fs::read_to_string(dir.child(file)).unwrap_err();
            let output = check(
                &dir,
                &[&format!("@{file}")],
                clientele::SysexitsError::from(&source) as i32,
                "",
                "Error: argument file",
                NO_COLOR_ENV,
            );
            let diagnostic = String::from_utf8(output.stderr).unwrap();
            assert!(diagnostic.contains(file), "{diagnostic}");
            // Compare with the native error rather than hard-coding OS wording.
            assert!(diagnostic.contains(&source.to_string()), "{diagnostic}");
        }
        std::fs::write(dir.child("args.txt"), "config\n").unwrap();
        check(
            &dir,
            &["@args.txt"],
            0,
            "implementation of the `config` subcommand",
            "",
            NO_COLOR_ENV,
        );
        std::fs::write(dir.child("args.txt"), "--debug\n").unwrap();
        check(&dir, &["@args.txt"], 64, "", "Usage:", NO_COLOR_ENV);
    }

    ExitCode::SUCCESS
}

fn check(
    dir: &TempDir,
    args: &[&str],
    code: i32,
    stdout: &str,
    stderr: &str,
    color_env: &[(&str, Option<&str>)],
) -> Output {
    let mut command = Command::new(env::current_exe().unwrap());
    command
        .args(args)
        .current_dir(dir.path())
        .env(CHILD_MODE, "1")
        .env("TERM", "xterm-256color")
        .env_remove("CLICOLOR");
    for &(key, value) in color_env {
        if let Some(value) = value {
            command.env(key, value);
        } else {
            command.env_remove(key);
        }
    }
    let output = command.output().expect("run skeleton child process");
    assert_eq!(output.status.code(), Some(code), "{args:?}: {output:?}");
    for (actual, expected) in [(&output.stdout, stdout), (&output.stderr, stderr)] {
        let actual = String::from_utf8_lossy(actual);
        if expected.is_empty() {
            assert!(actual.is_empty(), "{args:?}: unexpected output {actual:?}");
        } else {
            assert!(
                actual.contains(expected),
                "{args:?}: expected {expected:?} in {actual:?}"
            );
        }
    }
    output
}

#[cfg(feature = "color")]
fn assert_color(args: &[&str], output: &Output, ansi: bool) {
    for actual in [&output.stdout, &output.stderr] {
        assert_eq!(
            actual.windows(2).any(|bytes| bytes == b"\x1b["),
            ansi && !actual.is_empty(),
            "{args:?}: unexpected ANSI color presence in {output:?}"
        );
    }
}
