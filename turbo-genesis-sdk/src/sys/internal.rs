pub fn set(key: &str, data: &[u8]) -> Result<(), std::io::Error> {
    let key_ptr = key.as_ptr();
    let key_len = key.len() as u32;
    let data_ptr = data.as_ptr();
    let data_len = data.len() as u32;
    let n = turbo_genesis_ffi::sys::set_internal_storage(key_ptr, key_len, data_ptr, data_len);
    if n == 0 {
        return Ok(());
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "internal storage set failed",
    ))
}

pub fn get(key: &str) -> Result<Vec<u8>, std::io::Error> {
    let key_ptr = key.as_ptr();
    let key_len = key.len() as u32;
    let data = &mut [0; 4096];
    let data_ptr = data.as_mut_ptr();
    let mut data_len = 0 as u32;
    let n = turbo_genesis_ffi::sys::get_internal_storage(key_ptr, key_len, data_ptr, &mut data_len);
    if n == 0 {
        return Ok(data[..data_len as usize].to_vec());
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "internal storage get failed",
    ))
}
