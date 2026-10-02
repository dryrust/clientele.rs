extern crate alloc;

use alloc::{borrow::ToOwned, format, string::String, vec, vec::Vec};
use core::str::FromStr;

/// A failure to render sort keys through an approved SQL column mapping.
///
/// Available with `std,clap`. Checked rendering stops at the first failure and
/// returns no partial SQL fragment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SortSqlError {
    /// The mapping returned `None` for a requested sort key.
    UnmappedKey,
    /// The mapped column is not a supported unquoted SQL identifier or path.
    InvalidColumn {
        /// The rejected column text returned by the mapping.
        column: String,
    },
}

impl core::fmt::Display for SortSqlError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnmappedKey => write!(f, "sort key has no approved SQL column"),
            Self::InvalidColumn { column } => write!(f, "invalid SQL column mapping: {column:?}"),
        }
    }
}

impl core::error::Error for SortSqlError {}

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

    /// Renders a comma-separated SQL ordering fragment using approved columns.
    ///
    /// Requires `std,clap`. Calls `column_for` in key order and renders only its
    /// returned columns, preserving each key's direction. The mapping must select
    /// columns approved by the application; keys need not implement `ToString`.
    /// Columns follow [`SortKey::to_sql_checked`]'s identifier rules. No `ORDER BY`
    /// keyword is included. An empty sequence returns an empty string without
    /// calling the mapping.
    ///
    /// # Errors
    ///
    /// Returns [`SortSqlError::UnmappedKey`] for the first unmapped key, or
    /// [`SortSqlError::InvalidColumn`] for the first invalid mapped column.
    /// No partial SQL fragment is returned.
    ///
    /// ```
    /// use clientele::options::sort::{SortKey, SortKeys};
    ///
    /// #[derive(Clone)]
    /// enum Field { Name, Created }
    ///
    /// let sort = SortKeys::new(&[
    ///     SortKey::new(Field::Created, true),
    ///     SortKey::new(Field::Name, false),
    /// ]);
    /// let sql = sort.to_sql_checked(|field| match field {
    ///     Field::Name => Some("users.display_name"),
    ///     Field::Created => Some("users.created_at"),
    /// })?;
    /// assert_eq!(sql, "users.created_at DESC, users.display_name ASC");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn to_sql_checked<'a>(
        &'a self,
        mut column_for: impl FnMut(&'a T) -> Option<&'a str>,
    ) -> Result<String, SortSqlError> {
        self.keys
            .iter()
            .map(|key| key.to_sql_checked(&mut column_for))
            .collect::<Result<Vec<_>, _>>()
            .map(|parts| parts.join(", "))
    }
}

impl<T: Clone + ToString> SortKeys<T> {
    /// Renders raw key text as a comma-separated SQL ordering fragment.
    ///
    /// Requires `std,clap`. No `ORDER BY` keyword is included. An empty sequence
    /// returns an empty string.
    ///
    /// Keys are interpolated verbatim, without validation, quoting, or escaping.
    /// Only use this method with trusted SQL text; parsed CLI keys are untrusted.
    /// Use [`Self::to_sql_checked`] to map user-selected keys to approved columns.
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

    /// Renders an approved SQL column followed by `ASC` or `DESC`.
    ///
    /// Requires `std,clap`. `column_for` must return a column approved by the
    /// application, or `None` to reject the key. The key's own text is never
    /// interpolated. Each dot-separated column component must match
    /// `[A-Za-z_][A-Za-z0-9_]*`, for example `display_name` or `users.id`.
    /// Expressions, quotes, whitespace, wildcards, and empty components are
    /// rejected. Columns are not quoted or normalized; the application must
    /// choose names valid in its SQL dialect, including avoiding reserved words.
    ///
    /// # Errors
    ///
    /// Returns [`SortSqlError::UnmappedKey`] when the mapping returns `None`, or
    /// [`SortSqlError::InvalidColumn`] when its result violates the grammar above.
    ///
    /// ```
    /// use clientele::options::sort::SortKey;
    ///
    /// let key = SortKey::<String>::new("display-name", false);
    /// let sql = key.to_sql_checked(|key| match key.as_str() {
    ///     "display-name" => Some("display_name"),
    ///     _ => None,
    /// })?;
    /// assert_eq!(sql, "display_name ASC");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn to_sql_checked<'a>(
        &'a self,
        column_for: impl FnOnce(&'a T) -> Option<&'a str>,
    ) -> Result<String, SortSqlError> {
        let column = column_for(&self.key).ok_or(SortSqlError::UnmappedKey)?;
        if !is_sql_column(column) {
            return Err(SortSqlError::InvalidColumn {
                column: column.to_owned(),
            });
        }
        Ok(format!(
            "{} {}",
            column,
            if self.descending { "DESC" } else { "ASC" }
        ))
    }
}

impl<T: Clone + ToString> SortKey<T> {
    /// Renders raw key text followed by `ASC` or `DESC`.
    ///
    /// Requires `std,clap`. The key is interpolated verbatim, without validation,
    /// quoting, or escaping. Only use this method with trusted SQL text; use
    /// [`Self::to_sql_checked`] to map untrusted keys to approved columns.
    pub fn to_sql(&self) -> String {
        format!(
            "{} {}",
            self.key.to_string(),
            if self.descending { "DESC" } else { "ASC" }
        )
    }
}

fn is_sql_column(column: &str) -> bool {
    column.split('.').all(|part| {
        let mut bytes = part.bytes();
        matches!(bytes.next(), Some(b'A'..=b'Z' | b'a'..=b'z' | b'_'))
            && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    })
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

    use super::{SortKey, SortKeys, SortSqlError};
    use alloc::{borrow::ToOwned, format, vec};
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

    #[test]
    fn renders_checked_enum_sort_keys() {
        let args = EnumArgs::try_parse_from(["my-program", "--sort=-name,+id,handle"]).unwrap();
        let sort = args.sort.unwrap();
        let sql = sort.to_sql_checked(|column| {
            Some(match column {
                Column::Name => "users.display_name",
                Column::Id => "users.id",
                Column::Handle => "users.handle",
            })
        });
        assert_eq!(
            sql.unwrap(),
            "users.display_name DESC, users.id ASC, users.handle ASC"
        );
    }

    #[test]
    fn maps_string_aliases_before_rendering_sql() {
        let sort = "-display-name,+id".parse::<SortKeys>().unwrap();
        assert_eq!(
            sort.to_sql_checked(|key| match key.as_str() {
                "display-name" => Some("people.display_name"),
                "id" => Some("people.id"),
                _ => None,
            })
            .unwrap(),
            "people.display_name DESC, people.id ASC"
        );
    }

    #[test]
    fn rejects_unapproved_keys_without_returning_partial_sql() {
        for input in [
            "unknown",
            "password",
            "(SELECT 1)",
            "name DESC",
            "name; DROP TABLE users",
            "name/*comment*/",
            "\"name\"",
            "`name`",
            "[name]",
        ] {
            let sort = format!("name,-{input}").parse::<SortKeys>().unwrap();
            assert_eq!(
                sort.to_sql_checked(|key| match key.as_str() {
                    "name" => Some("users.name"),
                    _ => None,
                }),
                Err(SortSqlError::UnmappedKey),
                "accepted unapproved key {input:?}",
            );
        }
        let key = SortKey::<Column>::new(Column::Name, false);
        assert_eq!(key.to_sql_checked(|_| None), Err(SortSqlError::UnmappedKey));
    }

    #[test]
    fn rejects_invalid_mapped_columns() {
        let key = SortKey::<Column>::new(Column::Name, false);
        for column in [
            "",
            " ",
            "1name",
            ".name",
            "users.",
            "users..name",
            "name DESC",
            "name, id",
            "name; DROP TABLE users",
            "(SELECT 1)",
            "coalesce(name, '')",
            "name--",
            "name/*comment*/",
            "\"name\"",
            "`name`",
            "[name]",
            "users.\"name\"",
            "name'",
            "*",
            "users.*",
            "na\0me",
            "name\n",
            "naïve",
        ] {
            assert_eq!(
                key.to_sql_checked(|_| Some(column)),
                Err(SortSqlError::InvalidColumn {
                    column: column.to_owned(),
                }),
                "accepted invalid mapping {column:?}",
            );
        }

        let sort = SortKeys::new(&[key, SortKey::new(Column::Id, true)]);
        assert_eq!(
            sort.to_sql_checked(|key| match key {
                Column::Name => Some("users.name"),
                _ => Some("(SELECT 1)"),
            }),
            Err(SortSqlError::InvalidColumn {
                column: "(SELECT 1)".to_owned(),
            })
        );
    }

    #[test]
    fn accepts_borrowed_simple_and_qualified_columns() {
        let key = SortKey::<Column>::new(Column::Name, true);
        for column in ["id", "_id2", "People.Name", "schema.people._id_2"] {
            let mapped = column.to_owned();
            assert_eq!(
                key.to_sql_checked(|_| Some(mapped.as_str())).unwrap(),
                format!("{column} DESC")
            );
        }
    }

    #[test]
    fn checked_empty_sort_does_not_call_the_mapping() {
        assert_eq!(
            SortKeys::<String>::empty()
                .to_sql_checked(|_| panic!("empty sort must not call the mapping"))
                .unwrap(),
            ""
        );
    }

    #[test]
    fn raw_sql_rendering_remains_verbatim() {
        let key = SortKey::<String>::new("(SELECT 1)", true);
        assert_eq!(key.to_sql(), "(SELECT 1) DESC");
        let sort = "name,-\"QuotedName\",(SELECT 1)"
            .parse::<SortKeys>()
            .unwrap();
        assert_eq!(
            sort.to_sql(),
            "name ASC, \"QuotedName\" DESC, (SELECT 1) ASC"
        );
    }
}
