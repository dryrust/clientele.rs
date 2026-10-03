// This is free and unencumbered software released into the public domain.

#[cfg(feature = "camino")]
#[test]
fn camino_supports_serde_without_json() {
    use clientele::crates::serde::{de::value, Deserialize, Serialize};

    fn assert_serializable<T: Serialize>() {}
    assert_serializable::<clientele::Utf8PathBuf>();
    let input = value::StrDeserializer::<value::Error>::new("settings/config.toml");
    let path = clientele::Utf8PathBuf::deserialize(input).unwrap();
    assert_eq!(path, "settings/config.toml");
}

#[cfg(feature = "serde-json")]
#[test]
fn json_errors_keep_their_sysexits_integration() {
    use clientele::{crates::serde_json, SysexitsError};

    let error = serde_json::from_str::<bool>("invalid JSON").unwrap_err();
    assert_eq!(SysexitsError::from(&error), SysexitsError::EX_DATAERR);
    assert_eq!(SysexitsError::from(error), SysexitsError::EX_DATAERR);
}
