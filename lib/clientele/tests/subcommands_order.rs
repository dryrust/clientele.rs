// This is free and unencumbered software released into the public domain.

//! Listing order and lookup precedence in isolated search paths.

use clientele::{Subcommand, SubcommandsProvider};
use std::{env, fs, path::Path, process::Command};
use temp_dir::TempDir;

const CHILD_MODE: &str = "CLIENTELE_SUBCOMMAND_ORDER_CHILD";

fn main() {
    #[cfg(windows)]
    if env::var_os(CHILD_MODE).is_none() && windows_collisions() {
        return;
    }

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

/// Returns true after checking a collision child; parents continue other cases.
#[cfg(windows)]
fn windows_collisions() -> bool {
    const CHILD: &str = "CLIENTELE_DOTTED_COLLISION_CHILD";
    if env::var_os(CHILD).is_some() {
        let extensions = env::var("PATHEXT").unwrap();
        let suffix = if extensions.starts_with(".CMD") {
            "cmd"
        } else {
            "bat"
        };
        let mut expected = Vec::new();
        if extensions.contains(".V1") {
            expected.push(Subcommand {
                name: "report".into(),
                path: Path::new("exact").join("clientele-report.v1"),
            });
        }
        expected.extend([
            Subcommand {
                name: "report.v1".into(),
                path: Path::new("stem").join(format!("clientele-report.v1.{suffix}")),
            },
            Subcommand {
                name: "task".into(),
                path: Path::new("exact").join("clientele-task.bat"),
            },
            Subcommand {
                name: "task.bat".into(),
                path: Path::new("stem").join("clientele-task.bat.bat"),
            },
        ]);
        assert_eq!(
            SubcommandsProvider::collect("clientele-", 1).into_commands(),
            expected
        );
        for command in expected {
            assert_eq!(
                SubcommandsProvider::find("clientele-", &command.name),
                Some(command)
            );
        }
        // Explicit filenames still work when no matching logical stem exists.
        assert_eq!(
            SubcommandsProvider::find("clientele-", "report.v1.bat"),
            Some(Subcommand {
                name: "report.v1".into(),
                path: Path::new("stem").join("clientele-report.v1.bat"),
            })
        );
        return true;
    }

    let dir = TempDir::new().unwrap();
    for directory in ["exact", "stem"] {
        fs::create_dir(dir.child(directory)).unwrap();
    }
    for file in [
        "exact/clientele-report.v1",
        "exact/clientele-task.bat",
        "stem/clientele-report.v1.bat",
        "stem/clientele-report.v1.cmd",
        "stem/clientele-task.bat.bat",
    ] {
        fs::write(dir.child(file), "fixture").unwrap();
    }
    for directories in [["exact", "stem", "exact"], ["stem", "exact", "stem"]] {
        for extensions in [".BAT;.CMD", ".CMD;.BAT", ".V1;.BAT"] {
            let output = Command::new(env::current_exe().unwrap())
                .current_dir(dir.path())
                .env(CHILD, "1")
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
    false
}
