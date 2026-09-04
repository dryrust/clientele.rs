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
pub struct SortKeys<T: Clone = String> {
    keys: Vec<SortKey<T>>,
}

impl<T: Clone> AsRef<[SortKey<T>]> for SortKeys<T> {
    fn as_ref(&self) -> &[SortKey<T>] {
        &self.keys
    }
}

impl<T: Clone> From<Vec<SortKey<T>>> for SortKeys<T> {
    fn from(input: Vec<SortKey<T>>) -> Self {
        Self { keys: input }
    }
}

impl<T: Clone + Default> Default for SortKeys<T> {
    fn default() -> Self {
        Self {
            keys: vec![SortKey::<T>::default()],
        }
    }
}

impl<T: Clone> SortKeys<T> {
    pub fn empty() -> Self {
        Self { keys: vec![] }
    }

    pub fn new(keys: &[SortKey<T>]) -> Self {
        Self {
            keys: keys.to_owned(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn keys(&self) -> &[SortKey<T>] {
        &self.keys
    }
}

impl<T: Clone + ToString> SortKeys<T> {
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
pub struct SortKey<T: Clone = String> {
    key: T,
    descending: bool,
}

impl core::fmt::Display for SortKey<String> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}", if self.descending { "-" } else { "" }, self.key)
    }
}

impl<T: Clone> From<(T, bool)> for SortKey<T> {
    fn from((key, descending): (T, bool)) -> Self {
        Self::new(key, descending)
    }
}

impl<T: Clone + Default> Default for SortKey<T> {
    fn default() -> Self {
        Self::new(T::default(), false)
    }
}

impl<T: Clone> SortKey<T> {
    pub fn new(key: impl Into<T>, descending: bool) -> Self {
        Self {
            key: key.into(),
            descending,
        }
    }

    pub fn key(&self) -> &T {
        &self.key
    }

    pub fn ascending(&self) -> bool {
        !self.descending
    }

    pub fn descending(&self) -> bool {
        self.descending
    }
}

impl<T: Clone + ToString> SortKey<T> {
    pub fn to_sql(&self) -> String {
        // TODO: check that the key is SQL safe
        format!(
            "{} {}",
            self.key.to_string(),
            if self.descending { "DESC" } else { "ASC" }
        )
    }
}

fn parse_sort_keys<T: Clone>(
    input: &str,
    parse_key: impl Fn(&str) -> Result<T, String>,
) -> Result<SortKeys<T>, String> {
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

            Ok(SortKey {
                key: parse_key(key)?,
                descending,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(SortKeys { keys })
}

impl FromStr for SortKeys {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_sort_keys(input, |key| Ok(key.to_owned()))
    }
}

fn parse_value_enum_sort_keys<T: clap::ValueEnum>(input: &str) -> Result<SortKeys<T>, String> {
    parse_sort_keys(input, |key| <T as clap::ValueEnum>::from_str(key, false))
}

impl<T> clap::builder::ValueParserFactory for SortKeys<T>
where
    T: clap::ValueEnum + Send + Sync + 'static,
{
    type Parser = clap::builder::ValueParser;

    fn value_parser() -> Self::Parser {
        clap::builder::ValueParser::new(parse_value_enum_sort_keys::<T>)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::{SortKey, SortKeys};
    use alloc::{borrow::ToOwned, vec};
    use clap::{Parser, ValueEnum};

    #[derive(Parser, Debug)]
    struct Args {
        #[clap(long, value_name = "[+|-]KEY,...", allow_hyphen_values = true)]
        sort: Option<SortKeys>,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq, ValueEnum)]
    enum Column {
        Handle,
        Id,
        Name,
    }

    #[derive(Parser, Debug)]
    struct EnumArgs {
        #[clap(long, value_name = "[+|-]KEY,...", allow_hyphen_values = true)]
        sort: Option<SortKeys<Column>>,
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
    fn parses_value_enum_sort_keys() {
        let args = EnumArgs::try_parse_from(["my-program", "--sort=-name,+id,handle"]).unwrap();

        assert_eq!(
            args.sort,
            Some(SortKeys::new(&[
                SortKey::new(Column::Name, true),
                SortKey::new(Column::Id, false),
                SortKey::new(Column::Handle, false),
            ]))
        );
    }

    #[test]
    fn rejects_unknown_value_enum_sort_keys() {
        assert!(EnumArgs::try_parse_from(["my-program", "--sort=unknown"]).is_err());
    }

    #[test]
    fn rejects_empty_sort_keys() {
        for input in ["", ",", "name,", ",name", "name,,id", "+", "-"] {
            assert!(input.parse::<SortKeys>().is_err(), "accepted {input:?}");
        }
    }
}
