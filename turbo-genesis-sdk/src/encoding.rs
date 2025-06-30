/// A utility module for working with base64-encoded data.
pub mod b64 {
    use base64::{
        engine::general_purpose::{STANDARD as b64, URL_SAFE_NO_PAD as b64_url_safe},
        Engine,
    };

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
