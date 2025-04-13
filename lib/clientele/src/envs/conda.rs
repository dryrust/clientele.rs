// This is free and unencumbered software released into the public domain.

use super::var;

pub fn conda() -> Option<String> {
    var("CONDA")
}

/// See: https://docs.conda.io/projects/conda/en/latest/user-guide/tasks/manage-environments.html
pub fn conda_prefix() -> Option<String> {
    var("CONDA_PREFIX")
}
