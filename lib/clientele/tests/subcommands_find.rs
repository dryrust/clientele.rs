// This is free and unencumbered software released into the public domain.

use clientele::SubcommandsProvider;

mod subcommands_shared;
use subcommands_shared::{Result, TEST_FILES, TEST_PREFIX};

#[test]
pub fn test_find() -> Result<()> {
    let dir = subcommands_shared::init()?;

    for file in TEST_FILES {
        println!("{}: ", file.name);

        let cd_name = file.name.trim_start_matches(TEST_PREFIX);
        let cmd = SubcommandsProvider::find(TEST_PREFIX, cd_name);
        let expected_path = file.should_be_found.then(|| dir.child(file.full_name()));

        assert_eq!(
            cmd.map(|cmd| cmd.path),
            expected_path,
            "lookup result for {cd_name:?}",
        );
    }

    Ok(())
}
