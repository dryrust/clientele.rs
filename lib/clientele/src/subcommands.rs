// This is free and unencumbered software released into the public domain.

use std::path::{Path, PathBuf};

/// A discovered executable subcommand. Requires `std,subcommands`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Subcommand {
    /// Logical name derived from the resolved filename by both lookup and listing.
    ///
    /// Exactly one leading search prefix is removed. Unix preserves all filename
    /// suffixes; Windows removes the final extension, even when explicitly supplied
    /// to lookup. Earlier dots and repeated prefixes are preserved.
    pub name: String,
    /// Resolved executable path, retaining its prefix and any file extension.
    pub path: PathBuf,
}

impl Subcommand {
    fn from_path(prefix: &str, path: PathBuf) -> Option<Self> {
        let name = if cfg!(windows) {
            path.file_stem()?
        } else {
            path.file_name()?
        };
        let name = name.to_str()?.strip_prefix(prefix)?.to_string();
        Some(Self { name, path })
    }
}

#[derive(Debug, Clone)]
pub struct SubcommandsProvider {
    commands: Vec<Subcommand>,
}

impl SubcommandsProvider {
    /// Collects executable subcommands from `PATH` up to the requested name depth.
    ///
    /// Requires `std,subcommands`. Each name has exactly one leading `prefix`
    /// removed. Unix preserves the rest of the filename, including dot suffixes.
    /// Windows removes the final executable extension matched by `PATHEXT` but
    /// preserves earlier dots: `demo-report.v1.bat` becomes `report.v1` with
    /// prefix `demo-`. Collected names can be passed to [`Self::find`] with the
    /// same prefix.
    /// Directories are excluded, even if their names have executable extensions.
    /// On Unix, empty `PATH` components (including an empty `PATH`) search the
    /// current directory and return relative executable paths. An unset `PATH`
    /// yields no commands.
    ///
    /// On Windows, `PATHEXT` is a semicolon-separated list of dot-prefixed,
    /// nonempty extensions, matched case-insensitively in their original order.
    /// Empty entries, bare dots, and entries without a leading dot are ignored;
    /// whitespace and quotes are not trimmed. Missing or non-Unicode `PATHEXT`,
    /// or a list with no accepted extensions, yields no collected commands.
    ///
    /// Results are sorted lexically by logical name using Rust string ordering.
    /// Exactly equal names are returned once, with the first usable executable
    /// in `PATH` order. On Windows, lookup's exact-filename and `PATHEXT`
    /// precedence select the executable, rather than directory enumeration order.
    /// Names with different spelling or case remain distinct.
    ///
    /// Only names containing fewer than `level` hyphens are returned. Thus `1`
    /// lists top-level commands and `0` returns none. Any retained occurrence of
    /// the prefix counts toward this depth: `demo-demo-repeat` becomes
    /// `demo-repeat` on Unix and requires a level of at least `2`.
    pub fn collect(prefix: &str, level: usize) -> SubcommandsProvider {
        let mut commands: Vec<_> = Self::collect_commands(prefix)
            .into_iter()
            // Construct public command names.
            .filter_map(|path| Subcommand::from_path(prefix, path))
            // Respect level.
            .filter(|cmd| {
                let count = cmd.name.chars().filter(|&c| c == '-').count();
                count < level
            })
            .collect();

        // Stable sorting preserves PATH precedence among equal logical names.
        commands.sort_by(|left, right| left.name.cmp(&right.name));
        commands.dedup_by(|left, right| left.name == right.name);

        #[cfg(windows)]
        let commands = commands
            .into_iter()
            .filter_map(|mut command| {
                // Multiple extensions can share a stem in the same directory.
                // Reuse lookup to honor PATHEXT and exact-filename precedence.
                command.path = Self::resolve_command(prefix, &format!("{prefix}{}", command.name))?;
                Some(command)
            })
            .collect();

        SubcommandsProvider { commands }
    }

    /// Finds the first usable executable for the prefixed name in `PATH` order.
    ///
    /// Requires `std,subcommands`. On Windows, each directory first checks the
    /// exact prefixed filename if it has an extension. If no usable exact match
    /// exists, `PATHEXT` extensions are appended in order, preserving any dots
    /// already present in the command name.
    /// Directories are never returned, including for explicit filename lookups.
    /// Parsing follows [`Self::collect`]'s `PATHEXT` rules. Missing or non-Unicode
    /// `PATHEXT` returns `None`, even for an explicit filename. An empty list of
    /// accepted extensions still permits exact filename lookup on Windows.
    ///
    /// On Unix, empty `PATH` components search the current directory, whereas an
    /// unset `PATH` returns `None`.
    ///
    /// Returns `None` if no match is found or the required search variables are
    /// unavailable. The returned [`Subcommand::name`] follows the same naming
    /// rules as [`Self::collect`]: exactly one prefix is removed, along with the
    /// final extension on Windows. For example, `find("demo-", "hello")` returns
    /// the logical name `hello`, not `demo-hello`; Windows lookup of `hello.bat`
    /// also returns `hello` when that executable is found.
    pub fn find(prefix: &str, name: &str) -> Option<Subcommand> {
        let name = format!("{}{}", prefix, name);
        let path = Self::resolve_command(prefix, &name);
        path.and_then(|path| Subcommand::from_path(prefix, path))
    }
}

impl SubcommandsProvider {
    pub fn iter(&self) -> impl Iterator<Item = &Subcommand> {
        self.commands.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = Subcommand> {
        self.commands.into_iter()
    }

    pub fn get_commands(&self) -> &Vec<Subcommand> {
        &self.commands
    }

    pub fn into_commands(self) -> Vec<Subcommand> {
        self.commands
    }
}

#[cfg(unix)]
impl SubcommandsProvider {
    fn filter_file(prefix: &str, path: &Path) -> bool {
        use std::os::unix::prelude::*;

        let file_name = path.file_name();
        let Some(entry_name) = file_name.and_then(|name| name.to_str()) else {
            // skip files with invalid names.
            return false;
        };

        if entry_name.starts_with(".") || entry_name.ends_with("~") {
            // skip hidden and backup files.
            return false;
        }

        if !entry_name.starts_with(prefix) {
            // skip non-matching files.
            return false;
        }

        let Ok(metadata) = std::fs::metadata(path) else {
            // couldn't get metadata.
            return false;
        };

        if !metadata.is_file() || metadata.permissions().mode() & 0o111 == 0 {
            // skip non-executable files.
            return false;
        }

        true
    }

    fn collect_commands(prefix: &str) -> Vec<PathBuf> {
        let Some(paths) = std::env::var_os("PATH") else {
            // PATH variable is not set.
            return vec![];
        };

        let mut result = vec![];
        for path in std::env::split_paths(&paths) {
            let directory = if path.as_os_str().is_empty() {
                Path::new(".")
            } else {
                &path
            };
            let Ok(dir) = std::fs::read_dir(directory) else {
                continue;
            };

            for entry in dir {
                let Ok(entry) = entry else {
                    // invalid entry.
                    continue;
                };

                // Keep the same path spelling as lookup, including empty components.
                let path = path.join(entry.file_name());
                if Self::filter_file(prefix, &path) {
                    result.push(path);
                }
            }
        }

        result
    }

    fn resolve_command(prefix: &str, command: &str) -> Option<PathBuf> {
        let Some(paths) = std::env::var_os("PATH") else {
            // PATH variable is not set.
            return None;
        };

        for path in std::env::split_paths(&paths) {
            let path = path.join(command);

            if !path.exists() {
                continue;
            }

            if !Self::filter_file(prefix, &path) {
                continue;
            }

            return Some(path);
        }

        None
    }
}

#[cfg(windows)]
impl SubcommandsProvider {
    fn get_path_exts() -> Option<Vec<String>> {
        let Ok(exts) = std::env::var("PATHEXT") else {
            // PATHEXT variable is not set.
            return None;
        };

        Some(parse_path_exts(&exts))
    }

    fn filter_file(prefix: &str, path: &Path, exts: Option<&[String]>) -> bool {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_HIDDEN: u32 = 0x00000002;

        let file_name = path.file_name();
        let Some(entry_name) = file_name.and_then(|name| name.to_str()) else {
            // skip files with invalid names.
            return false;
        };

        if !entry_name.starts_with(prefix) {
            // skip non-matching files.
            return false;
        }

        if let Some(exts) = exts {
            let Some(entry_ext) = path.extension().and_then(|ext| ext.to_str()) else {
                // skip files without extensions.
                return false;
            };

            let entry_ext = entry_ext.to_lowercase();
            if !exts.contains(&entry_ext) {
                // skip non-executable files
                return false;
            }
        }

        let Ok(metadata) = std::fs::metadata(path) else {
            // couldn't get metadata.
            return false;
        };

        let is_hidden = metadata.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0;
        if !metadata.is_file() || is_hidden {
            // Skip directories and other non-files, as well as hidden files.
            return false;
        }

        true
    }

    fn collect_commands(prefix: &str) -> Vec<PathBuf> {
        let Some(paths) = std::env::var_os("PATH") else {
            // PATH variable is not set.
            return vec![];
        };

        let Some(exts) = Self::get_path_exts() else {
            // PATHEXT variable is not set or invalid.
            return vec![];
        };

        let mut result = vec![];
        for path in std::env::split_paths(&paths) {
            let Ok(dir) = std::fs::read_dir(path) else {
                continue;
            };

            for entry in dir {
                let Ok(entry) = entry else {
                    // invalid entry.
                    continue;
                };

                let path = entry.path();
                if Self::filter_file(prefix, &path, Some(&exts)) {
                    result.push(path);
                }
            }
        }

        result
    }

    fn resolve_command(prefix: &str, command: &str) -> Option<PathBuf> {
        let Some(paths) = std::env::var_os("PATH") else {
            // PATH variable is not set.
            return None;
        };

        let Some(exts) = Self::get_path_exts() else {
            // PATHEXT variable is not set or invalid.
            return None;
        };

        for path in std::env::split_paths(&paths) {
            let path = path.join(command);

            // Prefer an explicitly provided executable filename in this directory.
            if path.extension().is_some() && path.exists() && Self::filter_file(prefix, &path, None)
            {
                return Some(path);
            }

            // Append executable extensions without replacing dots in the command stem.
            for ext in &exts {
                let path = path.with_added_extension(ext);

                match path.exists() {
                    true if Self::filter_file(prefix, &path, None) => return Some(path),
                    _ => continue,
                }
            }
        }

        None
    }
}

// Kept platform-independent under tests so malformed Windows input is covered
// on every development platform without mutating process-global environment.
#[cfg(any(windows, test))]
fn parse_path_exts(value: &str) -> Vec<String> {
    value
        .split(';')
        .filter_map(|entry| entry.strip_prefix('.'))
        .filter(|extension| !extension.is_empty())
        .map(str::to_lowercase)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_path_exts;

    #[test]
    fn ignores_empty_and_missing_dot_entries() {
        for value in ["", ";;;", ".", "EXE;CMD", "éxe;λ;."] {
            assert!(parse_path_exts(value).is_empty(), "{value:?}");
        }
        assert_eq!(parse_path_exts(";.EXE;;CMD;.;.BAT;"), ["exe", "bat"]);
    }

    #[test]
    fn preserves_precedence_and_duplicates_with_case_folding() {
        assert_eq!(
            parse_path_exts(".cMd;.EXE;.bAt;.CMD"),
            ["cmd", "exe", "bat", "cmd"]
        );
    }

    #[test]
    fn handles_unicode_without_splitting_code_points() {
        assert_eq!(parse_path_exts("éxe;.ÉXE;λ;.Λ;.工具"), ["éxe", "λ", "工具"]);
    }

    #[test]
    fn does_not_trim_whitespace_or_quotes() {
        assert_eq!(
            parse_path_exts(" .EXE;\".CMD\";.BAT ;. CMD"),
            ["bat ", " cmd"]
        );
    }
}
