//! File system utilities for program data
//!
//! This module provides functions to read and write structured data
//! and raw bytes to program-specific files via the host FFI. It offers:
//!
//! - `read<T>(filepath)` to read and deserialize Borsh-encoded values.
//! - `write<T>(filepath, &value)` to serialize and write values as Borsh.
//! - `read_bytes(filepath)` to fetch raw file contents as a byte vector.
//! - `write_bytes(filepath, &data)` to write raw byte slices to files.

use borsh::{BorshDeserialize, BorshSerialize};

/// Reads a program file, deserializing its Borsh-encoded contents into `T`.
///
/// # Type Parameters
/// - `T`: The target type implementing `BorshDeserialize`.
///
/// # Parameters
/// - `filepath`: The path to the file to read, as a string slice.
///
/// # Returns
/// - `Ok(T)` on successful read and deserialization.
/// - `Err(io::Error)` if reading or deserialization fails.
pub fn read<T: BorshDeserialize>(filepath: &str) -> Result<T, std::io::Error> {
    let data = read_bytes(filepath)?;
    T::try_from_slice(&data)
}

/// Serializes a value `T` with Borsh and writes it to a program file.
///
/// # Type Parameters
/// - `T`: The value type implementing `BorshSerialize`.
///
/// # Parameters
/// - `filepath`: The path to the file to write, as a string slice.
/// - `value`: A reference to the value to serialize and write.
///
/// # Returns
/// - `Ok(bytes_written)` indicating number of bytes written.
/// - `Err(io::Error)` if serialization or writing fails.
pub fn write<T: BorshSerialize>(filepath: &str, value: &T) -> Result<usize, std::io::Error> {
    let data = borsh::to_vec(value)?;
    write_bytes(filepath, &data)
}

/// Reads the raw data of a program file into a `Vec<u8>`.
///
/// # Parameters
/// - `filepath`: The path to the file to read, as a string slice.
///
/// # Returns
/// - `Ok(Vec<u8>)` containing the file’s raw contents.
/// - `Err(io::Error)` if the file is not found or reading fails.
pub fn read_bytes(filepath: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut data = vec![0; 8192]; // Temporary buffer for file data
    let mut data_len = 0; // Actual number of bytes read

    // FFI call to host read_file function
    let err = turbo_genesis_ffi::os::server::read_file(
        filepath.as_ptr(),
        filepath.len(),
        data.as_mut_ptr(),
        &mut data_len,
    );

    // Non-zero return indicates file not found or other error
    if err != 0 {
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
    }

    // Return only the valid portion of the buffer
    Ok(data[..data_len as usize].to_vec())
}

/// Writes raw bytes to a program file via the host FFI.
///
/// # Parameters
/// - `filepath`: The path to the file to write, as a string slice.
/// - `data`: A byte slice containing the data to write.
///
/// # Returns
/// - `Ok(bytes_written)` indicating number of bytes written.
/// - `Err(io::Error)` if the write operation fails.
pub fn write_bytes(filepath: &str, data: &[u8]) -> Result<usize, std::io::Error> {
    // FFI call to host write_file function
    let err = turbo_genesis_ffi::os::server::write_file(
        filepath.as_ptr(),
        filepath.len(),
        data.as_ptr(),
        data.len(),
    );

    // Non-zero return indicates write error or invalid data
    if err != 0 {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
    }

    // Return count of bytes written
    Ok(data.len())
}
