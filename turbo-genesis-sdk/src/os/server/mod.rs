use borsh::{BorshDeserialize, BorshSerialize};

pub mod channel;
pub mod command;
pub mod fs;

/// Fills and returns a randomly generated value of any Copy type
pub fn random_number<T: Default + Copy>() -> T {
    let len = std::mem::size_of::<T>();
    let buf: &mut [u8; 32] = &mut [0u8; 32];
    turbo_genesis_ffi::os::server::random_bytes(buf.as_mut_ptr(), len);
    let mut arr = [0u8; 32];
    arr[..len].copy_from_slice(&buf[..len]);
    unsafe { std::ptr::read_unaligned(arr.as_ptr() as *const T) }
}

/// Returns the number of seconds since the Unix epoch
pub fn now() -> u32 {
    unsafe { turbo_genesis_ffi::os::server::secs_since_unix_epoch() }
}

/// Logs a message to the server console
pub fn log(message: &str) {
    turbo_genesis_ffi::os::server::log(message.as_ptr(), message.len());
}

/// Emits a custom event with type and payload
pub fn emit(event_type: &str, data: &[u8]) {
    turbo_genesis_ffi::os::server::emit_event(
        event_type.as_ptr(),
        event_type.len(),
        data.as_ptr(),
        data.len(),
    );
}

/// Macro to emit an `alert` event with a formatted string
#[doc(hidden)]
#[macro_export]
macro_rules! __os_server_alert__ {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        let bytes = message.as_bytes();
        $crate::os::server::emit("alert", bytes);
    }};
}

#[doc(inline)]
pub use __os_server_alert__ as alert;

/// Macro to log a formatted string to the console
#[doc(hidden)]
#[macro_export]
macro_rules! __os_server_log__ {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        $crate::os::server::log(&message);
    }};
}

#[doc(inline)]
pub use __os_server_log__ as log;

/// Macro to easily return a custom error with a formatted message
#[doc(hidden)]
#[macro_export]
macro_rules! __os_server_bail__ {
    ($($arg:tt)*) => {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, format!($($arg)*)))
    };
}

#[doc(inline)]
pub use __os_server_bail__ as bail;
