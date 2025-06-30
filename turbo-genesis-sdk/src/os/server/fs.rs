use super::*;

// Reads a program file and parses the contents
pub fn read<T: BorshDeserialize>(filepath: &str) -> Result<T, std::io::Error> {
    let data = read_bytes(filepath)?;
    T::try_from_slice(&data)
}

// Serializes a value and writes the data to a program file
pub fn write<T: BorshSerialize>(filepath: &str, value: &T) -> Result<usize, std::io::Error> {
    let data = borsh::to_vec(value)?;
    write_bytes(filepath, &data)
}

/// Reads a program files raw data
pub fn read_bytes(filepath: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut data = vec![0; 8192];
    let mut data_len = 0;
    let err = turbo_genesis_ffi::os::server::read_file(
        filepath.as_ptr(),
        filepath.len(),
        data.as_mut_ptr(),
        &mut data_len,
    );
    if err != 0 {
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
    }
    Ok(data[..data_len].to_vec())
}

// Writes raw data to a program file
pub fn write_bytes(filepath: &str, data: &[u8]) -> Result<usize, std::io::Error> {
    let err = turbo_genesis_ffi::os::server::write_file(
        filepath.as_ptr(),
        filepath.len(),
        data.as_ptr(),
        data.len(),
    );
    if err != 0 {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
    }
    Ok(data.len())
}
