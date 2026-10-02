// This is free and unencumbered software released into the public domain.

//! Unix PATH-component regression checks, each in an isolated child process.

fn main() {
    #[cfg(unix)]
    unix::run();
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
