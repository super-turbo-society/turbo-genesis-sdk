/// Saves the provided byte slice to persistent storage.
///
/// Returns `Ok(remaining_bytes)` on success, where the value is the number of bytes remaining.
/// Returns `Err(code)` if the FFI function returns a negative error code.
pub fn save(data: &[u8]) -> Result<i32, i32> {
    let ptr = data.as_ptr();
    let len = data.len() as u32;
    let n = turbo_genesis_ffi::sys::save(ptr, len);
    // If n is < 0, it's an error code
    if n < 0 {
        return Err(n);
    }
    // Otherwise, it's remaining storage bytes
    Ok(n)
}

/// Loads persistent storage contents into a static buffer and returns a view into it.
///
/// Returns `Ok(&[u8])` with the slice of valid data, or `Err(code)` if the FFI call failed.
/// Uses a 4MB static buffer and assumes only one consumer of save data at a time.
#[allow(static_mut_refs)]
pub fn load() -> Result<&'static [u8], i32> {
    unsafe {
        // Allocate a big buffer for reading/writing save data
        static mut TURBO_SAVE_DATA: [u8; 4096 * 1000] = [0; 4096 * 1000];
        let ptr = TURBO_SAVE_DATA.as_mut_ptr();
        let mut len = 0;
        let n = turbo_genesis_ffi::sys::load(ptr, &mut len);
        // If n is < 0, it's an error code
        if n < 0 {
            return Err(n);
        }
        Ok(&TURBO_SAVE_DATA[..len as usize])
    }
}
