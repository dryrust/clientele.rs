// This is free and unencumbered software released into the public domain.

//! Runs each global-subscriber initialization scenario in a fresh process.

use clientele::{
    crates::clap::Parser,
    tracing::{init_tracing_subscriber, try_init_tracing_subscriber},
    StandardOptions,
};
use std::{env, process::Command};

const CHILD_MODE: &str = "CLIENTELE_TRACING_INIT_TEST_CHILD";

#[derive(Parser)]
struct Options {
    #[command(flatten)]
    flags: StandardOptions,
}

fn main() {
    if let Some(mode) = env::var_os(CHILD_MODE) {
        let options = Options::parse_from(["tracing-init-test"]);
        match mode.to_str().expect("UTF-8 child mode") {
            "success" => {
                try_init_tracing_subscriber(&options.flags).expect("initialize tracing");
                let error = try_init_tracing_subscriber(&options.flags)
                    .expect_err("repeated initialization must fail");
                assert!(!error.to_string().is_empty());
                tracing::error!("initialized event");
                tracing::warn!("filtered event");
            }
            "preinstalled" | "legacy-preinstalled" => {
                let subscriber = tracing_subscriber::fmt()
                    .with_writer(std::io::stderr)
                    .with_max_level(tracing::Level::TRACE)
                    .without_time()
                    .with_target(false)
                    .with_level(false)
                    .with_ansi(false)
                    .finish();
                tracing::subscriber::set_global_default(subscriber)
                    .expect("install original subscriber");

                if mode == "legacy-preinstalled" {
                    init_tracing_subscriber(&options.flags);
                } else {
                    let error = try_init_tracing_subscriber(&options.flags)
                        .expect_err("preinstalled subscriber must cause an error");
                    assert!(!error.to_string().is_empty());
                    // TRACE would be filtered out by the requested default options.
                    tracing::trace!("original subscriber event");
                }
            }
            _ => panic!("unknown child mode: {mode:?}"),
        }
        return;
    }

    for (mode, expected) in [
        ("success", "initialized event"),
        ("preinstalled", "original subscriber event"),
        ("legacy-preinstalled", "Unable to install global subscriber"),
    ] {
        let output = Command::new(env::current_exe().unwrap())
            .env(CHILD_MODE, mode)
            .env("NO_COLOR", "1")
            .env("RUST_BACKTRACE", "0")
            .output()
            .expect("run tracing initialization child");
        assert!(output.stdout.is_empty(), "{mode}: {output:?}");
        let stderr = String::from_utf8(output.stderr).expect("UTF-8 tracing output");
        if mode == "legacy-preinstalled" {
            assert!(!output.status.success(), "initializer did not panic");
            assert!(stderr.contains("panicked at"), "{mode}: {stderr:?}");
            assert!(stderr.contains(expected), "{mode}: {stderr:?}");
        } else {
            assert!(output.status.success(), "{mode}: {stderr:?}");
            assert_eq!(stderr.trim(), expected, "{mode}");
        }
    }
}
