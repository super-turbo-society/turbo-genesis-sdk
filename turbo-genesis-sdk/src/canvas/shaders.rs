pub fn set(key: &str) {
    let key_ptr = key.as_ptr();
    let key_len = key.len() as u32;
    crate::ffi::canvas::set_surface_shader(key_ptr, key_len)
}

pub fn get() -> String {
    let mut bytes = [0; 512]; // shader name up to 512 bytes
    let key_ptr = bytes.as_mut_ptr();
    let mut key_len = 0;
    if crate::ffi::canvas::get_surface_shader(key_ptr, &mut key_len) != 0 {
        return String::new();
    }
    String::from_utf8(bytes[..key_len as usize].to_vec()).unwrap_or_default()
}

pub fn reset() {
    crate::ffi::canvas::reset_surface_shader()
}
