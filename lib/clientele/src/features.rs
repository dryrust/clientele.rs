// This is free and unencumbered software released into the public domain.

/// The set of features that are enabled in this build of the crate.
pub static FEATURES: &[&str] = &[
    #[cfg(feature = "argfile")]
    "argfile",
    #[cfg(feature = "camino")]
    "camino",
    #[cfg(feature = "clap")]
    "clap",
    #[cfg(feature = "color")]
    "color",
    #[cfg(feature = "dirs")]
    "dirs",
    #[cfg(feature = "dotenv")]
    "dotenv",
    #[cfg(feature = "error-stack")]
    "error-stack",
    #[cfg(feature = "parse")]
    "parse",
    #[cfg(feature = "parse-byteunit")]
    "parse-byteunit",
    #[cfg(feature = "parse-datetime")]
    "parse-datetime",
    #[cfg(feature = "parse-duration")]
    "parse-duration",
    #[cfg(feature = "serde")]
    "serde",
    #[cfg(feature = "serde-json")]
    "serde-json",
    #[cfg(feature = "subcommands")]
    "subcommands",
    #[cfg(feature = "tokio")]
    "tokio",
    #[cfg(feature = "tracing")]
    "tracing",
    #[cfg(feature = "unicode")]
    "unicode",
    #[cfg(feature = "unstable")]
    "unstable",
    #[cfg(feature = "wild")]
    "wild",
];
