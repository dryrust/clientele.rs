// This is free and unencumbered software released into the public domain.

use clientele::{Subcommand, SubcommandsProvider};
use std::{env, ffi::OsString, fs, path::Path, process::Command};
use temp_dir::TempDir;

const CHILD: &str = "CLIENTELE_NATIVE_DISCOVERY_CHILD";
const EXTENSION: &str = if cfg!(windows) { ".bat" } else { "" };

fn non_unicode() -> OsString {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        OsString::from_vec(vec![b'n', 0xff])
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStringExt;
        OsString::from_wide(&[b'n' as u16, 0xd800])
    }
}

/// Compare native paths directly; never use lossy text as the expected OS path.
pub fn run() -> bool {
    if let Some(directory) = env::var_os(CHILD) {
        let directory = Path::new(&directory);
        assert!(directory.to_str().is_none());
        assert_eq!(env::var_os("PATH").as_deref(), Some(directory.as_os_str()));
        let expected = ["alpha", "zulu"].map(|name| Subcommand {
            name: name.into(),
            path: directory.join(format!("clientele-{name}{EXTENSION}")),
        });
        assert_eq!(
            SubcommandsProvider::collect("clientele-", 1).commands(),
            expected
        );
        for command in expected {
            #[cfg(windows)]
            assert_eq!(
                SubcommandsProvider::find("clientele-", &format!("{}.BAT", command.name)),
                Some(command.clone())
            );
            assert_eq!(
                SubcommandsProvider::find("clientele-", &command.name),
                Some(command)
            );
        }
        let query = format!("invalid{}", non_unicode().to_string_lossy());
        assert!(SubcommandsProvider::find("clientele-", &query).is_none());
        return true;
    }

    let dir = TempDir::new().unwrap();
    let directory = std::path::absolute(dir.path()).unwrap().join(non_unicode());
    if let Err(error) = fs::create_dir(&directory) {
        // APFS/HFS+ can reject invalid UTF-8. Do not hide unrelated fixture errors.
        if (cfg!(target_os = "macos") && matches!(error.raw_os_error(), Some(22 | 92)))
            || (cfg!(windows) && error.raw_os_error() == Some(123))
        {
            println!("non-Unicode discovery fixture unsupported by this filesystem: {error}");
            return false;
        }
        panic!("create native discovery directory: {error}");
    }
    let mut invalid = OsString::from("clientele-invalid");
    invalid.push(non_unicode());
    for mut filename in ["clientele-zulu".into(), invalid, "clientele-alpha".into()] {
        filename.push(EXTENSION);
        let path = directory.join(&filename);
        fs::write(&path, "fixture").unwrap();
        assert!(
            fs::read_dir(&directory)
                .unwrap()
                .any(|entry| entry.unwrap().file_name() == filename)
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
        }
    }
    let output = Command::new(env::current_exe().unwrap())
        .env(CHILD, &directory)
        .env("PATH", &directory)
        .env("PATHEXT", ".BAT")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "native discovery child: {output:?}"
    );
    false
}
