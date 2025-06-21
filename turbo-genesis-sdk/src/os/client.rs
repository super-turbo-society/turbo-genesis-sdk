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
