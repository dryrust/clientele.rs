// This is free and unencumbered software released into the public domain.

use clientele::SubcommandsProvider;
use std::path::Path;

#[path = "support/subcommands_shared.rs"]
mod subcommands_shared;
use subcommands_shared::{Result, TEST_FILES, TEST_PREFIX};

fn main() -> Result<()> {
    subcommands_shared::run(test_find)
}

fn test_find(dir: &Path) -> Result<()> {
    for file in TEST_FILES {
        println!("{}: ", file.name);

        let cd_name = file.command_name;
        let cmd = SubcommandsProvider::find(TEST_PREFIX, cd_name);
        let expected_path = file.should_be_found.then(|| dir.join(file.full_name()));

        assert_eq!(
            cmd.map(|cmd| cmd.path),
            expected_path,
            "lookup result for {cd_name:?}",
        );

        #[cfg(windows)]
        if file.should_be_found {
            let explicit_name = format!("{}.{}", cd_name, file.win_ext);
            assert_eq!(
                SubcommandsProvider::find(TEST_PREFIX, &explicit_name).map(|cmd| cmd.path),
                expected_path,
                "lookup with explicit executable extension for {explicit_name:?}",
            );
        }
    }

    Ok(())
}
