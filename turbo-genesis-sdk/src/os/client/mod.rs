use borsh::{BorshDeserialize, BorshSerialize};

use super::*;

pub mod channel;
pub mod command;
pub mod fs;

/// Returns the current user's ID if authenticated
pub fn user_id() -> Option<String> {
    let data = &mut [0; 128];
    let mut data_len = 0;
    let ok = turbo_genesis_ffi::os::client::get_user_id(data.as_mut_ptr(), &mut data_len);
    if ok == 0 {
        String::from_utf8(data[..data_len as usize].to_vec()).ok()
    } else {
        None
    }
}

/// QueryResult wrapper around a single `ProgramEvent` response,
/// optionally filtered by `event_type`.
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

    // Prepare a loading or complete result
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

#[derive(Debug, Clone)]
pub struct QueryResult<T> {
    pub loading: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> QueryResult<T> {
    pub fn new() -> Self {
        Self {
            loading: false,
            data: None,
            error: None,
        }
    }
}
impl QueryResult<client::fs::ProgramFile> {
    pub fn parse<T: BorshDeserialize /* <- ideally, Borsh + Json */>(&self) -> Option<T> {
        let Some(data) = &self.data else {
            return None;
        };
        T::try_from_slice(&data.contents).ok()
    }
}

fn from_base64<'a, D: Deserializer<'a>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    use serde::de::Error;
    <String as Deserialize>::deserialize(deserializer).and_then(|string| {
        b64.decode(&string)
            .map_err(|err| Error::custom(err.to_string()))
    })
}

fn as_base64<T: AsRef<[u8]>, S: Serializer>(v: &T, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&b64.encode(v.as_ref()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramEvent {
    pub id: String,
    pub created_at: u32,
    pub program_id: String,
    pub tx_hash: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub data: Vec<u8>,
}
