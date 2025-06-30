/// Sets the active surface shader by name.
pub fn set(key: &str) {
    let key_ptr = key.as_ptr();
    let key_len = key.len() as u32;
    turbo_genesis_ffi::canvas::set_surface_shader(key_ptr, key_len);
}

/// Gets the name of the currently active surface shader as a UTF-8 string.
/// Returns an empty string if there was an error or invalid UTF-8.
pub fn get() -> String {
    let mut bytes = [0; 512]; // buffer for shader name (max 512 bytes)
    let key_ptr = bytes.as_mut_ptr();
    let mut key_len = 0;
    if turbo_genesis_ffi::canvas::get_surface_shader(key_ptr, &mut key_len) != 0 {
        return String::new();
    }
    String::from_utf8(bytes[..key_len as usize].to_vec()).unwrap_or_default()
}

/// Resets the active surface shader to the default or null shader.
pub fn reset() {
    turbo_genesis_ffi::canvas::reset_surface_shader();
}
