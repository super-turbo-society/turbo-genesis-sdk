use super::*;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramFile {
    pub checksum: String, // base64
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub contents: Vec<u8>,
    pub created_at: u32,
    pub updated_at: u32,
    pub prev_txn_hash: Option<String>,
    pub txn_hash: String, // base64
    pub version: u32,
}

/// Watch a specific file (streaming by default)
pub fn watch<P: AsRef<Path>>(path: P) -> QueryResult<ProgramFile> {
    read_with_opts(path, &[("stream", "true")])
}

/// Watch file with custom query string options
pub fn read_with_opts<P: AsRef<Path>, K: std::fmt::Display, V: std::fmt::Display>(
    path: P,
    opts: &[(K, V)],
) -> QueryResult<ProgramFile> {
    let query = opts
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");

    const STATUS_PENDING: u32 = 1;
    const STATUS_FAILED: u32 = 2;

    let path = path.as_ref();
    let program_id = path
        .iter()
        .next()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("");
    let filepath = path.iter().skip(1).collect::<PathBuf>();
    let filepath_str = filepath.to_string_lossy();

    let data = &mut [0; 8192];
    let mut data_len = 0;
    let err = &mut [0; 1024];
    let mut err_len = 0;

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

    let mut res = QueryResult {
        loading: status == STATUS_PENDING,
        data: None,
        error: None,
    };

    if status == STATUS_FAILED {
        res.error = Some("NetworkError".into());
        return res;
    }

    if data_len > 0 {
        if let Some(bytes) = data.get(..data_len as usize) {
            match serde_json::from_slice::<ProgramFile>(bytes) {
                Ok(event) => res.data = Some(event),
                Err(err) => res.error = Some(err.to_string()),
            }
        }
    }

    if err_len > 0 {
        if let Some(bytes) = err.get(..err_len as usize) {
            res.error = Some(String::from_utf8_lossy(bytes).to_string())
        }
    }

    res
}
