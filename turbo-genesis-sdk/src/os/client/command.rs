use super::*;

/// Executes a program command with given payload and returns the base64url-encoded transaction hash
pub fn exec<T: BorshSerialize>(program_id: &str, command: &str, payload: T) -> String {
    let data = borsh::to_vec(&payload).unwrap_or_default();
    exec_raw(program_id, command, &data)
}

/// Executes a program command with given bytes and returns the base64url-encoded transaction hash
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
