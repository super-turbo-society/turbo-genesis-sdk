//! Local Storage Module
//!
//! Provides a simple API for persisting data in the host’s local storage.
//! Data size is limited by the runtime (up to ~5 MB by default).
//!
//! - `save(data: &[u8]) -> Result<i32, i32>`  
//!   Stores the given byte slice in local storage.  
//!   - On success, returns `Ok(0)`.  
//!   - On failure, returns `Err(code)` where `code > 0` is the FFI error code.
//!
//! - `load() -> Result<Vec<u8>, i32>`  
//!   Retrieves the stored bytes into a `Vec<u8>`, allocating up to 5 MB.  
//!   - On success, returns `Ok(data)` truncated to the actual length.  
//!   - On failure, returns `Err(code)` where `code > 0` is the FFI error code.

/// Saves the provided byte slice to local storage.
///
/// # Parameters
/// - `data`: The byte slice to persist.
///
/// # Returns
/// - `Ok(0)` on success.
/// - `Err(code)` if the FFI call returns a positive error code.
///
/// # Notes
/// The runtime may impose a maximum storage size (default ~5 MB).
pub fn save(data: &[u8]) -> Result<i32, i32> {
    let ptr = data.as_ptr();
    let len = data.len() as u32;
    let n = turbo_genesis_ffi::sys::set_local_storage(ptr, len);
    // A positive return value indicates an error code
    if n > 0 {
        Err(n)
    } else {
        Ok(n)
    }
}

/// Loads the contents of local storage into a `Vec<u8>`.
///
/// # Returns
/// - `Ok(data)` containing the stored bytes, truncated to the actual length.
/// - `Err(code)` if the FFI call returns a positive error code.
///
/// # Notes
/// - Allocates a 5 MB buffer by default; large data may be truncated if it exceeds this size.
/// - The returned `Vec<u8>` will be resized to `len` bytes.
pub fn load() -> Result<Vec<u8>, i32> {
    // Allocate up to 5 MB
    let mut data = vec![0; 5 * 1024 * 1024];
    let ptr = data.as_mut_ptr();
    let mut len = 0;
    let n = turbo_genesis_ffi::sys::get_local_storage(ptr, &mut len);
    // A positive return value indicates an error code
    if n > 0 {
        return Err(n);
    }
    // Truncate to the actual data length
    data.truncate(len as usize);
    Ok(data)
}
