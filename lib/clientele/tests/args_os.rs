// This is free and unencumbered software released into the public domain.

//! Argument expansion contracts run in deadline-controlled child processes.

use std::{
    env,
    ffi::OsString,
    fs,
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant},
};
use temp_dir::TempDir;

const CHILD_MODE: &str = "CLIENTELE_ARGS_TEST_CHILD";

fn main() {
    if let Ok(mode) = env::var(CHILD_MODE) {
        match mode.as_str() {
            "cycle" => {
                let error = clientele::args_os().unwrap_err();
                assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
                assert!(error.to_string().contains("recursive argument file"));
            }
            "repeat" => {
                let mut expected: Vec<OsString> = [
                    "before",
                    "\"quoted value\"",
                    "leaf",
                    "\"quoted value\"",
                    "leaf",
                    "after",
                ]
                .map(OsString::from)
                .into();
                #[cfg(any(unix, windows))]
                expected.push(non_unicode());
                #[cfg(any(target_os = "linux", windows))]
                expected.push("native file".into());
                assert_eq!(&clientele::args_os().unwrap()[1..], expected);
            }
            "literal" => assert_eq!(
                clientele::args_os().unwrap(),
                env::args_os().collect::<Vec<_>>()
            ),
            "lines" => assert_eq!(
                &clientele::args_os().unwrap()[1..],
                [
                    "before",
                    "--",
                    " first line ",
                    "\"quoted value\"",
                    "",
                    "λ\tvalue",
                    "after"
                ]
                .map(OsString::from)
            ),
            "empty" => assert_eq!(
                &clientele::args_os().unwrap()[1..],
                ["before", "after"].map(OsString::from)
            ),
            "invalid" | "missing" => {
                let error = clientele::args_os().unwrap_err();
                let expected = if mode == "invalid" {
                    std::io::ErrorKind::InvalidData
                } else {
                    std::io::ErrorKind::NotFound
                };
                assert_eq!(error.kind(), expected);
                assert!(error.to_string().contains(&format!("{mode}.args")));
            }
            "argv0" => {
                let expected = if cfg!(feature = "argfile") {
                    vec!["replacement program", "--leading-value", "tail"]
                } else {
                    vec!["@program.args", "tail"]
                };
                assert_eq!(
                    clientele::args_os().unwrap(),
                    expected.into_iter().map(OsString::from).collect::<Vec<_>>()
                );
            }
            _ => panic!("unknown child mode: {mode}"),
        }
        return;
    }

    let dir = TempDir::new().unwrap();
    fs::write(dir.child("self.args"), "before\n@self.args\n").unwrap();
    fs::write(dir.child("invalid.args"), [b'v', b'\n', 0xff]).unwrap();
    fs::write(
        dir.child("program.args"),
        "replacement program\n--leading-value\n",
    )
    .unwrap();
    #[cfg(unix)]
    check(dir.path(), "argv0", &["tail".into()]);
    if !cfg!(feature = "argfile") {
        let mut args = [
            "before",
            "@self.args",
            "--",
            "@missing.args",
            "@invalid.args",
        ]
        .map(OsString::from)
        .to_vec();
        #[cfg(any(unix, windows))]
        args.push(non_unicode());
        check(dir.path(), "literal", &args);
        return;
    }

    fs::write(
        dir.child("lines.args"),
        " first line \r\n\"quoted value\"\r\n\r\nλ\tvalue\n",
    )
    .unwrap();
    fs::write(dir.child("empty.args"), "").unwrap();
    check(
        dir.path(),
        "lines",
        &["before", "--", "@lines.args", "after"].map(OsString::from),
    );
    check(
        dir.path(),
        "empty",
        &["before", "@empty.args", "after"].map(OsString::from),
    );
    for mode in ["invalid", "missing"] {
        // The result must be an error even after successful earlier expansion,
        // with no partial argument vector or output leaked to the caller.
        check(
            dir.path(),
            mode,
            &[
                "before".into(),
                "@lines.args".into(),
                format!("@{mode}.args").into(),
            ],
        );
    }

    fs::create_dir(dir.child("nested")).unwrap();
    fs::write(dir.child("first.args"), "@second.args\n").unwrap();
    fs::write(dir.child("second.args"), "@first.args\n").unwrap();
    fs::write(dir.child("alias.args"), "@nested/../alias.args\n").unwrap();
    for file in ["self.args", "first.args", "alias.args"] {
        check(dir.path(), "cycle", &[format!("@{file}").into()]);
    }

    let absolute = std::path::absolute(dir.child("absolute.args")).unwrap();
    fs::write(&absolute, format!("@{}\n", absolute.to_str().unwrap())).unwrap();
    check(dir.path(), "cycle", &["@absolute.args".into()]);

    #[cfg(unix)]
    {
        fs::write(dir.child("symlink.args"), "@link.args\n").unwrap();
        std::os::unix::fs::symlink("symlink.args", dir.child("link.args")).unwrap();
        check(dir.path(), "cycle", &["@symlink.args".into()]);
    }

    fs::write(
        dir.child("nested/branch.args"),
        "\"quoted value\"\n@leaf.args\n",
    )
    .unwrap();
    fs::write(dir.child("leaf.args"), "leaf\n").unwrap();
    let mut args: Vec<OsString> = [
        "before",
        "@nested/branch.args",
        "@nested/branch.args",
        "after",
    ]
    .map(OsString::from)
    .into();
    #[cfg(any(unix, windows))]
    args.push(non_unicode());
    // macOS filesystems reject invalid UTF-8 filenames, but native arguments
    // above can still contain arbitrary OS strings on that platform.
    #[cfg(any(target_os = "linux", windows))]
    {
        fs::write(dir.path().join(non_unicode()), "native file\n").unwrap();
        let mut argfile = OsString::from("@");
        argfile.push(non_unicode());
        args.push(argfile);
    }
    check(dir.path(), "repeat", &args);
}

fn check(dir: &Path, mode: &str, args: &[OsString]) {
    let mut command = Command::new(env::current_exe().unwrap());
    command
        .current_dir(dir)
        .env(CHILD_MODE, mode)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(unix)]
    if mode == "argv0" {
        use std::os::unix::process::CommandExt;
        command.arg0("@program.args");
    }
    let mut child = command.spawn().unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    while child.try_wait().unwrap().is_none() {
        if Instant::now() >= deadline {
            child.kill().unwrap();
            let output = child.wait_with_output().unwrap();
            panic!("argument expansion timed out: {args:?}: {output:?}");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{mode}, {args:?}: {output:?}");
    assert!(output.stdout.is_empty(), "{output:?}");
    assert!(output.stderr.is_empty(), "{output:?}");
}

#[cfg(any(unix, windows))]
fn non_unicode() -> OsString {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        OsString::from_vec(vec![b'n', 0xff])
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStringExt;
        OsString::from_wide(&[b'n' as u16, 0xd800])
    }
}
