use http::HttpRequestStatus;

use super::*;

pub fn user_id() -> String {
    "00000000-0000-0000-0000-000000000000".to_string()
}

#[derive(Debug, Clone)]
pub enum ReadFileStatus {
    Loading,
    Success(File<Vec<u8>>),
    Error,
}

#[derive(Debug, Clone)]
pub struct File<T> {
    pub path: String,
    pub checksum: String, // base64
    pub contents: T,
    pub created_at: u32,
    pub updated_at: u32,
    pub prev_txn_hash: Option<String>,
    pub txn_hash: String, // base64
    pub version: u32,
}
impl<T> File<T> {
    pub fn new(contents: T) -> Self {
        Self {
            path: "".to_string(),
            checksum: "".to_string(),
            contents,
            created_at: 0,
            updated_at: 0,
            prev_txn_hash: None,
            txn_hash: "".to_string(), // base64
            version: 0,
        }
    }

    pub fn with_contents<B>(&self, contents: B) -> File<B> {
        File {
            path: self.path.clone(),
            checksum: self.checksum.clone(),
            contents,
            created_at: self.created_at,
            updated_at: self.updated_at,
            prev_txn_hash: self.prev_txn_hash.clone(),
            txn_hash: self.txn_hash.clone(),
            version: self.version,
        }
    }

    pub fn map_contents<F, B>(&self, f: F) -> File<B>
    where
        F: FnOnce(&T) -> B,
    {
        File {
            path: self.path.clone(),
            checksum: self.checksum.clone(),
            contents: f(&self.contents),
            created_at: self.created_at,
            updated_at: self.updated_at,
            prev_txn_hash: self.prev_txn_hash.clone(),
            txn_hash: self.txn_hash.clone(),
            version: self.version,
        }
    }
}

pub fn read_file(program_id: &str, filepath: &str) -> ReadFileStatus {
    let url = format!("http://localhost:8000/files/{program_id}/{filepath}?stream=true");
    let res = http::get(&url);
    let (_req_id, req_status, response) = res;

    match &response {
        Some(response) if response.body.len() > 0 => {
            let mut file = File::new(vec![]);
            let body = std::str::from_utf8(&response.body[6..]).unwrap();
            let json = json::parse(body);
            let obj = json.as_object().unwrap();

            // crate::println!("{:?}", json);

            // Parse response body
            file.path = filepath.to_string();
            file.checksum = obj.get("checksum").unwrap().as_string().unwrap();
            file.contents = {
                let contents = obj.get("contents").unwrap().as_str().unwrap();
                // log!("{}", contents);
                base64_decode(contents).unwrap()
            };
            file.created_at = obj.get("created_at").unwrap().as_u32().unwrap();
            // file.updated_at = obj.get("updated_at").unwrap().as_u32().unwrap();
            file.prev_txn_hash = obj
                .get("prev_txn_hash")
                .unwrap()
                .as_option(|v| v.as_string())
                .unwrap();
            file.txn_hash = obj.get("txn_hash").unwrap().as_string().unwrap();
            file.version = obj.get("version").unwrap().as_u32().unwrap();

            return ReadFileStatus::Success(file);
        }
        _ => {
            if req_status == HttpRequestStatus::Fail {
                ReadFileStatus::Error
            } else {
                ReadFileStatus::Loading
            }
        }
    }
}

pub fn exec(program_id: &str, command: &str, data: &[u8]) -> String {
    let tx_hash_url_safe_b64 = &mut [0; 44];
    let _req_id = unsafe {
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
            ) -> u64;
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

const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64_decode(encoded: &str) -> Result<Vec<u8>, &'static str> {
    let mut decoded = Vec::new();
    let mut buffer = 0u32;
    let mut bits_collected = 0;

    for &byte in encoded.as_bytes() {
        if byte == b'=' {
            break;
        }

        let value = match BASE64_CHARS.iter().position(|&c| c == byte) {
            Some(v) => v as u32,
            None => return Err("Invalid base64 character"),
        };

        buffer = (buffer << 6) | value;
        bits_collected += 6;

        if bits_collected >= 8 {
            bits_collected -= 8;
            decoded.push((buffer >> bits_collected) as u8 & 0xFF);
        }
    }

    Ok(decoded)
}

#[allow(unused)]
fn base64_url_safe_encode(input: &[u8]) -> String {
    let mut output = Vec::new();
    let mut buffer = 0u32;
    let mut bits_collected = 0;

    for &byte in input {
        buffer = (buffer << 8) | byte as u32;
        bits_collected += 8;

        while bits_collected >= 6 {
            bits_collected -= 6;
            let index = (buffer >> bits_collected) & 0b111111;
            output.push(BASE64_CHARS[index as usize]);
        }
    }

    if bits_collected > 0 {
        buffer <<= 6 - bits_collected;
        let index = buffer & 0b111111;
        output.push(BASE64_CHARS[index as usize]);
    }

    let mut encoded = String::from_utf8(output).expect("Valid UTF-8");

    // Make it URL safe
    encoded = encoded.replace('+', "-").replace('/', "_");

    // Remove padding
    encoded = encoded.trim_end_matches('=').to_string();

    encoded
}
