// This is free and unencumbered software released into the public domain.

//! Real Windows raw command lines, independent of the invoking shell.

fn main() {
    #[cfg(windows)]
    windows::run();
    #[cfg(not(windows))]
    println!("raw Windows wildcard tests require native Windows");
}

#[cfg(windows)]
mod windows {
    use std::{
        env, ffi::OsString, fs, os::windows::process::CommandExt, path::Path, process::Command,
    };
    use temp_dir::TempDir;

    const CHILD: &str = "CLIENTELE_RAW_WILDCARD_CHILD";

    pub fn run() {
        if let Ok(mode) = env::var(CHILD) {
            let args = clientele::args_os().unwrap();
            assert_eq!(args.first(), env::args_os().next().as_ref());
            assert_eq!(args[1], "before");
            assert_eq!(args.last().unwrap(), "after");
            let mut values = args[2..args.len() - 1].to_vec();
            let expected: Vec<OsString> = match mode.as_str() {
                "matching" => {
                    // Enumeration order is not part of the wildcard contract.
                    values.sort();
                    vec!["match one.txt".into(), "match-two.txt".into()]
                }
                "question" => vec!["match-two.txt".into()],
                "unmatched" => vec!["absent*.txt".into()],
                "quoted" => vec!["match*.txt".into()],
                "mixed" => {
                    assert_eq!(values[0], "match*.txt");
                    assert_eq!(values[3], "absent*.txt");
                    values[1..3].sort();
                    vec![
                        "match*.txt".into(),
                        "match one.txt".into(),
                        "match-two.txt".into(),
                        "absent*.txt".into(),
                    ]
                }
                _ => panic!("unknown wildcard mode: {mode}"),
            };
            assert_eq!(values, expected, "{mode}");
            return;
        }

        let dir = TempDir::new().unwrap();
        for filename in ["match one.txt", "match-two.txt", "unrelated.bin"] {
            fs::write(dir.child(filename), "fixture").unwrap();
        }
        for (mode, raw) in [
            ("matching", "before match*.txt after"),
            ("question", "before match-?wo.txt after"),
            ("unmatched", "before absent*.txt after"),
            ("quoted", r#"before "match*.txt" after"#),
            (
                "mixed",
                r#"before "match*.txt" match*.txt absent*.txt after"#,
            ),
        ] {
            check(dir.path(), mode, raw);
        }
    }

    fn check(directory: &Path, mode: &str, raw: &str) {
        let output = Command::new(env::current_exe().unwrap())
            .current_dir(directory)
            .env(CHILD, mode)
            // Command::args would re-quote the input and fail to exercise the
            // distinction between raw unquoted and explicitly quoted patterns.
            .raw_arg(raw)
            .output()
            .unwrap();
        assert!(output.status.success(), "{mode}, {raw:?}: {output:?}");
    }
}
