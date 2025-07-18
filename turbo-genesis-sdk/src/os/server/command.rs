//! Command module
//!
//! This module defines the core abstractions and helpers for handling
//! program commands within the runtime. It provides:
//!
//! - Exit code constants (`COMMIT`, `CANCEL`) for signaling success or failure.
//! - The [`CommandHandler`] trait, which user‐defined command handlers implement.
//! - FFI‐backed utilities:
//!   - `user_id()` to retrieve the current user’s ID.
//!   - `read_input()` and `parse_input<T>()` to read and deserialize incoming command payloads.
//!   - `enqueue(...)` to schedule commands for later execution with optional delay.
//!   - `invoke(...)` to immediately invoke commands on other programs.
//!
//! Together these types and functions form the foundation for writing
//! and dispatching commands in a safe, ergonomic Rust API.

use super::*;
use std::io;

/// Exit code representing successful command execution.
pub const COMMIT: usize = 0;

/// Exit code representing command cancellation or failure.
pub const CANCEL: usize = 1;

/// Trait that command handlers must implement.
///
/// Types implementing this trait provide the logic for handling
/// commands invoked by users in the system.
pub trait CommandHandler {
    /// Execute the command for the given user.
    ///
    /// # Parameters
    /// - `user_id`: The identifier of the user invoking the command.
    ///
    /// # Returns
    /// - `Ok(())` on success.
    /// - `Err(io::Error)` if an I/O or execution error occurs.
    fn run(&mut self, _user_id: &str) -> Result<(), io::Error> {
        Ok(())
    }
}

/// Retrieve the current user ID from the host environment.
///
/// # Returns
/// The user ID as a `String`. Panics if the returned bytes are not valid UTF-8.
pub fn user_id() -> String {
    // Allocate a buffer sized by the host-provided length.
    let mut user_id = vec![0; turbo_genesis_ffi::os::server::get_user_id_len()];
    // Fill the buffer via FFI.
    turbo_genesis_ffi::os::server::get_user_id(user_id.as_mut_ptr());
    // Convert bytes to String, expecting valid UTF-8.
    String::from_utf8(user_id).expect("Invalid UTF-8 sequence")
}

/// Read the raw command input data provided by the host.
///
/// # Returns
/// A `Vec<u8>` containing the full input payload.
pub fn read_input() -> Vec<u8> {
    // Allocate a buffer sized by the host's input length.
    let mut input = vec![0; turbo_genesis_ffi::os::server::get_input_data_len()];
    // Populate the buffer via FFI.
    turbo_genesis_ffi::os::server::get_input_data(input.as_mut_ptr());
    input
}

/// Parse the command input data into a concrete type using Borsh.
///
/// # Type Parameters
/// - `T`: The target type that implements `BorshDeserialize`.
///
/// # Returns
/// - `Ok(T)` on successful deserialization.
/// - `Err(io::Error)` on failure.
pub fn parse_input<T: BorshDeserialize>() -> Result<T, std::io::Error> {
    // Read raw bytes from the host.
    let mut input = vec![0; turbo_genesis_ffi::os::server::get_input_data_len()];
    turbo_genesis_ffi::os::server::get_input_data(input.as_mut_ptr());
    // Attempt Borsh deserialization.
    T::try_from_slice(&input)
}

/// Queue a command to run later, optionally after a delay.
///
/// # Parameters
/// - `program_id`: Identifier of the program handling the command.
/// - `command`: Name of the command to enqueue.
/// - `data`: Serialized command payload bytes.
/// - `nonce`: A unique nonce for idempotency.
/// - `delay`: Optional delay in seconds before execution.
///
/// # Returns
/// - `Ok([u8; 32])` containing the generated command hash on success.
/// - `Err(io::Error)` if enqueuing fails.
pub fn enqueue(
    program_id: &str,
    command: &str,
    data: &[u8],
    nonce: u64,
    delay: Option<u32>,
) -> Result<[u8; 32], std::io::Error> {
    // Prepare an array to receive the command hash.
    let mut hash = [0; 32];
    // Invoke the host enqueue function via FFI.
    let err = turbo_genesis_ffi::os::server::enqueue_command(
        program_id.as_ptr(),
        program_id.len(),
        command.as_ptr(),
        command.len(),
        data.as_ptr(),
        data.len(),
        nonce.to_le_bytes().as_ptr(),
        delay.unwrap_or(0),
        hash.as_mut_ptr(),
    );
    // Interpret non-zero as failure.
    if err != 0 {
        return Err(std::io::Error::from(std::io::ErrorKind::Other));
    }
    Ok(hash)
}

/// Immediately invoke a command on another program.
///
/// # Parameters
/// - `program_id`: Identifier of the target program.
/// - `command`: Name of the command to invoke.
/// - `data`: Serialized command payload bytes.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(io::Error)` with a message containing the host error code.
pub fn invoke(program_id: &str, command: &str, data: &[u8]) -> Result<(), std::io::Error> {
    // Call the host invoke function via FFI.
    let err = turbo_genesis_ffi::os::server::invoke_command(
        program_id.as_ptr(),
        program_id.len(),
        command.as_ptr(),
        command.len(),
        data.as_ptr(),
        data.len(),
    );
    // Map error codes into `io::Error`.
    match err {
        0 => Ok(()),
        code => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error Code: {code}"),
        )),
    }
}
