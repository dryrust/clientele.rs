// This is free and unencumbered software released into the public domain.

//! Native directory resolution and UTF-8 conversion checks. Each environment
//! scenario runs in a child process; expected OS paths are passed as OS strings.

use clientele::paths;
use std::{
    env,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
    process::Command,
};
use temp_dir::TempDir;

const CHILD_MODE: &str = "CLIENTELE_NATIVE_PATHS_TEST_CHILD";

fn main() {
    if let Ok(case) = env::var(CHILD_MODE) {
        run_child(&case);
        return;
    }

    let dir = TempDir::new().expect("create native-path fixture directory");
    let values = [
        dir.child("native-λ"),
        #[cfg(unix)]
        PathBuf::from("relative-native"),
        #[cfg(any(unix, windows))]
        dir.path().join(non_utf8_component()),
    ];
    for path in &values {
        #[cfg(unix)]
        check("temp", Some(path), &[("TMPDIR", Some(path.as_os_str()))]);

        #[cfg(windows)]
        {
            let ignored = dir.child("lower-priority");
            check(
                "temp",
                Some(path),
                &[
                    ("TMP", Some(path.as_os_str())),
                    ("TEMP", Some(ignored.as_os_str())),
                    ("TMPDIR", Some(ignored.as_os_str())),
                ],
            );
            check("temp", Some(path), &[("TEMP", Some(path.as_os_str()))]);
            check(
                "temp",
                Some(path),
                &[("USERPROFILE", Some(path.as_os_str()))],
            );
        }

        #[cfg(all(unix, feature = "dirs"))]
        check("home", Some(path), &[("HOME", Some(path.as_os_str()))]);

        #[cfg(all(feature = "getenv", feature = "camino"))]
        check(
            "environment",
            Some(path),
            &[
                ("HOME", Some(path.as_os_str())),
                ("TMPDIR", Some(path.as_os_str())),
            ],
        );
    }

    check("temp-fallback", None, &[]);
    #[cfg(unix)]
    check(
        "temp",
        Some(Path::new("")),
        &[("TMPDIR", Some(OsStr::new("")))],
    );

    #[cfg(any(feature = "dirs", all(feature = "getenv", feature = "camino")))]
    for missing in [None, Some(OsStr::new(""))] {
        #[cfg(feature = "dirs")]
        check("home-fallback", None, &[("HOME", missing)]);

        #[cfg(all(feature = "getenv", feature = "camino"))]
        check(
            "environment",
            None,
            &[("HOME", missing), ("TMPDIR", missing)],
        );
    }

    #[cfg(all(windows, feature = "dirs"))]
    {
        let profile = paths::home_dir().expect("Windows user profile directory");
        let fake_home = dir.child("not-the-user-profile");
        check(
            "home",
            Some(&profile),
            &[
                ("HOME", Some(fake_home.as_os_str())),
                ("USERPROFILE", Some(fake_home.as_os_str())),
            ],
        );
    }
}

fn run_child(case: &str) {
    let expected = env::args_os().nth(1).map(PathBuf::from);
    match case {
        "temp" => assert_temp(expected.as_deref().expect("expected temporary path")),
        "temp-fallback" => {
            let path = paths::temp_dir();
            assert!(path.is_absolute(), "native temporary fallback: {path:?}");
            #[cfg(target_os = "linux")]
            assert_eq!(path, Path::new("/tmp"));
            assert_temp(&path);
            #[cfg(all(feature = "getenv", feature = "camino"))]
            assert!(paths::tmpdir().is_none());
        }
        #[cfg(feature = "dirs")]
        "home" => assert_home(expected.as_deref().expect("expected home path")),
        #[cfg(feature = "dirs")]
        "home-fallback" => {
            let path = paths::home_dir().expect("home from the current OS account/profile");
            assert!(path.is_absolute(), "native home fallback: {path:?}");
            assert_home(&path);
            #[cfg(all(feature = "getenv", feature = "camino"))]
            assert!(paths::home().is_none());
        }
        #[cfg(all(feature = "getenv", feature = "camino"))]
        "environment" => {
            let expected = expected
                .as_deref()
                .and_then(Path::to_str)
                .map(clientele::Utf8Path::new);
            assert_eq!(paths::home().as_deref(), expected);
            assert_eq!(paths::tmpdir().as_deref(), expected);
        }
        _ => panic!("unknown native-path test case: {case}"),
    }
}

fn assert_temp(expected: &Path) {
    assert_eq!(paths::temp_dir(), expected);
    #[cfg(feature = "camino")]
    assert_eq!(
        paths::temp_dir_utf8()
            .as_deref()
            .map(|path| path.as_std_path()),
        expected.to_str().map(|_| expected),
    );
}

#[cfg(feature = "dirs")]
fn assert_home(expected: &Path) {
    assert_eq!(paths::home_dir().as_deref(), Some(expected));
    #[cfg(feature = "camino")]
    assert_eq!(
        paths::home_dir_utf8()
            .as_deref()
            .map(|path| path.as_std_path()),
        expected.to_str().map(|_| expected),
    );
}

fn check(case: &str, expected: Option<&Path>, variables: &[(&str, Option<&OsStr>)]) {
    let mut command = Command::new(env::current_exe().unwrap());
    command.env(CHILD_MODE, case);
    for variable in [
        "HOME",
        "TMPDIR",
        "TMP",
        "TEMP",
        "USERPROFILE",
        "HOMEDRIVE",
        "HOMEPATH",
    ] {
        command.env_remove(variable);
    }
    for &(variable, value) in variables {
        if let Some(value) = value {
            command.env(variable, value);
        }
    }
    if let Some(expected) = expected {
        command.arg(expected);
    }
    let output = command.output().expect("run native-path child process");
    assert!(output.status.success(), "{case}, {variables:?}: {output:?}");
    assert!(output.stdout.is_empty(), "{case}: {output:?}");
    assert!(output.stderr.is_empty(), "{case}: {output:?}");
}

#[cfg(any(unix, windows))]
fn non_utf8_component() -> OsString {
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
