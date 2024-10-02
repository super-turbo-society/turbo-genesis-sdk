use super::*;
use base64::{engine::general_purpose::STANDARD as b64, Engine};

#[derive(Debug, Clone)]
pub enum ReadFileError {
    Loading,
    NetworkError,
    NotFound,
    ParsingError(String),
}
impl ReadFileError {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
    pub checksum: String, // base64
    pub contents: Vec<u8>,
    pub created_at: u32,
    pub updated_at: u32,
    pub prev_txn_hash: Option<String>,
    pub txn_hash: String, // base64
    pub version: u32,
}
impl File {
    pub fn new(contents: &[u8]) -> Self {
        Self {
            path: "".to_string(),
            checksum: "".to_string(),
            contents: contents.to_vec(),
            created_at: 0,
            updated_at: 0,
            prev_txn_hash: None,
            txn_hash: "".to_string(), // base64
            version: 0,
        }
    }
}

pub fn read_file(program_id: &str, filepath: &str) -> Result<File, ReadFileError> {
    let data = &mut [0; 8192];
    let mut data_len = 0;
    let ok = unsafe {
        #[allow(clashing_extern_declarations)]
        #[link(wasm_import_module = "@turbo_genesis/turbo_os")]
        extern "C" {
            fn read_file(
                program_id_ptr: *const u8,
                program_id_len: u32,
                filepath_ptr: *const u8,
                filepath_len: u32,
                out_data_ptr: *mut u8,
                out_data_len_ptr: *mut u32,
            ) -> u32;
        }
        read_file(
            program_id.as_ptr(),
            program_id.len() as u32,
            filepath.as_ptr(),
            filepath.len() as u32,
            data.as_mut_ptr(),
            &mut data_len,
        )
    };

    // Read file was unsuccessful
    if ok != 0 {
        return Err(ReadFileError::NetworkError);
    }

    // No file data
    if data_len == 0 {
        // TODO: disambiguate NotFound and Loading
        return Err(ReadFileError::NotFound);
    }

    // Read the file data (trim SSE "data: " prefix)
    let body = data.get(..data_len as usize).ok_or_else(|| {
        ReadFileError::ParsingError("Could not read file data response body".to_string())
    })?;

    // Parse response body as JSON
    let json_str =
        std::str::from_utf8(body).map_err(|err| ReadFileError::ParsingError(err.to_string()))?;
    let json_value = json::parse(json_str);
    let json = json_value
        .as_object()
        .map_err(|err| ReadFileError::ParsingError(err.to_string()))?;

    // Parse contents
    let contents_str = json
        .get("contents")
        .ok_or_else(|| ReadFileError::ParsingError("Could not read file contents".to_string()))?
        .as_str()
        .map_err(|err| ReadFileError::ParsingError(err.to_string()))?;
    let contents = b64
        .decode(contents_str)
        .map_err(|err| ReadFileError::ParsingError(err.to_string()))?;

    // Initialize file with contents
    let mut file = File::new(&contents);

    // Set filepath
    file.path = filepath.to_string();

    // Set checksum
    file.checksum = json
        .get("checksum")
        .ok_or_else(|| ReadFileError::ParsingError("Could not read file checksum".to_string()))?
        .as_string()
        .map_err(|err| ReadFileError::ParsingError(err.to_string()))?;

    // Set created at
    file.created_at = json
        .get("created_at")
        .ok_or_else(|| ReadFileError::ParsingError("Could not read file created_at".to_string()))?
        .as_u32()
        .map_err(|err| ReadFileError::ParsingError(err.to_string()))?;

    // Set updated at
    file.updated_at = json
        .get("updated_at")
        .ok_or_else(|| ReadFileError::ParsingError("Could not read file updated_at".to_string()))?
        .as_u32()
        .map_err(|err| ReadFileError::ParsingError(err.to_string()))?;

    // Set previous transaction hash
    file.prev_txn_hash = json
        .get("prev_txn_hash")
        .ok_or_else(|| {
            ReadFileError::ParsingError("Could not read file prev_txn_hash".to_string())
        })?
        .as_option(|a| a.as_string())
        .map_err(|err| ReadFileError::ParsingError(err.to_string()))?;

    // Set transaction hash
    file.txn_hash = json
        .get("txn_hash")
        .ok_or_else(|| ReadFileError::ParsingError("Could not read file txn_hash".to_string()))?
        .as_string()
        .map_err(|err| ReadFileError::ParsingError(err.to_string()))?;

    // Set version
    file.version = json
        .get("version")
        .ok_or_else(|| ReadFileError::ParsingError("Could not read file version".to_string()))?
        .as_u32()
        .map_err(|err| ReadFileError::ParsingError(err.to_string()))?;

    return Ok(file);
}

pub fn exec(program_id: &str, command: &str, data: &[u8]) -> String {
    let tx_hash_url_safe_b64 = &mut [0; 43]; // url-safe, no-pad
    let _ok = unsafe {
        #[link(wasm_import_module = "@turbo_genesis/turbo_os")]
        extern "C" {
            fn exec(
                program_id_ptr: *const u8,
                program_id_len: u32,
                command_ptr: *const u8,
                command_len: u32,
                data_ptr: *const u8,
                data_len: u32,
                tx_hash_ptr: *mut u8,
            ) -> u32;
        }
        exec(
            program_id.as_ptr(),
            program_id.len() as u32,
            command.as_ptr(),
            command.len() as u32,
            data.as_ptr(),
            data.len() as u32,
            tx_hash_url_safe_b64.as_mut_ptr(),
        )
    };

    std::str::from_utf8(tx_hash_url_safe_b64)
        .unwrap()
        .to_string()
}

pub fn user_id() -> Option<String> {
    let data = &mut [0; 128];
    let mut data_len = 0;
    let ok = unsafe {
        #[allow(clashing_extern_declarations)]
        #[link(wasm_import_module = "@turbo_genesis/turbo_os")]
        extern "C" {
            fn get_user_id(out_user_id_ptr: *mut u8, out_user_id_len_ptr: *mut u32) -> u32;
        }
        get_user_id(data.as_mut_ptr(), &mut data_len)
    };
    if ok == 0 {
        String::from_utf8(data[..data_len as usize].to_vec()).ok()
    } else {
        None
    }
}

// const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

// #[allow(unused)]
// fn base64_url_safe_encode(input: &[u8]) -> String {
//     let mut output = Vec::new();
//     let mut buffer = 0u32;
//     let mut bits_collected = 0;

//     for &byte in input {
//         buffer = (buffer << 8) | byte as u32;
//         bits_collected += 8;

//         while bits_collected >= 6 {
//             bits_collected -= 6;
//             let index = (buffer >> bits_collected) & 0b111111;
//             output.push(BASE64_CHARS[index as usize]);
//         }
//     }

//     if bits_collected > 0 {
//         buffer <<= 6 - bits_collected;
//         let index = buffer & 0b111111;
//         output.push(BASE64_CHARS[index as usize]);
//     }

//     let mut encoded = String::from_utf8(output).expect("Valid UTF-8");

//     // Make it URL safe
//     encoded = encoded.replace('+', "-").replace('/', "_");

//     // Remove padding
//     encoded = encoded.trim_end_matches('=').to_string();

//     encoded
// }
