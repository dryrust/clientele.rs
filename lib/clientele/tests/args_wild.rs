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
                "argfile-glob" | "argfile-mixed" => {
                    let mut expected: Vec<OsString> = if cfg!(feature = "argfile") {
                        // Neither direct nor nested argfile patterns get a second
                        // wildcard pass, even though both match fixture files.
                        vec!["match*.txt".into(), "match-?wo.txt".into()]
                    } else {
                        vec!["@only.args".into()]
                    };
                    if mode == "argfile-mixed" {
                        let start = values.len() - 2;
                        values[start..].sort();
                        expected.extend(["match one.txt".into(), "match-two.txt".into()]);
                    }
                    expected
                }
                "argfile-space" => {
                    if cfg!(feature = "argfile") {
                        vec!["match*.txt".into()]
                    } else {
                        vec!["@space file.args".into()]
                    }
                }
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

        // The raw @*.args glob must expand to @only.args before argfile loading
        // opens only.args. Reading the globbed file itself would produce a wrong
        // sentinel; attempting argfile expansion first would open a literal '*'.
        fs::write(dir.child("@only.args"), "wrong file\n").unwrap();
        fs::write(dir.child("only.args"), "match*.txt\n@nested.args\n").unwrap();
        fs::write(dir.child("nested.args"), "match-?wo.txt\n").unwrap();
        fs::write(dir.child("space file.args"), "match*.txt\n").unwrap();
        for (mode, raw) in [
            ("argfile-glob", "before @*.args after"),
            ("argfile-mixed", "before @*.args match*.txt after"),
            ("argfile-space", r#"before "@space file.args" after"#),
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
