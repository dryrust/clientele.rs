// This is free and unencumbered software released into the public domain.

use std::{env, path::Path, process::Command};
use temp_dir::TempDir;

const CHILD_DIR: &str = "CLIENTELE_SUBCOMMANDS_TEST_DIR";

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct TestFile {
    pub name: &'static str,
    pub command_name: &'static str,
    pub content: &'static str,
    #[allow(dead_code)]
    pub should_be_listed: bool,
    #[allow(dead_code)]
    pub should_be_found: bool,
    #[allow(dead_code)]
    pub win_ext: &'static str,
}

impl TestFile {
    pub fn full_name(&self) -> String {
        #[cfg(windows)]
        return format!("{}.{}", self.name, self.win_ext);
        #[cfg(unix)]
        return self.name.to_string();
    }
}

pub static TEST_PREFIX: &str = "clientele-";
pub const TEST_DIRECTORY: &str = "clientele-directory.bat";

#[allow(unused)]
pub static TEST_LEVEL: usize = 1;

pub static TEST_FILES: &[TestFile] = &[
    TestFile {
        name: "clientele-hello",
        command_name: "hello",
        content: "Hello, world!",
        should_be_listed: true,
        should_be_found: true,
        win_ext: "bat",
    },
    TestFile {
        name: "clientele-two-levels",
        command_name: "two-levels",
        content: "Should be filtered out!",
        should_be_listed: false,
        should_be_found: true, // Lookup is not limited by listing depth.
        win_ext: "bat",
    },
    TestFile {
        name: "abcdefg-test",
        command_name: "abcdefg-test",
        content: "Shouldn't appear!",
        should_be_listed: false,
        should_be_found: false,
        win_ext: "bat",
    },
    TestFile {
        name: "clientele-report.v1",
        command_name: "report.v1",
        content: "Keep the dotted command name!",
        should_be_listed: true,
        should_be_found: true,
        win_ext: "bat",
    },
    TestFile {
        name: "clientele-clientele-repeat",
        command_name: "clientele-repeat",
        content: "Remove the prefix only once!",
        should_be_listed: false,
        should_be_found: true,
        win_ext: "bat",
    },
    TestFile {
        name: "clientele-clientele-report.v1",
        command_name: "clientele-report.v1",
        content: "Keep both the repeated prefix and dotted suffix!",
        should_be_listed: false,
        should_be_found: true,
        win_ext: "cmd",
    },
    #[cfg(windows)]
    TestFile {
        name: "clientele-hola",
        command_name: "hola",
        content: "Hola mundo!",
        should_be_listed: true,
        should_be_found: true,
        win_ext: "cmd",
    },
];

/// Runs discovery assertions in a child with an isolated fixture directory.
///
/// The child receives its own `PATH` and, on Windows, `PATHEXT`. Passing the
/// fixture directory as an OS string preserves non-UTF-8 paths. The parent
/// retains ownership of the fixtures until the child exits.
pub fn run(check: impl FnOnce(&Path) -> Result<()>) -> Result<()> {
    if let Some(dir) = env::var_os(CHILD_DIR) {
        return check(Path::new(&dir));
    }

    let original_environment = ["PATH", "PATHEXT"].map(|key| (key, env::var_os(key)));
    let dir = init()?;
    // Resolve relative temp paths before changing the child's working directory.
    let fixture_dir = std::path::absolute(dir.path())?;
    let mut command = Command::new(env::current_exe()?);
    command
        .current_dir(&fixture_dir)
        .env(CHILD_DIR, &fixture_dir)
        .env("PATH", &fixture_dir);
    #[cfg(windows)]
    command.env("PATHEXT", ".BAT;.CMD");

    let output = command.output()?;
    assert!(
        output.status.success(),
        "discovery child failed with {}:\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    for (key, value) in original_environment {
        assert_eq!(env::var_os(key), value, "parent {key} changed");
    }
    Ok(())
}

/// Creates executable fixtures without changing the process environment.
pub fn init() -> Result<TempDir> {
    let dir = TempDir::new()?;
    std::fs::create_dir(dir.child(TEST_DIRECTORY))?;

    #[cfg(unix)]
    for file in TEST_FILES {
        use std::fs::OpenOptions;
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;

        let content = format!("#!/bin/sh\necho {}", file.content);
        let path = dir.child(file.name);
        let mut file = OpenOptions::new()
            .write(true)
            .mode(0o755)
            .truncate(true)
            .create(true)
            .open(&path)?;
        file.write_all(content.as_bytes())?;
    }

    #[cfg(windows)]
    for file in TEST_FILES {
        let name = file.full_name();
        let content = format!("@echo off\necho {}", file.content);
        std::fs::write(dir.child(name), content)?;
    }

    Ok(dir)
}
