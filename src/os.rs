use super::*;
use base64::{
    engine::general_purpose::{STANDARD as b64, URL_SAFE_NO_PAD as b64_url_safe},
    Engine,
};
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

pub mod encoding {
    use super::*;

    pub fn encode_base64<T: AsRef<[u8]>>(input: T) -> String {
        b64.encode(input)
    }

    pub fn encode_base64_url_safe<T: AsRef<[u8]>>(input: T) -> String {
        b64_url_safe.encode(input)
    }

    pub fn decode_base64<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, base64::DecodeError> {
        b64.decode(input)
    }

    pub fn decode_base64_url_safe<T: AsRef<[u8]>>(
        input: T,
    ) -> Result<Vec<u8>, base64::DecodeError> {
        b64_url_safe.decode(input)
    }
}

#[derive(Debug, Clone)]
pub struct QueryResult<T> {
    pub loading: bool,
    pub data: Option<T>,
    pub error: Option<String>,
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
    use borsh::{BorshDeserialize, BorshSerialize};

    use super::*;

    #[allow(clashing_extern_declarations)]
    #[link(wasm_import_module = "@turbo_genesis/turbo_os")]
    extern "C" {
        #[link_name = "get_user_id"]
        fn turbo_genesis_get_user_id(
            out_user_id_ptr: *mut u8,
            out_user_id_len_ptr: *mut u32,
        ) -> u32;

        #[link_name = "channel_is_connected"]
        fn turbo_genesis_channel_is_connected(
            program_id_ptr: *const u8,
            program_id_len: u32,
            channel_kind_ptr: *const u8,
            channel_kind_len: u32,
            channel_id_ptr: *const u8,
            channel_id_len: u32,
        ) -> u32;

        #[link_name = "channel_recv"]
        fn turbo_genesis_channel_recv(
            program_id_ptr: *const u8,
            program_id_len: u32,
            channel_kind_ptr: *const u8,
            channel_kind_len: u32,
            channel_id_ptr: *const u8,
            channel_id_len: u32,
            out_data_ptr: *const u8,
            out_data_len_ptr: *mut u32,
            out_err_ptr: *const u8,
            out_err_len_ptr: *mut u32,
        ) -> u32;

        #[link_name = "channel_send"]
        fn turbo_genesis_channel_send(
            program_id_ptr: *const u8,
            program_id_len: u32,
            channel_kind_ptr: *const u8,
            channel_kind_len: u32,
            channel_id_ptr: *const u8,
            channel_id_len: u32,
            data_ptr: *const u8,
            data_len: u32,
            out_err_ptr: *const u8,
            out_err_len_ptr: *mut u32,
        ) -> u32;

        #[link_name = "watch_events"]
        fn turbo_genesis_watch_events(
            program_id_ptr: *const u8,
            program_id_len: u32,
            event_type_ptr: *const u8,
            event_type_len: u32,
            out_data_ptr: *const u8,
            out_data_len_ptr: *mut u32,
            out_err_ptr: *const u8,
            out_err_len_ptr: *mut u32,
        ) -> u32;

        #[link_name = "read_file"]
        fn turbo_genesis_read_file(
            program_id_ptr: *const u8,
            program_id_len: u32,
            filepath_ptr: *const u8,
            filepath_len: u32,
            query_ptr: *const u8,
            query_len: u32,
            out_data_ptr: *mut u8,
            out_data_len_ptr: *mut u32,
            out_err_ptr: *mut u8,
            out_err_len_ptr: *mut u32,
        ) -> u32;

        #[link_name = "exec"]
        fn turbo_genesis_exec(
            program_id_ptr: *const u8,
            program_id_len: u32,
            command_ptr: *const u8,
            command_len: u32,
            data_ptr: *const u8,
            data_len: u32,
            tx_hash_ptr: *mut u8,
        ) -> u32;
    }

    pub mod channel {
        use super::*;

        #[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
        pub enum Channel {
            Connected(Connection<Open>),
            Disconnected(Connection<Closed>),
        }
        impl Channel {
            pub fn subscribe(program_id: &str, channel_kind: &str, channel_id: &str) -> Channel {
                match os::client::channel::is_connected(program_id, channel_kind, channel_id) {
                    true => Channel::Connected(Connection {
                        state: Open,
                        program_id: program_id.to_string(),
                        channel_kind: channel_kind.to_string(),
                        channel_id: channel_id.to_string(),
                    }),
                    false => Channel::Disconnected(Connection {
                        state: Closed,
                        program_id: program_id.to_string(),
                        channel_kind: channel_kind.to_string(),
                        channel_id: channel_id.to_string(),
                    }),
                }
            }
        }

        #[derive(Debug)]
        pub enum ChannelError {
            Connection(std::io::Error),
            WithMessage(Vec<u8>),
            Unknown,
        }

        /// Generic connection type with a state
        #[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
        pub struct Connection<T> {
            state: T,
            program_id: String,
            channel_kind: String,
            channel_id: String,
        }

        /// Marker type for an active connection
        #[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
        pub struct Open;

        /// Marker type for an inactive connection
        #[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
        pub struct Closed;

        impl Connection<Open> {
            /// Receives a message from the channel.
            pub fn recv(&self) -> Result<Option<Vec<u8>>, ChannelError> {
                match os::client::channel::recv(
                    &self.program_id,
                    &self.channel_kind,
                    &self.channel_id,
                ) {
                    Ok(None) => Ok(None),
                    Ok(Some(Ok(msg))) => Ok(Some(msg)),
                    Ok(Some(Err(msg))) => Err(ChannelError::WithMessage(msg)),
                    Err(None) => Err(ChannelError::Unknown),
                    Err(Some(err)) => Err(ChannelError::Connection(err)),
                }
            }

            /// Sends a message to the channel.
            pub fn send(&self, data: &[u8]) -> Result<(), std::io::Error> {
                let err = &mut [0; 4096];
                let mut err_len = 0;
                let status = unsafe {
                    turbo_genesis_channel_send(
                        self.program_id.as_ptr(),
                        self.program_id.len() as u32,
                        self.channel_kind.as_ptr(),
                        self.channel_kind.len() as u32,
                        self.channel_id.as_ptr(),
                        self.channel_id.len() as u32,
                        data.as_ptr(),
                        data.len() as u32,
                        err.as_mut_ptr(),
                        &mut err_len,
                    )
                };

                match status {
                    0 => Ok(()), // STATUS_OK
                    _ => {
                        // Parse err into error string
                        if err_len > 0 {
                            let bytes = err[..err_len as usize].to_vec();
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::NotConnected,
                                String::from_utf8_lossy(&bytes).to_string(),
                            ));
                        }
                        // Default error
                        Err(std::io::Error::new(
                            std::io::ErrorKind::NotConnected,
                            "Failed to send data",
                        ))
                    }
                }
            }
        }

        impl Connection<Closed> {
            /// Attempts to reconnect to the channel.
            pub fn connect(&self) {
                let _ = os::client::channel::recv(
                    &self.program_id,
                    &self.channel_kind,
                    &self.channel_id,
                );
            }
        }

        fn is_connected(program_id: &str, channel_kind: &str, channel_id: &str) -> bool {
            let status = unsafe {
                turbo_genesis_channel_is_connected(
                    program_id.as_ptr(),
                    program_id.len() as u32,
                    channel_kind.as_ptr(),
                    channel_kind.len() as u32,
                    channel_id.as_ptr(),
                    channel_id.len() as u32,
                )
            };
            status == 0
        }

        fn recv(
            program_id: &str,
            channel_kind: &str,
            channel_id: &str,
        ) -> Result<Option<Result<Vec<u8>, Vec<u8>>>, Option<std::io::Error>> {
            const STATUS_PENDING: u32 = 1;
            const STATUS_FAILED: u32 = 2;

            let data = &mut [0; 4096];
            let mut data_len = 0;
            let err = &mut [0; 1024];
            let mut err_len = 0;

            let status = unsafe {
                turbo_genesis_channel_recv(
                    program_id.as_ptr(),
                    program_id.len() as u32,
                    channel_kind.as_ptr(),
                    channel_kind.len() as u32,
                    channel_id.as_ptr(),
                    channel_id.len() as u32,
                    data.as_mut_ptr(),
                    &mut data_len,
                    err.as_mut_ptr(),
                    &mut err_len,
                )
            };

            // Connecting...
            if status == STATUS_PENDING {
                return Err(None);
            }

            // Error connecting
            if status == STATUS_FAILED {
                let err = std::io::Error::new(
                    std::io::ErrorKind::NotConnected,
                    "Channel connection failed",
                );
                return Err(Some(err));
            }

            // Parse data into program event
            if data_len > 0 {
                let bytes = data[..data_len as usize].to_vec();
                return Ok(Some(Ok(bytes)));
            }

            // Parse err into error string
            if err_len > 0 {
                let bytes = err[..err_len as usize].to_vec();
                return Ok(Some(Err(bytes)));
            }

            // No new message or errors
            return Ok(None);
        }
    }

    pub fn watch_events(program_id: &str, event_type: Option<&str>) -> QueryResult<ProgramEvent> {
        // const STATUS_COMPLETE: u32 = 0;
        const STATUS_PENDING: u32 = 1;
        const STATUS_FAILED: u32 = 2;

        let data = &mut [0; 4096];
        let mut data_len = 0;
        let err = &mut [0; 1024];
        let mut err_len = 0;

        let event_type = event_type.unwrap_or("");
        let status = unsafe {
            turbo_genesis_watch_events(
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
        watch_file_with_opts(program_id, filepath, &[("stream", "true")])
    }

    pub fn watch_file_with_opts<'a, S: std::fmt::Display>(
        program_id: &str,
        filepath: &str,
        opts: &[(S, S)],
    ) -> QueryResult<ProgramFile> {
        let query = opts
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        // const STATUS_COMPLETE: u32 = 0;
        const STATUS_PENDING: u32 = 1;
        const STATUS_FAILED: u32 = 2;
        let data = &mut [0; 8192];
        let mut data_len = 0;
        let err = &mut [0; 1024];
        let mut err_len = 0;
        let status = unsafe {
            turbo_genesis_read_file(
                program_id.as_ptr(),
                program_id.len() as u32,
                filepath.as_ptr(),
                filepath.len() as u32,
                query.as_ptr(),
                query.len() as u32,
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

    pub fn exec(program_id: &str, command: &str, data: &[u8]) -> String {
        let tx_hash_url_safe_b64 = &mut [0; 43]; // url-safe, no-pad
        let _ok = unsafe {
            turbo_genesis_exec(
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
        let ok = unsafe { turbo_genesis_get_user_id(data.as_mut_ptr(), &mut data_len) };
        if ok == 0 {
            String::from_utf8(data[..data_len as usize].to_vec()).ok()
        } else {
            None
        }
    }
}

pub mod server {
    use std::u32;

    use borsh::{BorshDeserialize, BorshSerialize};

    use super::*;

    #[allow(clashing_extern_declarations)]
    #[link(wasm_import_module = "turbo")]
    extern "C" {
        #[link_name = "random_bytes"]
        fn turbo_os_random_bytes(ptr: *mut u8, len: usize) -> usize;

        #[link_name = "secs_since_unix_epoch"]
        fn turbo_os_secs_since_unix_epoch() -> u32;

        #[link_name = "get_user_id_len"]
        fn turbo_os_get_user_id_len() -> usize;

        #[link_name = "get_user_id"]
        fn turbo_os_get_user_id(ptr: *mut u8) -> usize;

        #[link_name = "get_input_data_len"]
        fn turbo_os_get_input_data_len() -> usize;

        #[link_name = "get_input_data"]
        fn turbo_os_get_input_data(ptr: *mut u8) -> usize;

        #[link_name = "log"]
        fn turbo_os_log(ptr: *const u8, len: usize) -> usize;

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

        #[link_name = "enqueue_command"]
        fn turbo_os_enqueue_command(
            program_id_ptr: *const u8,
            program_id_len: usize,
            command_ptr: *const u8,
            command_len: usize,
            data_ptr: *const u8,
            data_len: usize,
            nonce_ptr: *const u8,
            delay: u32,
            hash_out_ptr: *mut u8,
        ) -> usize;

        #[link_name = "invoke_command"]
        fn turbo_os_invoke_command(
            program_id_ptr: *const u8,
            program_id_len: usize,
            command_ptr: *const u8,
            command_len: usize,
            data_ptr: *const u8,
            data_len: usize,
        ) -> usize;

        #[link_name = "channel_recv"]
        fn turbo_os_channel_recv_with_timeout(
            msg_type_ptr: *mut u8,
            user_id_ptr: *mut u8,
            user_id_len_ptr: *mut usize,
            data_ptr: *mut u8,
            data_len_ptr: *mut usize,
            timeout_ms: u32,
        ) -> usize;

        #[link_name = "channel_send"]
        fn turbo_os_channel_send(
            user_id_ptr: *const u8,
            user_id_len: usize,
            data_ptr: *const u8,
            data_len: usize,
        ) -> usize;

        #[link_name = "channel_broadcast"]
        fn turbo_os_channel_broadcast(data_ptr: *const u8, data_len: usize) -> usize;
    }

    pub const COMMIT: usize = 0;

    pub const CANCEL: usize = 1;

    #[derive(Debug, Clone)]
    pub enum ChannelMessage {
        Connect(String, Vec<u8>),
        Disconnect(String, Vec<u8>),
        Data(String, Vec<u8>),
    }

    #[derive(Debug, Clone)]
    pub enum ChannelError {
        Timeout,
        AlreadyClosed,
        Unknown,
        Code(u8),
    }

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

    pub fn secs_since_unix_epoch() -> u32 {
        unsafe { turbo_os_secs_since_unix_epoch() }
    }

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

    pub fn write_file(filepath: &str, data: &[u8]) -> Result<usize, std::io::Error> {
        let err = unsafe {
            turbo_os_write_file(filepath.as_ptr(), filepath.len(), data.as_ptr(), data.len())
        };
        if err != 0 {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        return Ok(data.len());
    }

    pub fn enqueue_command(
        program_id: &str,
        command: &str,
        data: &[u8],
        nonce: u64,
        delay: Option<u32>,
    ) -> Result<[u8; 32], std::io::Error> {
        let mut hash = [0; 32];
        let err = unsafe {
            turbo_os_enqueue_command(
                program_id.as_ptr(),
                program_id.len(),
                command.as_ptr(),
                command.len(),
                data.as_ptr(),
                data.len(),
                nonce.to_le_bytes().as_ptr(),
                delay.unwrap_or(0),
                hash.as_mut_ptr(),
            )
        };
        if err != 0 {
            return Err(std::io::Error::from(std::io::ErrorKind::Other));
        }
        return Ok(hash);
    }

    pub fn invoke_command(
        program_id: &str,
        command: &str,
        data: &[u8],
    ) -> Result<(), std::io::Error> {
        let err = unsafe {
            turbo_os_invoke_command(
                program_id.as_ptr(),
                program_id.len(),
                command.as_ptr(),
                command.len(),
                data.as_ptr(),
                data.len(),
            )
        };
        match err {
            0 => Ok(()),
            code => Err(std::io::Error::other(format!("Error Code: {code}"))),
        }
    }

    pub fn channel_recv_with_timeout(timeout_ms: u32) -> Result<ChannelMessage, ChannelError> {
        let mut msg_type = 0;
        let mut user_id = [0; 128];
        let mut user_id_len = 0;
        let mut data = [0; 1024];
        let mut data_len = 0;
        let err = unsafe {
            turbo_os_channel_recv_with_timeout(
                &mut msg_type,
                user_id.as_mut_ptr(),
                &mut user_id_len,
                data.as_mut_ptr(),
                &mut data_len,
                timeout_ms,
            )
        };
        match err {
            0 => {
                let user_id = std::str::from_utf8(&user_id[..user_id_len])
                    .map_err(|_| ChannelError::Unknown)?
                    .to_string();
                let data = data[..data_len].to_vec();
                Ok(match msg_type {
                    0 => ChannelMessage::Connect(user_id, data),
                    1 => ChannelMessage::Disconnect(user_id, data),
                    2 => ChannelMessage::Data(user_id, data),
                    _ => return Err(ChannelError::Unknown),
                })
            }
            4 => Err(ChannelError::AlreadyClosed),
            5 => Err(ChannelError::Timeout),
            code => Err(ChannelError::Code(code as u8)),
        }
    }

    pub fn channel_recv() -> Result<ChannelMessage, ChannelError> {
        channel_recv_with_timeout(u32::MAX)
    }

    pub fn channel_send(user_id: &str, data: &[u8]) -> bool {
        let err = unsafe {
            turbo_os_channel_send(user_id.as_ptr(), user_id.len(), data.as_ptr(), data.len())
        };
        err == 0
    }

    pub fn channel_broadcast(data: &[u8]) -> bool {
        let err = unsafe { turbo_os_channel_broadcast(data.as_ptr(), data.len()) };
        err == 0
    }

    pub fn random_number<T: Default + Copy>() -> T {
        let len = std::mem::size_of::<T>();
        let buf: &mut [u8; 32] = &mut [0u8; 32];
        unsafe { turbo_os_random_bytes(buf.as_mut_ptr(), len) };
        let mut arr = [0u8; 32];
        arr[..len].copy_from_slice(&buf[..len]);
        unsafe { std::ptr::read_unaligned(arr.as_ptr() as *const T) }
    }

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
}
