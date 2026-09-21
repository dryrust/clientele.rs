// This is free and unencumbered software released into the public domain.

//! Subprocess regression checks for the skeleton's actual entry point.
//!
//! A harness-free target lets child invocations accept ordinary CLI arguments
//! and return the example's exit status. Including the example ensures it is
//! rebuilt with this target's features, without invoking Cargo inside tests.

use std::{
    env,
    process::{Command, ExitCode, Termination},
};
use temp_dir::TempDir;

#[path = "../examples/skeleton/main.rs"]
mod skeleton;

const CHILD_MODE: &str = "CLIENTELE_SKELETON_TEST_CHILD";

fn main() -> ExitCode {
    if env::var_os(CHILD_MODE).is_some() {
        return skeleton::main().report();
    }

    let dir = TempDir::new().expect("create isolated CLI directory");
    // Stop dotenv from searching ancestor directories for the developer's .env.
    std::fs::write(dir.child(".env"), "").unwrap();

    for args in [&[][..], &["--unknown-option"], &["unknown-command"]] {
        check(&dir, args, 2, "", "Usage:");
    }
    for args in [&["--debug"][..], &["-vv"], &["--debug", "--verbose"]] {
        check(&dir, args, 64, "", "Usage:");
    }
    #[cfg(feature = "color")]
    check(&dir, &["--color", "never"], 64, "", "Usage:");

    for args in [&["--help"][..], &["-h"]] {
        check(&dir, args, 0, "Usage:", "");
    }
    let version = format!("skeleton {}", env!("CARGO_PKG_VERSION"));
    for args in [&["--version"][..], &["-V"], &["--debug", "--version"]] {
        check(&dir, args, 0, &version, "");
    }
    for args in [&["--license"][..], &["--verbose", "--license"]] {
        check(&dir, args, 0, "released into the public domain", "");
    }
    for args in [
        &["config"][..],
        &["--debug", "config"],
        &["config", "--verbose"],
    ] {
        check(
            &dir,
            args,
            0,
            "implementation of the `config` subcommand",
            "",
        );
    }

    #[cfg(feature = "argfile")]
    {
        check(&dir, &["@missing-args.txt"], 66, "", "Error: EX_NOINPUT");
        std::fs::write(dir.child("args.txt"), "config\n").unwrap();
        check(
            &dir,
            &["@args.txt"],
            0,
            "implementation of the `config` subcommand",
            "",
        );
        std::fs::write(dir.child("args.txt"), "--debug\n").unwrap();
        check(&dir, &["@args.txt"], 64, "", "Usage:");
    }

    ExitCode::SUCCESS
}

fn check(dir: &TempDir, args: &[&str], code: i32, stdout: &str, stderr: &str) {
    let output = Command::new(env::current_exe().unwrap())
        .args(args)
        .current_dir(dir.path())
        .env(CHILD_MODE, "1")
        .env("NO_COLOR", "1")
        .env_remove("CLICOLOR_FORCE")
        .env_remove("FORCE_COLOR")
        .output()
        .expect("run skeleton child process");
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
}
