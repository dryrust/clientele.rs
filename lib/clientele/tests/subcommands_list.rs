// This is free and unencumbered software released into the public domain.

use clientele::SubcommandsProvider;
use std::path::Path;

#[path = "support/subcommands_shared.rs"]
mod subcommands_shared;
use subcommands_shared::{Result, TEST_DIRECTORY, TEST_FILES, TEST_LEVEL, TEST_PREFIX};

fn main() -> Result<()> {
    subcommands_shared::run(test_list)
}

fn test_list(dir: &Path) -> Result<()> {
    let cmds = SubcommandsProvider::collect(TEST_PREFIX, TEST_LEVEL);
    assert!(dir.join(TEST_DIRECTORY).is_dir());
    assert!(cmds.iter().all(|cmd| cmd.path != dir.join(TEST_DIRECTORY)));

    for file in TEST_FILES {
        println!("{}: ", file.name);

        let cd_name = file.command_name;
        let cmd = cmds.iter().find(|cmd| cmd.name == cd_name);
        let path = dir.join(file.full_name());

        assert_eq!(cmd.is_some(), file.should_be_listed);

        if let Some(cmd) = cmd {
            assert_eq!(cmd.path, path);
        }
    }

    // Include deeper names so repeated prefixes cannot be hidden by the level filter.
    let cmds = SubcommandsProvider::collect(TEST_PREFIX, usize::MAX);
    for file in TEST_FILES.iter().filter(|file| file.should_be_found) {
        let cmd = cmds
            .iter()
            .find(|cmd| cmd.name == file.command_name)
            .unwrap_or_else(|| panic!("missing collected name {:?}", file.command_name));
        assert_eq!(cmd.path, dir.join(file.full_name()));

        let found = SubcommandsProvider::find(TEST_PREFIX, &cmd.name)
            .unwrap_or_else(|| panic!("collected name {:?} cannot be found", cmd.name));
        assert_eq!(&found, cmd, "round-trip lookup for {:?}", cmd.name);
    }

    Ok(())
}
