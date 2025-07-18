//! This module provides functionality for interacting with the filesystem on the client side.
//! It includes utilities for reading and watching files, as well as deserializing file data
//! into the `ProgramFile` structure. The module leverages FFI (Foreign Function Interface)
//! to communicate with the underlying Turbo Genesis OS client.

use super::*;
use std::path::{Path, PathBuf};

/// Represents a file in the program with metadata and contents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramFile {
    /// A base64-encoded checksum of the file contents.
    pub checksum: String, // base64
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    /// The raw binary contents of the file, serialized/deserialized as base64.
    pub contents: Vec<u8>,
    /// The timestamp (in seconds since epoch) when the file was created.
    pub created_at: u32,
    /// The timestamp (in seconds since epoch) when the file was last updated.
    pub updated_at: u32,
    /// An optional base64-encoded hash of the previous transaction.
    pub prev_txn_hash: Option<String>,
    /// A base64-encoded hash of the current transaction.
    pub txn_hash: String, // base64
    /// The version number of the file.
    pub version: u32,
}

/// Watches a specific file for changes. By default, streaming is enabled.
///
/// # Arguments
/// - `path`: The path to the file to watch.
///
/// # Returns
/// A `QueryResult` containing the `ProgramFile` data or an error.
pub fn watch<P: AsRef<Path>>(path: P) -> QueryResult<ProgramFile> {
    read_with_opts(path, &[("stream", "true")])
}

/// Reads a file with custom query string options.
///
/// # Arguments
/// - `path`: The path to the file to read.
/// - `opts`: A slice of key-value pairs representing query string options.
///
/// # Returns
/// A `QueryResult` containing the `ProgramFile` data or an error.
pub fn read_with_opts<P: AsRef<Path>, K: std::fmt::Display, V: std::fmt::Display>(
    path: P,
    opts: &[(K, V)],
) -> QueryResult<ProgramFile> {
    // Construct the query string from the provided options.
    let query = opts
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");

    // Status codes returned by the FFI function.
    const STATUS_PENDING: u32 = 1;
    const STATUS_FAILED: u32 = 2;

    // Extract the program ID and file path from the provided path.
    let path = path.as_ref();
    let program_id = path
        .iter()
        .next()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("");
    let filepath = path.iter().skip(1).collect::<PathBuf>();
    let filepath_str = filepath.to_string_lossy();

    // Buffers for storing data and error messages returned by the FFI function.
    let data = &mut [0; 8192];
    let mut data_len = 0;
    let err = &mut [0; 1024];
    let mut err_len = 0;

    // Call the FFI function to read the file.
    let status = turbo_genesis_ffi::os::client::read_file(
        program_id.as_ptr(),
        program_id.len() as u32,
        filepath_str.as_ptr(),
        filepath_str.len() as u32,
        query.as_ptr(),
        query.len() as u32,
        data.as_mut_ptr(),
        &mut data_len,
        err.as_mut_ptr(),
        &mut err_len,
    );

    // Initialize the result object.
    let mut res = QueryResult {
        loading: status == STATUS_PENDING,
        data: None,
        error: None,
    };

    // Handle the case where the FFI function indicates a failure.
    if status == STATUS_FAILED {
        res.error = Some("NetworkError".into());
        return res;
    }

    // If data was returned, attempt to deserialize it into a `ProgramFile`.
    if data_len > 0 {
        if let Some(bytes) = data.get(..data_len as usize) {
            match serde_json::from_slice::<ProgramFile>(bytes) {
                Ok(event) => res.data = Some(event),
                Err(err) => res.error = Some(err.to_string()),
            }
        }
    }

    // If an error message was returned, decode it and store it in the result.
    if err_len > 0 {
        if let Some(bytes) = err.get(..err_len as usize) {
            res.error = Some(String::from_utf8_lossy(bytes).to_string())
        }
    }

    res
}
