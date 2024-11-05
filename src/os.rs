use super::*;
use base64::{engine::general_purpose::STANDARD as b64, Engine};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

fn from_base64<'a, D: Deserializer<'a>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    use serde::de::Error;
    String::deserialize(deserializer).and_then(|string| {
        b64.decode(&string)
            .map_err(|err| Error::custom(err.to_string()))
    })
}

fn as_base64<T: AsRef<[u8]>, S: Serializer>(v: &T, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&b64.encode(v.as_ref()))
}

#[derive(Debug, Clone)]
pub struct QueryResult<T> {
    pub loading: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[deprecated(note = "newer methods use `std::io::Error` instead")]
#[derive(Debug, Clone)]
pub enum ReadError {
    Loading,
    NetworkError,
    NotFound,
    ParsingError(String),
}
impl ReadError {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub type ReadFileError = ReadError;

#[deprecated(note = "newer methods use `ProgramFile` instead")]
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
    // type Error = ReadError;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramEvent {
    pub id: String,
    pub created_at: u64,
    pub program_id: String,
    pub tx_hash: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub data: Vec<u8>,
}

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

pub mod client {
    use super::*;

    pub fn watch_events(program_id: &str, event_type: Option<&str>) -> QueryResult<ProgramEvent> {
        // const STATUS_COMPLETE: u32 = 0;
        const STATUS_PENDING: u32 = 1;
        const STATUS_FAILED: u32 = 2;

        let data = &mut [0; 1024];
        let mut data_len = 0;
        let err = &mut [0; 1024];
        let mut err_len = 0;

        let event_type = event_type.unwrap_or("");
        let status = unsafe {
            #[allow(clashing_extern_declarations)]
            #[link(wasm_import_module = "@turbo_genesis/turbo_os")]
            extern "C" {
                fn watch_events(
                    program_id_ptr: *const u8,
                    program_id_len: u32,
                    event_type_ptr: *const u8,
                    event_type_len: u32,
                    out_data_ptr: *const u8,
                    out_data_len_ptr: *mut u32,
                    out_err_ptr: *const u8,
                    out_err_len_ptr: *mut u32,
                ) -> u32;
            }
            watch_events(
                program_id.as_ptr(),
                program_id.len() as u32,
                event_type.as_ptr(),
                event_type.len() as u32,
                data.as_mut_ptr(),
                &mut data_len,
                err.as_mut_ptr(),
                &mut err_len,
            )
        };

        // Network error
        if status == STATUS_FAILED {
            return QueryResult {
                loading: false,
                data: None,
                error: Some("NetworkError".to_string()),
            };
        }

        // Request is loading or complete
        let mut res = QueryResult {
            loading: status == STATUS_PENDING,
            data: None,
            error: None,
        };

        // Parse data into program event
        if data_len > 0 {
            if let Some(bytes) = data.get(..data_len as usize) {
                match serde_json::from_slice::<ProgramEvent>(bytes) {
                    Ok(event) => res.data = Some(event),
                    Err(err) => res.error = Some(err.to_string()),
                }
            }
        }

        // Parse err into error string
        if err_len > 0 {
            if let Some(bytes) = err.get(..err_len as usize) {
                res.error = Some(String::from_utf8_lossy(bytes).to_string())
            }
        }

        res
    }

    pub fn watch_file(program_id: &str, filepath: &str) -> QueryResult<ProgramFile> {
        // const STATUS_COMPLETE: u32 = 0;
        const STATUS_PENDING: u32 = 1;
        const STATUS_FAILED: u32 = 2;
        let data = &mut [0; 8192];
        let mut data_len = 0;
        let err = &mut [0; 1024];
        let mut err_len = 0;
        let status = unsafe {
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
                    out_err_ptr: *mut u8,
                    out_err_len_ptr: *mut u32,
                ) -> u32;
            }
            read_file(
                program_id.as_ptr(),
                program_id.len() as u32,
                filepath.as_ptr(),
                filepath.len() as u32,
                data.as_mut_ptr(),
                &mut data_len,
                err.as_mut_ptr(),
                &mut err_len,
            )
        };
        // Network error
        if status == STATUS_FAILED {
            return QueryResult {
                loading: false,
                data: None,
                error: Some("NetworkError".to_string()),
            };
        }

        // Request is loading or complete
        let mut res = QueryResult {
            loading: status == STATUS_PENDING,
            data: None,
            error: None,
        };

        // Parse data into program event
        if data_len > 0 {
            if let Some(bytes) = data.get(..data_len as usize) {
                match serde_json::from_slice::<ProgramFile>(bytes) {
                    Ok(event) => res.data = Some(event),
                    Err(err) => res.error = Some(err.to_string()),
                }
            }
        }

        // Parse err into error string
        if err_len > 0 {
            if let Some(bytes) = err.get(..err_len as usize) {
                res.error = Some(String::from_utf8_lossy(bytes).to_string())
            }
        }

        res
    }

    #[deprecated(note = "please use `watch_file` instead")]
    pub fn read_file(program_id: &str, filepath: &str) -> Result<File, ReadError> {
        let data = &mut [0; 8192];
        let mut data_len = 0;
        let err = &mut [0; 1024];
        let mut err_len = 0;
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
                    out_err_ptr: *mut u8,
                    out_err_len_ptr: *mut u32,
                ) -> u32;
            }
            read_file(
                program_id.as_ptr(),
                program_id.len() as u32,
                filepath.as_ptr(),
                filepath.len() as u32,
                data.as_mut_ptr(),
                &mut data_len,
                err.as_mut_ptr(),
                &mut err_len,
            )
        };

        // Read file was unsuccessful
        if ok != 0 {
            return Err(ReadError::NetworkError);
        }

        // No file data
        if data_len == 0 {
            // TODO: disambiguate NotFound and Loading
            return Err(ReadError::NotFound);
        }

        // Read the file data
        let body = data.get(..data_len as usize).ok_or_else(|| {
            ReadError::ParsingError("Could not read file data response body".to_string())
        })?;

        // Parse response body as JSON
        let json_str =
            std::str::from_utf8(body).map_err(|err| ReadError::ParsingError(err.to_string()))?;
        let json_value = json::parse(json_str);
        let json = json_value
            .as_object()
            .map_err(|err| ReadError::ParsingError(err.to_string()))?;

        // Parse contents
        let contents_str = json
            .get("contents")
            .ok_or_else(|| ReadError::ParsingError("Could not read file contents".to_string()))?
            .as_str()
            .map_err(|err| ReadError::ParsingError(err.to_string()))?;
        let contents = b64
            .decode(contents_str)
            .map_err(|err| ReadError::ParsingError(err.to_string()))?;

        // Initialize file with contents
        let mut file = File::new(&contents);

        // Set filepath
        file.path = filepath.to_string();

        // Set checksum
        file.checksum = json
            .get("checksum")
            .ok_or_else(|| ReadError::ParsingError("Could not read file checksum".to_string()))?
            .as_string()
            .map_err(|err| ReadError::ParsingError(err.to_string()))?;

        // Set created at
        file.created_at = json
            .get("created_at")
            .ok_or_else(|| ReadError::ParsingError("Could not read file created_at".to_string()))?
            .as_u32()
            .map_err(|err| ReadError::ParsingError(err.to_string()))?;

        // Set updated at
        file.updated_at = json
            .get("updated_at")
            .ok_or_else(|| ReadError::ParsingError("Could not read file updated_at".to_string()))?
            .as_u32()
            .map_err(|err| ReadError::ParsingError(err.to_string()))?;

        // Set previous transaction hash
        file.prev_txn_hash = json
            .get("prev_txn_hash")
            .ok_or_else(|| {
                ReadError::ParsingError("Could not read file prev_txn_hash".to_string())
            })?
            .as_option(|a| a.as_string())
            .map_err(|err| ReadError::ParsingError(err.to_string()))?;

        // Set transaction hash
        file.txn_hash = json
            .get("txn_hash")
            .ok_or_else(|| ReadError::ParsingError("Could not read file txn_hash".to_string()))?
            .as_string()
            .map_err(|err| ReadError::ParsingError(err.to_string()))?;

        // Set version
        file.version = json
            .get("version")
            .ok_or_else(|| ReadError::ParsingError("Could not read file version".to_string()))?
            .as_u32()
            .map_err(|err| ReadError::ParsingError(err.to_string()))?;

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
}

pub mod server {
    use borsh::{BorshDeserialize, BorshSerialize};

    use super::*;

    #[link(wasm_import_module = "turbo")]
    extern "C" {
        #[link_name = "random_bytes"]
        fn turbo_os_random_bytes(ptr: *mut u8, len: usize) -> usize;

        #[link_name = "get_user_id_len"]
        fn turbo_os_get_user_id_len() -> usize;

        #[allow(clashing_extern_declarations)]
        #[link_name = "get_user_id"]
        fn turbo_os_get_user_id(ptr: *mut u8) -> usize;

        #[link_name = "get_input_data_len"]
        fn turbo_os_get_input_data_len() -> usize;

        #[link_name = "get_input_data"]
        fn turbo_os_get_input_data(ptr: *mut u8) -> usize;

        #[link_name = "log"]
        fn turbo_os_log(ptr: *const u8, len: usize) -> usize;

        #[allow(clashing_extern_declarations)]
        #[link_name = "read_file"]
        fn turbo_os_read_file(
            filepath_ptr: *const u8,
            filepath_len: usize,
            data_ptr: *mut u8,
            data_len: *mut usize,
        ) -> usize;

        #[link_name = "write_file"]
        fn turbo_os_write_file(
            filepath_ptr: *const u8,
            filepath_len: usize,
            data_ptr: *const u8,
            data_len: usize,
        ) -> usize;

        #[link_name = "emit_event"]
        fn turbo_os_emit_event(
            type_ptr: *const u8,
            data_len: usize,
            data_ptr: *const u8,
            data_len: usize,
        ) -> usize;
    }

    pub const COMMIT: usize = 0;

    pub const CANCEL: usize = 1;

    pub trait AutoDeserialize: Sized {
        fn auto_deserialize(data: &[u8]) -> Result<Self, std::io::Error>;
    }
    pub trait AutoSerialize: Sized {
        fn auto_serialize(&self) -> Result<Vec<u8>, std::io::Error>;
    }

    // Implement for Borsh
    impl<T> AutoDeserialize for T
    where
        T: BorshDeserialize,
    {
        fn auto_deserialize(data: &[u8]) -> Result<Self, std::io::Error> {
            T::try_from_slice(data).map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Failed Borsh deserialization",
                )
            })
        }
    }
    // Implement for Borsh
    impl<T> AutoSerialize for T
    where
        T: BorshSerialize,
    {
        fn auto_serialize(&self) -> Result<Vec<u8>, std::io::Error> {
            self.try_to_vec().map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Failed Borsh serialization",
                )
            })
        }
    }

    // // Implement for JSON
    // impl<T> AutoDeserialize for T
    // where
    //     T: DeserializeOwned,
    // {
    //     fn deserialize(data: &[u8]) -> Result<Self, std::io::Error> {
    //         serde_json::from_slice(data).map_err(|_| {
    //             std::io::Error::new(
    //                 std::io::ErrorKind::InvalidData,
    //                 "Failed JSON deserialization",
    //             )
    //         })
    //     }
    // }

    #[macro_export]
    macro_rules! os_server_command {
        ($t:ty) => {{
            let input = $crate::os::server::get_command_data();
            match <$t as $crate::os::server::AutoDeserialize>::auto_deserialize(&input) {
                Ok(cmd) => cmd,
                Err(err) => {
                    $crate::os::server::log(&format!("Failed to parse command data: {:?}", err));
                    return $crate::os::server::CANCEL;
                }
            }
        }};
    }
    pub use os_server_command as command;

    #[macro_export]
    macro_rules! os_server_read {
        ($t:ty, $filepath:expr) => {{
            let data = match $crate::os::server::read_file($filepath) {
                Ok(data) => data,
                Err(err) => {
                    $crate::os::server::log(&format!("Failed to read file data: {:?}", err));
                    return $crate::os::server::CANCEL;
                }
            };
            match <$t as $crate::os::server::AutoDeserialize>::auto_deserialize(&data) {
                Ok(data) => data,
                Err(err) => {
                    $crate::os::server::log(&format!("Failed to parse file data: {:?}", err));
                    return $crate::os::server::CANCEL;
                }
            }
        }};
    }
    pub use os_server_read as read;

    #[macro_export]
    macro_rules! os_server_read_or {
        ($t:ty, $filepath:expr, $default:expr) => {{
            $crate::os::server::read_file($filepath)
                .and_then(|data| {
                    <$t as $crate::os::server::AutoDeserialize>::auto_deserialize(&data)
                })
                .unwrap_or($default)
        }};
    }
    pub use os_server_read_or as read_or;

    #[macro_export]
    macro_rules! os_server_read_or_else {
        ($t:ty, $filepath:expr, $cb:expr) => {{
            $crate::os::server::read_file($filepath)
                .and_then(|data| {
                    <$t as $crate::os::server::AutoDeserialize>::auto_deserialize(&data)
                })
                .unwrap_or_else($cb)
        }};
    }
    pub use os_server_read_or_else as read_or_else;

    #[macro_export]
    macro_rules! os_server_read_else {
        ($t:ty, $filepath:expr, $cb:expr) => {{
            match $crate::os::server::read_file($filepath).and_then(|data| {
                <$t as $crate::os::server::AutoDeserialize>::auto_deserialize(&data)
            }) {
                Ok(a) => a,
                Err(err) => $cb,
            }
        }};
    }
    pub use os_server_read_else as read_else;

    #[macro_export]
    macro_rules! os_server_write {
        ($filepath:expr, $data:expr) => {{
            use $crate::os::server::AutoSerialize;
            $data
                .auto_serialize()
                .and_then(|data| $crate::os::server::write_file($filepath, &data))
        }};
    }
    pub use os_server_write as write;

    #[macro_export]
    macro_rules! os_server_log {
        ($($arg:tt)*) => {{
            let message = format!($($arg)*);
            $crate::os::server::log(&message);
        }};
    }
    pub use os_server_log as log;

    #[macro_export]
    macro_rules! os_server_alert {
        ($($arg:tt)*) => {{
            let message = format!($($arg)*);
            let bytes = message.as_bytes();
            $crate::os::server::emit("alert", bytes);
        }};
    }
    pub use os_server_alert as alert;

    pub fn get_user_id() -> String {
        let mut user_id = vec![0; unsafe { turbo_os_get_user_id_len() }];
        unsafe { turbo_os_get_user_id(user_id.as_mut_ptr()) };
        String::from_utf8(user_id).expect("Invalid UTF-8 sequence")
    }

    pub fn get_command_data() -> Vec<u8> {
        let mut input = vec![0; unsafe { turbo_os_get_input_data_len() }];
        unsafe { turbo_os_get_input_data(input.as_mut_ptr()) };
        input
    }

    pub fn parse_command_data<T: BorshDeserialize>() -> Result<T, std::io::Error> {
        let mut input = vec![0; unsafe { turbo_os_get_input_data_len() }];
        unsafe { turbo_os_get_input_data(input.as_mut_ptr()) };
        T::try_from_slice(&input)
    }

    #[deprecated]
    pub fn get_input_data() -> Vec<u8> {
        get_command_data()
    }

    pub fn log(message: &str) {
        unsafe { turbo_os_log(message.as_ptr(), message.len()) };
    }

    pub fn emit(event_type: &str, data: &[u8]) {
        unsafe {
            turbo_os_emit_event(
                event_type.as_ptr(),
                event_type.len(),
                data.as_ptr(),
                data.len(),
            )
        };
    }

    pub fn read_file_(filepath: &str) -> Result<Vec<u8>, &'static str> {
        let mut data = vec![0; 8192];
        let mut data_len = 0;
        let err = unsafe {
            turbo_os_read_file(
                filepath.as_ptr(),
                filepath.len(),
                data.as_mut_ptr(),
                &mut data_len,
            )
        };
        if err != 0 {
            log(&format!("No data for file {}", filepath));
            return Err("File not found");
        }
        Ok(data[..data_len].to_vec())
    }

    pub fn read_file(filepath: &str) -> Result<Vec<u8>, std::io::Error> {
        let mut data = vec![0; 8192];
        let mut data_len = 0;
        let err = unsafe {
            turbo_os_read_file(
                filepath.as_ptr(),
                filepath.len(),
                data.as_mut_ptr(),
                &mut data_len,
            )
        };
        if err != 0 {
            return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
        }
        Ok(data[..data_len].to_vec())
    }

    pub fn write_file_(filepath: &str, data: &[u8]) -> Result<(), &'static str> {
        let err = unsafe {
            turbo_os_write_file(filepath.as_ptr(), filepath.len(), data.as_ptr(), data.len())
        };
        if err != 0 {
            log(&format!("Could not update file {}", filepath));
            return Err("Failed to write file");
        }
        return Ok(());
    }

    pub fn write_file(filepath: &str, data: &[u8]) -> Result<usize, std::io::Error> {
        let err = unsafe {
            turbo_os_write_file(filepath.as_ptr(), filepath.len(), data.as_ptr(), data.len())
        };
        if err != 0 {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        return Ok(data.len());
    }

    pub fn random_number<T: Default + Copy>() -> T {
        let len = std::mem::size_of::<T>();
        let buf: &mut [u8; 32] = &mut [0u8; 32];
        unsafe { turbo_os_random_bytes(buf.as_mut_ptr(), len) };
        let mut arr = [0u8; 32];
        arr[..len].copy_from_slice(&buf[..len]);
        unsafe { std::ptr::read_unaligned(arr.as_ptr() as *const T) }
    }
}
