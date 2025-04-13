// This is free and unencumbered software released into the public domain.

use super::var;

/// See: https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/basedefs/V1_chap08.html
pub fn editor() -> Option<String> {
    var("EDITOR")
}

/// See: https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/basedefs/V1_chap08.html
pub fn home() -> Option<String> {
    var("HOME")
}

/// See: https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/basedefs/V1_chap08.html
pub fn ifs() -> Option<String> {
    var("IFS")
}

/// See: https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/basedefs/V1_chap08.html
pub fn lang() -> Option<String> {
    var("LANG")
}

/// See: https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/basedefs/V1_chap08.html
pub fn path() -> Option<String> {
    var("PATH")
}

/// See: https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/basedefs/V1_chap08.html
pub fn ps1() -> Option<String> {
    var("PS1")
}

/// See: https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/basedefs/V1_chap08.html
pub fn pwd() -> Option<String> {
    var("PWD")
}

/// See: https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/basedefs/V1_chap08.html
pub fn shell() -> Option<String> {
    var("SHELL")
}

/// See: https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/basedefs/V1_chap08.html
pub fn term() -> Option<String> {
    var("TERM")
}

/// See: https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/basedefs/V1_chap08.html
pub fn tmpdir() -> Option<String> {
    var("TMPDIR")
}

/// See: https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/basedefs/V1_chap08.html
pub fn tz() -> Option<String> {
    var("TZ")
}

/// See: https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/basedefs/V1_chap08.html
pub fn user() -> Option<String> {
    var("USER")
}
