//! This module contains the implementation of the `DocumentQueryResult` struct,
//! which is used to encapsulate the result of a query for a document in the system.

use super::*;
use borsh::{BorshDeserialize, BorshSerialize};
use encoding::b64::serde_utils::*;
use fs::ProgramFile;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod channel;
pub mod command;
pub mod fs;

/// Returns the current user's ID if authenticated.
///
/// It fills a buffer via FFI and returns the UTF-8 decoded string on success.
pub fn user_id() -> Option<String> {
    let data = &mut [0; 128]; // Buffer for user ID bytes
    let mut data_len = 0;
    let ok = turbo_genesis_ffi::os::client::get_user_id(data.as_mut_ptr(), &mut data_len);
    if ok == 0 {
        // Safe UTF-8 conversion of the returned bytes
        String::from_utf8(data[..data_len as usize].to_vec()).ok()
    } else {
        None
    }
}

/// Represents a program event returned by the host.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramEvent {
    /// Unique event identifier.
    pub id: String,

    /// Timestamp of creation (Unix epoch seconds).
    pub created_at: u32,

    /// The associated program's ID.
    pub program_id: String,

    /// Transaction hash of the event (base64).
    pub tx_hash: String,

    /// Event kind or type.
    #[serde(rename = "type")]
    pub kind: String,

    /// Binary payload, base64 encoded in JSON.
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub data: Vec<u8>,
}

/// Watches events for a given program, optionally filtering by event type.
///
/// Returns a `QueryResult<ProgramEvent>` capturing loading state, data, and errors.
pub fn watch_events(program_id: &str, event_type: Option<&str>) -> QueryResult<ProgramEvent> {
    const STATUS_PENDING: u32 = 1;
    const STATUS_FAILED: u32 = 2;

    let data = &mut [0; 4096]; // Buffer for serialized event data
    let mut data_len = 0;
    let err = &mut [0; 1024]; // Buffer for error message
    let mut err_len = 0;

    let event_type = event_type.unwrap_or("");
    let status = turbo_genesis_ffi::os::client::watch_events(
        program_id.as_ptr(),
        program_id.len() as u32,
        event_type.as_ptr(),
        event_type.len() as u32,
        data.as_mut_ptr(),
        &mut data_len,
        err.as_mut_ptr(),
        &mut err_len,
    );

    // Handle hard error (e.g. network failure)
    if status == STATUS_FAILED {
        return QueryResult {
            loading: false,
            data: None,
            error: Some("NetworkError".to_string()),
        };
    }

    // Prepare loading or complete result
    let mut res = QueryResult {
        loading: status == STATUS_PENDING,
        data: None,
        error: None,
    };

    // Attempt to decode event payload
    if data_len > 0 {
        if let Some(bytes) = data.get(..data_len as usize) {
            match serde_json::from_slice::<ProgramEvent>(bytes) {
                Ok(event) => res.data = Some(event),
                Err(err) => res.error = Some(err.to_string()),
            }
        }
    }

    // Attempt to decode error payload
    if err_len > 0 {
        if let Some(bytes) = err.get(..err_len as usize) {
            res.error = Some(String::from_utf8_lossy(bytes).to_string())
        }
    }

    res
}

/// Generic wrapper for query results, capturing loading state, data, and errors.
#[derive(Debug, Clone)]
pub struct QueryResult<T> {
    /// Indicates whether the query is still in progress.
    pub loading: bool,

    /// The optional successful result data.
    pub data: Option<T>,

    /// The optional error message if the query failed.
    pub error: Option<String>,
}

impl<T> QueryResult<T> {
    /// Creates an empty `QueryResult` with default values.
    pub fn new() -> Self {
        Self {
            loading: false,
            data: None,
            error: None,
        }
    }
}

/// Extension for `QueryResult<ProgramFile>` to deserialize the file contents.
impl QueryResult<ProgramFile> {
    /// Deserialize the embedded `ProgramFile.contents` into any `U: BorshDeserialize`.
    pub fn parse<U: BorshDeserialize>(&self) -> Option<U> {
        let pf = self.data.as_ref()?; // Extract ProgramFile
        U::try_from_slice(&pf.contents).ok() // Deserialize contents
    }
}

/// Trait for types that can be watched as documents.
pub trait WatchDocument<T> {
    /// Starts watching the given path and returns a `DocumentQueryResult<T>`.
    fn watch(path: impl AsRef<std::path::Path>) -> DocumentQueryResult<T>;
}

/// Default `WatchDocument` impl for any `T` that implements `BorshDeserialize` and `HasProgramId`.
impl<T: BorshDeserialize + HasProgramId> WatchDocument<T> for T {
    fn watch(path: impl AsRef<std::path::Path>) -> DocumentQueryResult<T> {
        // Build the full path based on the program's ID constant.
        let path = std::path::Path::new(T::PROGRAM_ID).join(path.as_ref());
        let result = fs::watch(&path);
        DocumentQueryResult::new(path, result)
    }
}

/// Represents the result of a document query, including path, result, and type info.
#[derive(Debug, Clone)]
pub struct DocumentQueryResult<T> {
    /// The file system path to the document being queried.
    #[allow(unused)]
    path: PathBuf,

    /// The raw query result (loading, data, error).
    result: QueryResult<ProgramFile>,

    /// Phantom data to tie this result to a specific type `T`.
    _phantom: std::marker::PhantomData<T>,
}

impl<T> DocumentQueryResult<T> {
    /// Creates a new `DocumentQueryResult`.
    ///
    /// `path` is the file path to the document, `result` is the raw query result.
    pub fn new(path: impl AsRef<std::path::Path>, result: QueryResult<ProgramFile>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            result,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Retrieves a reference to the `ProgramFile` if available.
    pub fn data(&self) -> Option<&ProgramFile> {
        self.result.data.as_ref()
    }

    /// Returns `true` if the document is still loading.
    pub fn loading(&self) -> bool {
        self.result.loading
    }

    /// Returns an optional reference to the error message.
    pub fn error(&self) -> Option<&String> {
        self.result.error.as_ref()
    }
}

impl<T: BorshDeserialize> DocumentQueryResult<T> {
    /// Consumes the wrapper and parses the document contents into `T`.
    pub fn parse(self) -> Option<T> {
        let pf = self.result.data?; // Get ProgramFile
        T::try_from_slice(&pf.contents).ok() // Deserialize into T
    }
}
