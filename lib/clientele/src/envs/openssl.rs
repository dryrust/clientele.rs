// This is free and unencumbered software released into the public domain.

use super::var;

/// See: https://github.com/rustls/rustls-native-certs/issues/16
pub fn ssl_cert_file() -> Option<String> {
    var("SSL_CERT_FILE")
}

/// See: https://github.com/rustls/rustls-native-certs/issues/16
pub fn ssl_cert_dir() -> Option<String> {
    var("SSL_CERT_DIR")
}
