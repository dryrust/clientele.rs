// This is free and unencumbered software released into the public domain.

//! Checks XDG home overrides and fallbacks with an isolated environment in each
//! child process. The harness-free child prints its resolved path, if any.

use clientele::{paths, Utf8Path};
use std::{
    env,
    ffi::{OsStr, OsString},
    process::Command,
};
use temp_dir::TempDir;

const CHILD_MODE: &str = "CLIENTELE_XDG_TEST_CHILD";
const XDG_HOMES: [(&str, &str); 4] = [
    ("XDG_DATA_HOME", ".local/share"),
    ("XDG_CONFIG_HOME", ".config"),
    ("XDG_STATE_HOME", ".local/state"),
    ("XDG_CACHE_HOME", ".cache"),
];

fn main() {
    if let Ok(variable) = env::var(CHILD_MODE) {
        let path = match variable.as_str() {
            "XDG_DATA_HOME" => paths::xdg_data_home(),
            "XDG_CONFIG_HOME" => paths::xdg_config_home(),
            "XDG_STATE_HOME" => paths::xdg_state_home(),
            "XDG_CACHE_HOME" => paths::xdg_cache_home(),
            _ => panic!("unknown XDG variable: {variable}"),
        };
        if let Some(path) = path {
            println!("{path}");
        }
        return;
    }

    let dir = TempDir::new().expect("create isolated XDG directory");
    let root = Utf8Path::from_path(dir.path()).expect("UTF-8 fixture directory");
    // Resolution does not require either directory to exist.
    let home = root.join("home");
    let absolute = root.join("xdg-override");
    assert!(absolute.is_absolute());

    let invalid_values = [
        None,
        Some(OsString::new()),
        Some("relative/xdg".into()),
        Some(".".into()),
        Some("..".into()),
        #[cfg(windows)]
        Some(r"C:relative".into()),
        #[cfg(windows)]
        Some(r"\rooted".into()),
        #[cfg(unix)]
        Some({
            use std::os::unix::ffi::OsStringExt;
            OsString::from_vec(vec![0xff])
        }),
        #[cfg(windows)]
        Some({
            use std::os::windows::ffi::OsStringExt;
            OsString::from_wide(&[0xd800])
        }),
    ];
    let absolute_values = [
        absolute.as_path(),
        #[cfg(windows)]
        Utf8Path::new(r"\\server\share\xdg"),
    ];

    for (variable, suffix) in XDG_HOMES {
        for (home_env, fallback) in [
            (
                Some(home.as_std_path().as_os_str()),
                Some(home.join(suffix)),
            ),
            (None, None),
            (Some(OsStr::new("")), None),
        ] {
            for value in &invalid_values {
                check(variable, home_env, value.as_deref(), fallback.as_deref());
            }
            for value in absolute_values {
                check(
                    variable,
                    home_env,
                    Some(value.as_std_path().as_os_str()),
                    Some(value),
                );
            }
        }
    }
}

fn check(variable: &str, home: Option<&OsStr>, value: Option<&OsStr>, expected: Option<&Utf8Path>) {
    let mut command = Command::new(env::current_exe().unwrap());
    command
        .env(CHILD_MODE, variable)
        .env_remove("HOME")
        .env_remove("USERPROFILE")
        .env_remove("HOMEDRIVE")
        .env_remove("HOMEPATH");
    for (name, _) in XDG_HOMES {
        command.env_remove(name);
    }
    if let Some(home) = home {
        command.env("HOME", home);
    }
    if let Some(value) = value {
        command.env(variable, value);
    }
    let output = command.output().expect("run XDG child process");
    assert!(output.status.success(), "{variable}: {output:?}");
    assert!(output.stderr.is_empty(), "{variable}: {output:?}");
    let actual = String::from_utf8(output.stdout).expect("UTF-8 XDG path");
    let expected = expected.map(|path| format!("{path}\n")).unwrap_or_default();
    assert_eq!(actual, expected, "{variable}={value:?}, HOME={home:?}");
}
