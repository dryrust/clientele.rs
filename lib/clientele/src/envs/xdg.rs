// This is free and unencumbered software released into the public domain.

use super::var;

/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_data_home() -> Option<String> {
    var("XDG_DATA_HOME")
}

/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_config_home() -> Option<String> {
    var("XDG_CONFIG_HOME")
}

/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_state_home() -> Option<String> {
    var("XDG_STATE_HOME")
}

/// See: https://specifications.freedesktop.org/basedir-spec/latest/#variables
pub fn xdg_cache_home() -> Option<String> {
    var("XDG_CACHE_HOME")
}
