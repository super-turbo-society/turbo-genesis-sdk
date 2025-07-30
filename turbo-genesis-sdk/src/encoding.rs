//! Encoding Utility Module
//!
//! This module currently offers comprehensive Base64 support, including:
//!
//! - Serde adapters for automatically (de)serializing fields as Base64 strings.
//! - Standard Base64 encoding/decoding (with ‘+’, ‘/’, and padding).
//! - URL-safe Base64 encoding/decoding (with ‘-’, ‘_’, no padding).
//!
//! Submodules:
//! - `serde_utils`: Serde `Serializer`/`Deserializer` functions for Base64.
//! - `standard`: Convenient functions `encode` and `decode_base64` using the standard alphabet.
//! - `url_safe`: Convenient functions `encode` and `decode` using the URL-safe, no-pad alphabet.

pub mod b64 {
    use base64::{
        engine::general_purpose::{STANDARD as b64, URL_SAFE_NO_PAD as b64_url_safe},
        Engine,
    };

    /// Serde utilities for serializing and deserializing base64 data.
    pub mod serde_utils {
        use super::*;
        use serde::{Deserialize, Deserializer, Serializer};

        /// Deserializes a base64-encoded string into a `Vec<u8>`.
        /// Returns an error if the string is not valid base64.
        pub fn from_base64<'a, D: Deserializer<'a>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
            use serde::de::Error;
            <String as Deserialize>::deserialize(deserializer).and_then(|string| {
                b64.decode(&string)
                    .map_err(|err| Error::custom(err.to_string()))
            })
        }

        /// Serializes a byte slice as a base64-encoded string.
        pub fn as_base64<T: AsRef<[u8]>, S: Serializer>(
            v: &T,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_str(&b64.encode(v.as_ref()))
        }
    }

    /// Standard base64 encoding/decoding (with padding, + and /).
    pub mod standard {
        use super::*;
        /// Encodes bytes into a standard base64 string.
        /// Uses the standard alphabet with `+` and `/`, and padding `=`.
        pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
            b64.encode(input)
        }
        /// Decodes a standard base64-encoded string or byte slice.
        /// Returns an error if the input is invalid.
        pub fn decode_base64<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, base64::DecodeError> {
            b64.decode(input)
        }
    }

    /// URL-safe base64 encoding/decoding (no padding, - and _).
    pub mod url_safe {
        use super::*;
        /// Encodes bytes into a URL-safe base64 string.
        /// Uses `-` and `_` instead of `+` and `/`, and omits padding.
        pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
            b64_url_safe.encode(input)
        }

        /// Decodes a URL-safe base64-encoded string or byte slice.
        /// Returns an error if the input is invalid.
        pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, base64::DecodeError> {
            b64_url_safe.decode(input)
        }
    }
}
