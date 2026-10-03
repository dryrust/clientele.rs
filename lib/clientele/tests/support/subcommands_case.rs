// This is free and unencumbered software released into the public domain.

use clientele::{Subcommand, SubcommandsProvider};
use std::{env, fs, path::Path, process::Command};
use temp_dir::TempDir;

const CHILD: &str = "CLIENTELE_CASE_COLLISION_CHILD";
const EXTENSION: &str = if cfg!(windows) { ".BAT" } else { "" };

/// Exercise case collisions without requiring two case variants in one directory.
pub fn run() -> bool {
    if let Ok(winner) = env::var(CHILD) {
        let expected = [
            Subcommand {
                name: "HELLO".into(),
                path: Path::new("upper").join(format!("clientele-HELLO{EXTENSION}")),
            },
            Subcommand {
                name: "hello".into(),
                path: Path::new(&winner).join(format!("clientele-hello{EXTENSION}")),
            },
        ];
        assert_eq!(
            SubcommandsProvider::collect("clientele-", 1).commands(),
            expected
        );
        for command in expected {
            assert_eq!(
                SubcommandsProvider::find("clientele-", &command.name),
                Some(command)
            );
        }
        assert!(SubcommandsProvider::find("clientele-", "Hello").is_none());
        let upper_prefix = Subcommand {
            name: "hello".into(),
            path: Path::new("prefix").join(format!("CLIENTELE-hello{EXTENSION}")),
        };
        assert_eq!(
            SubcommandsProvider::collect("CLIENTELE-", 1).commands(),
            std::slice::from_ref(&upper_prefix)
        );
        assert_eq!(
            SubcommandsProvider::find("CLIENTELE-", "hello"),
            Some(upper_prefix)
        );
        return true;
    }

    let dir = TempDir::new().unwrap();
    for (directory, filename) in [
        ("prefix", "CLIENTELE-hello"),
        ("upper", "clientele-HELLO"),
        ("lower", "clientele-hello"),
        ("later", "clientele-hello"),
    ] {
        fs::create_dir(dir.child(directory)).unwrap();
        let path = dir.child(directory).join(format!("{filename}{EXTENSION}"));
        fs::write(&path, "fixture").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
        }
    }
    println!(
        "case-insensitive fixture filesystem: {}",
        dir.child("prefix")
            .join(format!("clientele-hello{EXTENSION}"))
            .exists()
    );
    for (directories, winner) in [
        (["prefix", "upper", "lower", "later", "lower"], "lower"),
        (["later", "upper", "lower", "prefix", "later"], "later"),
    ] {
        let output = Command::new(env::current_exe().unwrap())
            .current_dir(dir.path())
            .env(CHILD, winner)
            .env("PATH", env::join_paths(directories).unwrap())
            .env("PATHEXT", ".bat")
            .output()
            .unwrap();
        assert!(output.status.success(), "{directories:?}: {output:?}");
    }
    false
}
