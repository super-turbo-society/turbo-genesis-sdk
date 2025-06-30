/// Emits an event with the given name and associated data.
///
/// This is typically used to dispatch a `CustomEvent("turboGameEvent")` on `window` when running in a browser.
/// The `name` and `data` values provided will be accessible via the event.details object.
/// 
/// Internally forwards the call to the FFI layer.
pub fn emit(name: &str, data: &str) {
    let name_ptr = name.as_ptr();
    let name_len = name.len() as u32;
    let data_ptr = data.as_ptr();
    let data_len = data.len() as u32;
    turbo_genesis_ffi::sys::emit(name_ptr, name_len, data_ptr, data_len)
}
