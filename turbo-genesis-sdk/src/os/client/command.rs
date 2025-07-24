//! This module provides functionality for executing program commands in the Turbo Genesis SDK.
//!
//! It includes methods for sending commands with serialized payloads or raw byte data to a specified program ID.
//! The result of the execution is a base64url-encoded transaction hash, which can be used for tracking or verification.

use super::*;

/// Executes a program command with a given payload and returns the base64url-encoded transaction hash.
///
/// # Type Parameters
/// - `T`: A type that implements the `BorshSerialize` trait, allowing the payload to be serialized.
///
/// # Arguments
/// - `program_id`: A string slice representing the ID of the program to execute the command on.
/// - `command`: A string slice representing the command to execute.
/// - `payload`: The payload to be serialized and sent with the command.
///
/// # Returns
/// A `String` containing the base64url-encoded transaction hash.
///
/// # Panics
/// This function will panic if the payload serialization fails.
pub fn exec<T: BorshSerialize>(program_id: &str, command: &str, payload: T) -> String {
    let data = borsh::to_vec(&payload).unwrap_or_default();
    exec_raw(program_id, command, &data)
}

/// Executes a program command with raw byte data and returns the base64url-encoded transaction hash.
///
/// # Arguments
/// - `program_id`: A string slice representing the ID of the program to execute the command on.
/// - `command`: A string slice representing the command to execute.
/// - `data`: A byte slice containing the raw data to be sent with the command.
///
/// # Returns
/// A `String` containing the base64url-encoded transaction hash.
///
/// # Notes
/// - If the UTF-8 conversion of the transaction hash fails, an empty string will be returned.
pub fn exec_raw(program_id: &str, command: &str, data: &[u8]) -> String {
    let tx_hash_url_safe_b64 = &mut [0; 43];
    let _ok = turbo_genesis_ffi::os::client::exec(
        program_id.as_ptr(),
        program_id.len() as u32,
        command.as_ptr(),
        command.len() as u32,
        data.as_ptr(),
        data.len() as u32,
        tx_hash_url_safe_b64.as_mut_ptr(),
    );
    std::str::from_utf8(tx_hash_url_safe_b64)
        .unwrap_or_default()
        .to_string()
}
