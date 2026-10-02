// This is free and unencumbered software released into the public domain.

//! Listing order and lookup precedence in isolated search paths.

use clientele::{Subcommand, SubcommandsProvider};
use std::{env, fs, path::Path, process::Command};
use temp_dir::TempDir;

const CHILD_MODE: &str = "CLIENTELE_SUBCOMMAND_ORDER_CHILD";

fn main() {
    if let Ok(winner) = env::var(CHILD_MODE) {
        let extension = if cfg!(windows) {
            if env::var("PATHEXT").unwrap().starts_with(".CMD") {
                ".cmd"
            } else {
                ".bat"
            }
        } else {
            ""
        };
        let expected = [
            ("alpha", "first"),
            ("shared", winner.as_str()),
            ("zulu", "second"),
        ]
        .map(|(name, directory)| Subcommand {
            name: name.to_string(),
            path: Path::new(directory).join(format!("clientele-{name}{extension}")),
        });
        let commands = SubcommandsProvider::collect("clientele-", 1).into_commands();
        assert_eq!(commands, expected);
        for command in commands {
            assert_eq!(
                SubcommandsProvider::find("clientele-", &command.name),
                Some(command)
            );
        }
        return;
    }

    let dir = TempDir::new().unwrap();
    for directory in ["first", "second"] {
        fs::create_dir(dir.child(directory)).unwrap();
    }
    // Deliberately create files in a different order from the expected listing.
    for (directory, name) in [
        ("second", "zulu"),
        ("first", "shared"),
        ("second", "shared"),
        ("first", "alpha"),
    ] {
        for extension in if cfg!(windows) {
            &[".bat", ".cmd"][..]
        } else {
            &[""][..]
        } {
            let path = dir
                .child(directory)
                .join(format!("clientele-{name}{extension}"));
            fs::write(&path, "fixture").unwrap();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
            }
        }
    }

    for directories in [["first", "second", "first"], ["second", "first", "second"]] {
        for extensions in [".CMD;.BAT", ".BAT;.CMD"] {
            let output = Command::new(env::current_exe().unwrap())
                .current_dir(dir.path())
                .env(CHILD_MODE, directories[0])
                .env("PATH", env::join_paths(directories).unwrap())
                .env("PATHEXT", extensions)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{directories:?}, {extensions}: {output:?}"
            );
        }
    }
}
