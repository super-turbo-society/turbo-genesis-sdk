use super::*;
use std::io;

// Exit code representing command success
pub const COMMIT: usize = 0;

// Exit code representing command failure
pub const CANCEL: usize = 1;

/// Trait that command handlers must implement
pub trait CommandHandler {
    fn run(&mut self, user_id: &str) -> Result<(), io::Error> {
        Ok(())
    }
}

/// Returns the current user ID as a string
pub fn user_id() -> String {
    let mut user_id = vec![0; turbo_genesis_ffi::os::server::get_user_id_len()];
    turbo_genesis_ffi::os::server::get_user_id(user_id.as_mut_ptr());
    String::from_utf8(user_id).expect("Invalid UTF-8 sequence")
}

/// Reads the full raw command input data from the host
pub fn read_input() -> Vec<u8> {
    let mut input = vec![0; turbo_genesis_ffi::os::server::get_input_data_len()];
    turbo_genesis_ffi::os::server::get_input_data(input.as_mut_ptr());
    input
}

/// Attempts to parse command input data using Borsh
pub fn parse_input<T: BorshDeserialize>() -> Result<T, std::io::Error> {
    let mut input = vec![0; turbo_genesis_ffi::os::server::get_input_data_len()];
    turbo_genesis_ffi::os::server::get_input_data(input.as_mut_ptr());
    T::try_from_slice(&input)
}

/// Queues a command to run at a later time with an optional delay
pub fn enqueue(
    program_id: &str,
    command: &str,
    data: &[u8],
    nonce: u64,
    delay: Option<u32>,
) -> Result<[u8; 32], std::io::Error> {
    let mut hash = [0; 32];
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
    if err != 0 {
        return Err(std::io::Error::from(std::io::ErrorKind::Other));
    }
    Ok(hash)
}

/// Immediately invokes a command for another program
pub fn invoke(program_id: &str, command: &str, data: &[u8]) -> Result<(), std::io::Error> {
    let err = turbo_genesis_ffi::os::server::invoke_command(
        program_id.as_ptr(),
        program_id.len(),
        command.as_ptr(),
        command.len(),
        data.as_ptr(),
        data.len(),
    );
    match err {
        0 => Ok(()),
        code => Err(std::io::Error::other(format!("Error Code: {code}"))),
    }
}
