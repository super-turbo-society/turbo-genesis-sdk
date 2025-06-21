/// Saves the provided byte slice to local storage.
///
/// Returns `Ok(0)` on success, or `Err(code)` if the FFI function returns an error code (`> 0`).
pub fn save(data: &[u8]) -> Result<i32, i32> {
    let ptr = data.as_ptr();
    let len = data.len() as u32;
    let n = turbo_genesis_ffi::sys::set_local_storage(ptr, len);
    // If n is > 0, it's an error code
    if n > 0 {
        return Err(n);
    }
    Ok(n)
}

/// Loads the contents of local storage into a `Vec<u8>`.
///
/// Allocates up to 5MB and truncates to the actual written length.
/// Returns `Ok(data)` on success, or `Err(code)` if the FFI function returns an error code (`> 0`).
pub fn load() -> Result<Vec<u8>, i32> {
    unsafe {
        // Allocate a 5MB buffer
        let mut data = vec![0; 1048576 * 5];
        let ptr = data.as_mut_ptr();
        let mut len = 0;
        let n = turbo_genesis_ffi::sys::get_local_storage(ptr, &mut len);
        // If n is > 0, it's an error code
        if n > 0 {
            return Err(n);
        }
        data.truncate(len as usize);
        Ok(data)
    }
}
