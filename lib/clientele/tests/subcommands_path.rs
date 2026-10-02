// This is free and unencumbered software released into the public domain.

//! Unix PATH-component and Windows missing-environment checks in isolated children.

fn main() {
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
            assert!(matches!(case.as_str(), "present" | "PATH" | "PATHEXT"));
            let dir = PathBuf::from(env::args_os().nth(1).expect("fixture directory"));
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
