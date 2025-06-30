/// Reads an environment variable from the runtime by key.
///
/// Returns an empty string if the key doesn't exist or the value is empty.
/// FFI expects `key` as a UTF-8 string, and returns up to 1024 bytes of UTF-8 data.
///
/// # Safety
/// Relies on a static mutable buffer and assumes returned data is valid UTF-8.
#[allow(static_mut_refs)]
pub fn get(key: &str) -> String {
    unsafe {
        let key_ptr = key.as_ptr();
        let key_len = key.len() as u32;

        // Fixed-size buffer for env var output (up to 1024 bytes)
        let mut data = [0; 1024];
        let out_var_ptr = data.as_mut_ptr();
        let mut out_var_len = 0;

        turbo_genesis_ffi::sys::env_get(key_ptr, key_len, out_var_ptr, &mut out_var_len);

        if out_var_len == 0 {
            return String::new();
        }

        String::from_utf8(data[..(out_var_len as usize)].to_vec()).unwrap_or_default()
    }
}
