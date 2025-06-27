use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::BTreeMap;

/// Channel settings
#[derive(Debug, Clone)]
pub struct ChannelSettings {
    pub interval: Option<u32>,
}
impl ChannelSettings {
    pub fn set_interval(&mut self, ms: u32) {
        self.interval = Some(ms);
    }
}
impl Default for ChannelSettings {
    fn default() -> Self {
        Self { interval: None }
    }
}

/// Variants of channel messages a server can receive
#[derive(Debug, Clone)]
pub enum ChannelMessage {
    Connect(String, Vec<u8>),
    Disconnect(String, Vec<u8>),
    Data(String, Vec<u8>),
}

/// Variants of possible channel errors
#[derive(Debug, Clone)]
pub enum ChannelError {
    Timeout,
    AlreadyClosed,
    Unknown,
    Code(u8),
}

/// Receives a message from a client with a timeout
pub fn recv_with_timeout(timeout_ms: u32) -> Result<ChannelMessage, ChannelError> {
    let mut msg_type = 0;
    let mut user_id = [0; 128];
    let mut user_id_len = 0;
    let mut data = [0; 1024];
    let mut data_len = 0;
    let err = turbo_genesis_ffi::os::server::channel_recv_with_timeout(
        &mut msg_type,
        user_id.as_mut_ptr(),
        &mut user_id_len,
        data.as_mut_ptr(),
        &mut data_len,
        timeout_ms,
    );
    match err {
        0 => {
            let user = std::str::from_utf8(&user_id[..user_id_len])
                .map_err(|_| ChannelError::Unknown)?
                .to_string();
            let payload = data[..data_len].to_vec();
            Ok(match msg_type {
                0 => ChannelMessage::Connect(user, payload),
                1 => ChannelMessage::Disconnect(user, payload),
                2 => ChannelMessage::Data(user, payload),
                _ => return Err(ChannelError::Unknown),
            })
        }
        4 => Err(ChannelError::AlreadyClosed),
        5 => Err(ChannelError::Timeout),
        code => Err(ChannelError::Code(code as u8)),
    }
}

/// Blocks indefinitely until a channel message is received
pub fn recv() -> Result<ChannelMessage, ChannelError> {
    recv_with_timeout(u32::MAX)
}

/// Sends a message to a specific connected client
pub fn send<T: BorshSerialize>(user_id: &str, data: T) -> bool {
    let data = borsh::to_vec(&data).unwrap();
    let err = turbo_genesis_ffi::os::server::channel_send(
        user_id.as_ptr(),
        user_id.len(),
        data.as_ptr(),
        data.len(),
    );
    err == 0
}

/// Broadcasts a message to all connected clients
pub fn broadcast<T: BorshSerialize>(data: T) -> bool {
    let data = borsh::to_vec(&data).unwrap();
    let err = turbo_genesis_ffi::os::server::channel_broadcast(data.as_ptr(), data.len());
    err == 0
}

/// Trait that channel handlers must implement
pub trait ChannelHandler {
    type Send: BorshSerialize;
    type Recv: BorshDeserialize;
    fn new() -> Self;
    fn on_open(&mut self, settings: &mut ChannelSettings) {}
    fn on_connect(&mut self, user_id: &str) {}
    fn on_disconnect(&mut self, user_id: &str) {}
    fn on_data(&mut self, user_id: &str, data: Self::Recv) {}
    fn on_interval(&mut self) {}
    fn on_close(&mut self) {}
    fn parse(data: &[u8]) -> Result<Self::Recv, std::io::Error> {
        Self::Recv::try_from_slice(&data)
    }
    fn send(&self, user_id: &str, data: Self::Send) -> Result<(), std::io::Error> {
        let data = borsh::to_vec(&data)?;
        send(user_id, &data);
        Ok(())
    }
    fn broadcast(&self, data: Self::Send) -> Result<(), std::io::Error> {
        let data = borsh::to_vec(&data)?;
        broadcast(&data);
        Ok(())
    }
}
