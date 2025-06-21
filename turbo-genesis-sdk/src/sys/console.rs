/// Sends a log message to the Turbo runtime's console output.
///
/// This is a direct FFI call that passes a UTF-8 encoded string to the host.
/// On non-WASM platforms, this may no-op depending on the stub implementation.
pub fn log(text: &str) {
    let ptr = text.as_ptr();
    let len = text.len() as u32;
    turbo_genesis_ffi::sys::log(ptr, len)
}
