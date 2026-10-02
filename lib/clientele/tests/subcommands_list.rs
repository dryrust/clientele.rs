// This is free and unencumbered software released into the public domain.

//! Complete, ordered listings and lookup round trips in an isolated search path.

use clientele::{Subcommand, SubcommandsProvider};
use std::path::Path;

#[path = "support/subcommands_shared.rs"]
mod subcommands_shared;
use subcommands_shared::{Result, TEST_DIRECTORY, TEST_FILES, TEST_LEVEL, TEST_PREFIX};

fn main() -> Result<()> {
    subcommands_shared::run(test_list)
}

fn test_list(dir: &Path) -> Result<()> {
    assert!(dir.join(TEST_DIRECTORY).is_dir());
    for file in TEST_FILES {
        assert!(
            dir.join(file.full_name()).is_file(),
            "fixture {}",
            file.name
        );
    }

    // Include deeper names so repeated prefixes cannot be hidden by the level filter.
    for level in [TEST_LEVEL, usize::MAX] {
        let mut expected: Vec<_> = TEST_FILES
            .iter()
            .filter(|file| {
                if level == TEST_LEVEL {
                    file.should_be_listed
                } else {
                    file.should_be_found
                }
            })
            .map(|file| Subcommand {
                name: file.command_name.to_string(),
                path: dir.join(file.full_name()),
            })
            .collect();
        expected.sort_by(|left, right| left.name.cmp(&right.name));

        let cmds = SubcommandsProvider::collect(TEST_PREFIX, level).into_commands();
        assert_eq!(cmds, expected, "complete listing at level {level}");

        for cmd in &cmds {
            let found = SubcommandsProvider::find(TEST_PREFIX, &cmd.name)
                .unwrap_or_else(|| panic!("collected name {:?} cannot be found", cmd.name));
            assert_eq!(&found, cmd, "round-trip lookup for {:?}", cmd.name);
        }
    }

    Ok(())
}
