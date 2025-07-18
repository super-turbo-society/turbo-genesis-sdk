//! This module provides the `ChannelConnection` implementation for sending and receiving
//! Borsh-serializable messages over a Turbo Genesis OS client channel. It defines methods
//! for serializing outgoing messages, deserializing incoming messages, and handling
//! communication errors using FFI bindings.

use borsh::{BorshDeserialize, BorshSerialize};
use std::io;

/// Represents a channel bound to a specific program and ID, parameterized over
/// transmit (`Tx`) and receive (`Rx`) types. These types must match on both sides of the channel.
#[derive(Debug)]
pub struct Channel<Tx, Rx> {
    program_id: String,   // The program identifier for the channel
    channel_kind: String, // The kind/type of the channel
    channel_id: String,   // The unique channel identifier
    _phantom: std::marker::PhantomData<(Tx, Rx)>, // Marker for generic types
}

impl<Tx, Rx> Channel<Tx, Rx> {
    /// Creates a new typed channel handle. This does not establish a connection yet.
    ///
    /// # Arguments
    /// * `program_id` - The program identifier as a string slice.
    /// * `channel_kind` - The kind/type of the channel as a string slice.
    /// * `channel_id` - The unique channel identifier as a string slice.
    pub fn new(program_id: &str, channel_kind: &str, channel_id: &str) -> Self {
        Self {
            program_id: program_id.to_string(),
            channel_kind: channel_kind.to_string(),
            channel_id: channel_id.to_string(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Attempts to connect to the channel. Returns `None` if not ready.
    ///
    /// # Returns
    /// * `Some(ChannelConnection<Tx, Rx>)` if connection is successful.
    /// * `None` if connection fails.
    pub fn connection(&self) -> Option<ChannelConnection<Tx, Rx>> {
        let status = turbo_genesis_ffi::os::client::channel_connect(
            self.program_id.as_ptr(),
            self.program_id.len() as u32,
            self.channel_kind.as_ptr(),
            self.channel_kind.len() as u32,
            self.channel_id.as_ptr(),
            self.channel_id.len() as u32,
        );
        if status != 0 {
            return None;
        }
        Some(ChannelConnection {
            program_id: self.program_id.clone(),
            channel_kind: self.channel_kind.clone(),
            channel_id: self.channel_id.clone(),
            _phantom: std::marker::PhantomData,
        })
    }

    /// Creates and connects a channel in one step. Returns `None` if not ready.
    ///
    /// # Arguments
    /// * `program_id` - The program identifier as a string slice.
    /// * `channel_kind` - The kind/type of the channel as a string slice.
    /// * `channel_id` - The unique channel identifier as a string slice.
    ///
    /// # Returns
    /// * `Some(ChannelConnection<Tx, Rx>)` if connection is successful.
    /// * `None` if connection fails.
    pub fn subscribe(
        program_id: &str,
        channel_kind: &str,
        channel_id: &str,
    ) -> Option<ChannelConnection<Tx, Rx>> {
        Channel::<Tx, Rx>::new(program_id, channel_kind, channel_id).connection()
    }
}

/// Represents an active channel connection that can send and receive messages.
#[derive(Debug)]
pub struct ChannelConnection<Tx, Rx> {
    program_id: String,   // The program identifier for the channel
    channel_kind: String, // The kind/type of the channel
    channel_id: String,   // The unique channel identifier
    _phantom: std::marker::PhantomData<(Tx, Rx)>, // Marker for generic types
}

impl<Tx: BorshSerialize, Rx: BorshDeserialize> ChannelConnection<Tx, Rx> {
    /// Sends a Borsh-serializable message over the channel.
    ///
    /// # Arguments
    /// * `msg` - Reference to the message to send, which must implement `BorshSerialize`.
    ///
    /// # Returns
    /// * `Ok(())` if the message was sent successfully.
    /// * `Err(io::Error)` if sending failed.
    pub fn send(&self, msg: &Tx) -> io::Result<()> {
        let data = borsh::to_vec(msg)?;

        let err = &mut [0; 4096]; // Buffer for error messages
        let mut err_len = 0;

        let status = turbo_genesis_ffi::os::client::channel_send(
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
        );

        match status {
            0 => Ok(()),
            _ if err_len > 0 => Err(io::Error::new(
                io::ErrorKind::Other,
                String::from_utf8_lossy(&err[..err_len as usize]).to_string(),
            )),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Failed to send data")),
        }
    }

    /// Attempts to receive a Borsh-deserializable message from the channel.
    ///
    /// # Returns
    /// * `Ok(Rx)` if a message was received and deserialized successfully.
    /// * `Err(io::Error)` if connection failed, no messages, or deserialization failed.
    #[inline]
    pub fn recv(&self) -> io::Result<Rx> {
        const STATUS_PENDING: u32 = 1;
        const STATUS_FAILED: u32 = 2;

        let data = &mut [0; 4096]; // Buffer for received data
        let mut data_len = 0;
        let err = &mut [0; 1024]; // Buffer for error messages
        let mut err_len = 0;

        let status = turbo_genesis_ffi::os::client::channel_recv(
            self.program_id.as_ptr(),
            self.program_id.len() as u32,
            self.channel_kind.as_ptr(),
            self.channel_kind.len() as u32,
            self.channel_id.as_ptr(),
            self.channel_id.len() as u32,
            data.as_mut_ptr(),
            &mut data_len,
            err.as_mut_ptr(),
            &mut err_len,
        );

        match status {
            STATUS_PENDING | STATUS_FAILED => Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "Connection closed",
            )),
            _ if data_len > 0 => {
                let slice = &data[..data_len as usize];
                Ok(Rx::try_from_slice(slice).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Failed to deserialize recv data",
                    )
                })?)
            }
            _ if err_len > 0 => Err(io::Error::new(
                io::ErrorKind::Other,
                String::from_utf8_lossy(&err[..err_len as usize]).to_string(),
            )),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "No data received",
            )),
        }
    }
}
