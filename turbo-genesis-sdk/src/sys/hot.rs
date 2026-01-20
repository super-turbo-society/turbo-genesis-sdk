//! Hot Storage Module
//!
//! **Note:** This module is intended for use during development
//!
//! Provides a simple API for saving and loading application state to the Turbo runtime’s
//! built-in hot storage. Designed for small to moderate data sizes (up to ~4 MB).
//!
//!
//! - `save(data: &[u8]) -> Result<i32, i32>`  
//!   Store the given bytes. On success, returns the number of bytes remaining in storage.
//!   On failure, returns an FFI error code (< 0).
//!
//! - `load() -> Result<&'static [u8], i32>`  
//!   Retrieve all saved bytes into a static 4 MB buffer and return a slice of the valid range.
//!   On failure, returns an FFI error code (< 0).
//!
//! # Notes
//! - Uses fixed‐size buffers and FFI calls; assumes only one concurrent loader/saver.
//! - Error codes are the raw negative values returned by the host API.

/// Saves the provided byte slice to hot storage.
///
/// # Parameters
/// - `data`: The byte slice to write to hot storage.
///
/// # Returns
/// - `Ok(remaining_bytes)`: Number of free bytes left in storage after writing.
/// - `Err(code)`: Negative error code from the FFI layer if the operation failed.
pub fn save(data: &[u8]) -> Result<i32, i32> {
    let ptr = data.as_ptr();
    let len = data.len() as u32;
    let n = turbo_genesis_ffi::sys::save(ptr, len);
    if n < 0 {
        // FFI reports an error via a negative return value
        Err(n)
    } else {
        // Positive return value indicates remaining storage capacity
        Ok(n)
    }
}

/// Loads hot storage contents into a static buffer and returns a slice.
///
/// # Returns
/// - `Ok(&'static [u8])`: Slice of the valid saved bytes in the 4 MB buffer.
/// - `Err(code)`: Negative FFI error code if loading failed.
///
/// # Safety
/// - Uses a mutable static buffer of size 4 096 000 bytes.
/// - Assumes a single consumer and that the host writes valid data.
/// - The returned slice borrows from the static buffer, which remains valid for the program’s lifetime.
#[allow(static_mut_refs)]
pub fn load() -> Result<&'static [u8], i32> {
    // Static 4 MB buffer to hold loaded data
    static mut TURBO_SAVE_DATA: [u8; 4096 * 1000] = [0; 4096 * 1000];
    let mut len = 0;
    let n = unsafe { turbo_genesis_ffi::sys::load(TURBO_SAVE_DATA.as_mut_ptr(), &mut len) };
    if n < 0 {
        // FFI error code
        Err(n)
    } else {
        // SAFETY: It is safe to take a slice of the initialized bytes in the static buffer.
        let data = unsafe { &TURBO_SAVE_DATA[..len as usize] };
        Ok(data)
    }
}
