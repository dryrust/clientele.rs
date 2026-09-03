extern crate alloc;

use alloc::{borrow::ToOwned, format, string::String, vec, vec::Vec};
use core::str::FromStr;

/// A sequence of sort keys.
///
/// ```rust,ignore
/// /// Sort resources by the specified keys. (Prefix a key with `-` for descending order.)
/// #[clap(long, aliases = ["sort-by", "order", "order-by"], value_name = "[+|-]KEY,...", allow_hyphen_values = true)]
/// sort: Option<SortKeys>,
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SortKeys<T: Clone + ToString = String> {
    keys: Vec<SortKey<T>>,
}

impl Default for SortKeys {
    fn default() -> Self {
        Self { keys: vec![] }
    }
}

impl<T: Clone + ToString> SortKeys<T> {
    pub fn new(keys: &[SortKey<T>]) -> Self {
        Self {
            keys: keys.to_owned(),
        }
    }

    pub fn to_sql(&self) -> String {
        self.keys
            .iter()
            .map(|key| key.to_sql())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl core::fmt::Display for SortKeys {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (i, key) in self.keys.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", key)?;
        }
        Ok(())
    }
}

/// A sort key.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SortKey<T: Clone + ToString = String> {
    key: T,
    descending: bool,
}

impl core::fmt::Display for SortKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}", if self.descending { "-" } else { "" }, self.key)
    }
}

impl<T: Clone + ToString> SortKey<T> {
    pub fn new(key: impl Into<T>, descending: bool) -> Self {
        Self {
            key: key.into(),
            descending,
        }
    }

    pub fn to_sql(&self) -> String {
        format!(
            "{} {}",
            self.key.to_string(),
            if self.descending { "DESC" } else { "ASC" }
        )
    }
}

impl FromStr for SortKeys {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err("sort expression must contain at least one key".to_owned());
        }

        let keys = input
            .split(',')
            .map(|key| {
                let (descending, key) = match key.as_bytes().first() {
                    Some(b'-') => (true, &key[1..]),
                    Some(b'+') => (false, &key[1..]),
                    _ => (false, key),
                };

                if key.is_empty() {
                    return Err("sort keys must not be empty".to_owned());
                }
                if key.starts_with('+') || key.starts_with('-') {
                    return Err(format!("invalid sort key: {key}"));
                }
                // TODO: check that the key is SQL safe

                Ok(SortKey {
                    key: key.to_owned(),
                    descending,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { keys })
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::{SortKey, SortKeys};
    use alloc::{borrow::ToOwned, vec};
    use clap::Parser;

    #[derive(Parser, Debug)]
    struct Args {
        #[clap(long, value_name = "[+|-]KEY,...", allow_hyphen_values = true)]
        sort: Option<SortKeys>,
    }

    #[test]
    fn parses_sort_properties() {
        let args = Args::try_parse_from(["my-program", "--sort=-name,+id,handle"]).unwrap();

        assert_eq!(
            args.sort,
            Some(SortKeys {
                keys: vec![
                    SortKey {
                        key: "name".to_owned(),
                        descending: true,
                    },
                    SortKey {
                        key: "id".to_owned(),
                        descending: false,
                    },
                    SortKey {
                        key: "handle".to_owned(),
                        descending: false,
                    },
                ],
            })
        );
    }

    #[test]
    fn accepts_a_separate_hyphenated_sort_value() {
        let args = Args::try_parse_from(["my-program", "--sort", "-name"]).unwrap();

        assert_eq!(
            args.sort,
            Some(SortKeys {
                keys: vec![SortKey {
                    key: "name".to_owned(),
                    descending: true,
                }],
            })
        );
    }

    #[test]
    fn rejects_empty_sort_keys() {
        for input in ["", ",", "name,", ",name", "name,,id", "+", "-"] {
            assert!(input.parse::<SortKeys>().is_err(), "accepted {input:?}");
        }
    }
}
