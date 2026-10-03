// This is free and unencumbered software released into the public domain.

//! Unix PATH-component and Windows missing-environment checks in isolated children.

#[cfg(any(unix, windows))]
#[path = "support/subcommands_native.rs"]
mod subcommands_native;

fn main() {
    #[cfg(any(unix, windows))]
    if std::env::var_os("CLIENTELE_PATH_COMPONENT_TEST_CHILD").is_none()
        && std::env::var_os("CLIENTELE_WINDOWS_PATH_TEST_CHILD").is_none()
        && subcommands_native::run()
    {
        return;
    }
    #[cfg(unix)]
    unix::run();
    #[cfg(windows)]
    windows::run();
}

#[cfg(windows)]
mod windows {
    use clientele::{Subcommand, SubcommandsProvider};
    use std::{env, ffi::OsStr, fs, path::PathBuf, process::Command};
    use temp_dir::TempDir;

    const CHILD_MODE: &str = "CLIENTELE_WINDOWS_PATH_TEST_CHILD";
    const PATH_EXTENSIONS: &str = ".BAT;.CMD";
    const FILES: &[(&str, &str)] = &[("hello", "bat"), ("hola", "cmd")];

    /// Checks each missing search variable independently, plus a populated control.
    pub fn run() {
        if let Ok(case) = env::var(CHILD_MODE) {
            let dir = PathBuf::from(env::args_os().nth(1).expect("fixture directory"));
            if case == "refresh" {
                check_refresh(&dir);
                return;
            }
            if case.starts_with("malformed-") {
                let expected = (case == "malformed-valid").then_some(Subcommand {
                    name: "hello".into(),
                    path: dir.join("clientele-hello.bat"),
                });
                assert_eq!(SubcommandsProvider::find("clientele-", "hello"), expected);
                assert_eq!(
                    SubcommandsProvider::collect("clientele-", 1).into_commands(),
                    expected.into_iter().collect::<Vec<_>>()
                );
                // An invalid-only list still permits an explicit filename lookup.
                assert_eq!(
                    SubcommandsProvider::find("clientele-", "hello.bat"),
                    Some(Subcommand {
                        name: "hello".into(),
                        path: dir.join("clientele-hello.bat"),
                    })
                );
                return;
            }
            assert!(matches!(case.as_str(), "present" | "PATH" | "PATHEXT"));
            // Verify the requested absence, including that the other variable is set.
            for (variable, value) in [
                ("PATH", dir.as_os_str()),
                ("PATHEXT", OsStr::new(PATH_EXTENSIONS)),
            ] {
                let expected = (case != variable).then(|| value.to_os_string());
                assert_eq!(env::var_os(variable), expected, "{case}: {variable}");
            }

            let mut expected_listing = Vec::new();
            for &(name, extension) in FILES {
                let path = dir.join(format!("clientele-{name}.{extension}"));
                assert!(path.is_file(), "missing fixture: {path:?}");
                let expected = (case == "present").then_some(Subcommand {
                    name: name.to_string(),
                    path,
                });
                for query in [name.to_string(), format!("{name}.{extension}")] {
                    assert_eq!(
                        SubcommandsProvider::find("clientele-", &query),
                        expected,
                        "{case}: lookup {query:?}",
                    );
                }
                expected_listing.extend(expected);
            }
            assert_eq!(
                SubcommandsProvider::collect("clientele-", usize::MAX).into_commands(),
                expected_listing,
                "{case}: listing",
            );
            return;
        }

        let dir = TempDir::new().expect("create Windows discovery fixtures");
        let fixture_dir = std::path::absolute(dir.path()).unwrap();
        for &(name, extension) in FILES {
            fs::write(
                fixture_dir.join(format!("clientele-{name}.{extension}")),
                "@echo off\r\nexit /b 0\r\n",
            )
            .unwrap();
        }

        for case in ["present", "PATH", "PATHEXT"] {
            let mut command = Command::new(env::current_exe().unwrap());
            command
                .arg(&fixture_dir)
                // A current-directory fixture also detects accidental PATH fallbacks.
                .current_dir(&fixture_dir)
                .env(CHILD_MODE, case)
                .env("PATH", &fixture_dir)
                .env("PATHEXT", PATH_EXTENSIONS);
            if case != "present" {
                command.env_remove(case);
            }
            let output = command
                .output()
                .expect("run Windows search-environment child");
            assert!(output.status.success(), "{case}: {output:?}");
        }

        for malformed in [".bad/name", ".bad\\name", ".bad/name;.bad\\name"] {
            for (case, extensions) in [
                ("malformed-only", malformed.to_string()),
                ("malformed-valid", format!("{malformed};.BAT")),
            ] {
                let output = Command::new(env::current_exe().unwrap())
                    .arg(&fixture_dir)
                    .current_dir(&fixture_dir)
                    .env(CHILD_MODE, case)
                    .env("PATH", &fixture_dir)
                    .env("PATHEXT", &extensions)
                    .output()
                    .unwrap();
                assert!(output.status.success(), "{extensions:?}: {output:?}");
            }
        }

        for directory in ["first", "second"] {
            fs::create_dir(fixture_dir.join(directory)).unwrap();
            for extension in ["bat", "cmd"] {
                fs::write(
                    fixture_dir
                        .join(directory)
                        .join(format!("clientele-hello.{extension}")),
                    "fixture",
                )
                .unwrap();
            }
        }
        let output = Command::new(env::current_exe().unwrap())
            .arg(&fixture_dir)
            .env(CHILD_MODE, "refresh")
            .env(
                "PATH",
                env::join_paths([
                    fixture_dir.join("first"),
                    fixture_dir.join("second"),
                    fixture_dir.join("first"),
                ])
                .unwrap(),
            )
            .env("PATHEXT", ".BAT;.CMD")
            .output()
            .unwrap();
        assert!(output.status.success(), "refresh: {output:?}");
    }

    // This harness-free child is single-threaded; mutations never reach the
    // parent or another test. Exercise public calls repeatedly in one process.
    fn check_refresh(dir: &std::path::Path) {
        let check = |path: Option<PathBuf>| {
            let expected = path.map(|path| Subcommand {
                name: "hello".into(),
                path,
            });
            let listing = SubcommandsProvider::collect("clientele-", 1);
            assert_eq!(listing.commands(), expected.as_slice());
            assert_eq!(SubcommandsProvider::find("clientele-", "hello"), expected);
            listing
        };
        let first_bat = dir.join("first/clientele-hello.bat");
        let snapshot = check(Some(first_bat.clone()));
        env::set_var("PATHEXT", ".CMD;.BAT");
        check(Some(dir.join("first/clientele-hello.cmd")));
        env::set_var(
            "PATH",
            env::join_paths([dir.join("second"), dir.join("first"), dir.join("second")]).unwrap(),
        );
        let second_cmd = dir.join("second/clientele-hello.cmd");
        let second_bat = dir.join("second/clientele-hello.bat");
        check(Some(second_cmd.clone()));
        fs::remove_file(second_cmd).unwrap();
        check(Some(second_bat.clone()));
        fs::remove_file(second_bat).unwrap();
        check(Some(dir.join("first/clientele-hello.cmd")));

        env::set_var("PATHEXT", ".bad/name");
        check(None);
        assert_eq!(
            SubcommandsProvider::find("clientele-", "hello.bat")
                .unwrap()
                .path,
            first_bat
        );
        env::remove_var("PATHEXT");
        check(None);
        assert!(SubcommandsProvider::find("clientele-", "hello.bat").is_none());

        let late = dir.join("late");
        env::set_var("PATHEXT", ".BAT");
        env::set_var("PATH", &late);
        check(None);
        fs::create_dir(&late).unwrap();
        let late_bat = late.join("clientele-hello.bat");
        fs::write(&late_bat, "new fixture").unwrap();
        check(Some(late_bat));
        env::remove_var("PATH");
        check(None);
        assert_eq!(snapshot.commands()[0].path, first_bat);
    }
}

#[cfg(unix)]
mod unix {
    use clientele::{Subcommand, SubcommandsProvider};
    use std::{env, fs, os::unix::fs::PermissionsExt, path::PathBuf, process::Command};
    use temp_dir::TempDir;

    const CHILD_MODE: &str = "CLIENTELE_PATH_COMPONENT_TEST_CHILD";
    const EXECUTABLE: &str = "clientele-local";

    pub fn run() {
        if let Ok(mode) = env::var(CHILD_MODE) {
            let expected = if mode == "found" {
                vec![Subcommand {
                    name: "local".to_string(),
                    path: PathBuf::from(EXECUTABLE),
                }]
            } else {
                assert_eq!(mode, "absent");
                vec![]
            };
            assert_eq!(
                SubcommandsProvider::collect("clientele-", 1).into_commands(),
                expected,
                "listing with PATH={:?}",
                env::var_os("PATH"),
            );
            assert_eq!(
                SubcommandsProvider::find("clientele-", "local"),
                expected.into_iter().next(),
                "lookup with PATH={:?}",
                env::var_os("PATH"),
            );
            return;
        }

        let dir = TempDir::new().expect("create isolated working directory");
        fs::create_dir(dir.child("first")).unwrap();
        fs::create_dir(dir.child("last")).unwrap();
        let executable = dir.child(EXECUTABLE);
        fs::write(&executable, "#!/bin/sh\nexit 0\n").unwrap();
        fs::set_permissions(&executable, fs::Permissions::from_mode(0o755)).unwrap();

        for (path, found) in [
            (Some(""), true),
            (Some(":first"), true),
            (Some("first:"), true),
            (Some("first::last"), true),
            (Some("first:last"), false),
            (None, false),
        ] {
            let mut command = Command::new(env::current_exe().unwrap());
            command
                .current_dir(dir.path())
                .env(CHILD_MODE, if found { "found" } else { "absent" });
            if let Some(path) = path {
                command.env("PATH", path);
            } else {
                command.env_remove("PATH");
            }
            let output = command.output().expect("run PATH-component child");
            assert!(output.status.success(), "PATH={path:?}: {output:?}");
        }
    }
}
