// This is free and unencumbered software released into the public domain.

use clientele::SubcommandsProvider;

mod subcommands_shared;
use subcommands_shared::{Result, TEST_FILES, TEST_LEVEL, TEST_PREFIX};

#[test]
pub fn test_list() -> Result<()> {
    let dir = subcommands_shared::init()?;
    let cmds = SubcommandsProvider::collect(TEST_PREFIX, TEST_LEVEL);

    for file in TEST_FILES {
        println!("{}: ", file.name);

        let cd_name = file.name.trim_start_matches(TEST_PREFIX);
        let cmd = cmds.iter().find(|cmd| cmd.name == cd_name);
        let path = dir.child(file.full_name());

        assert_eq!(cmd.is_some(), file.should_be_listed);

        if let Some(cmd) = cmd {
            assert_eq!(cmd.path, path);
        }
    }

    Ok(())
}
