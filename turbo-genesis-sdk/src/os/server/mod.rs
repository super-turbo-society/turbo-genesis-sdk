//! OS server interface
//!
//! This module exposes functions and macros for interacting with the host OS server
//! environment, including random number generation, time access, event emission,
//! and convenience macros for alerts and error bailing.

use borsh::BorshDeserialize;

pub mod channel;
pub mod command;
pub mod fs;

/// Emits a custom event to the OS server with a given type and payload.
///
/// # Parameters
/// - `event_type`: A string slice identifying the event type.
/// - `data`: A byte slice containing the event payload.
///
/// # Example
/// ```ignore
/// emit("userLogin", &[1,2,3]);
/// ```
pub fn emit(event_type: &str, data: &[u8]) {
    turbo_genesis_ffi::os::server::emit_event(
        event_type.as_ptr(),
        event_type.len(),
        data.as_ptr(),
        data.len(),
    );
}

/// Internal macro: format and emit an `alert` event.
///
/// # Usage
/// ```ignore
/// alert!("Value exceeded: {}", threshold);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __os_server_alert__ {
    ($($arg:tt)*) => {{
        // Construct the message string
        let message = format!($($arg)*);
        // Emit as bytes under the "alert" type
        $crate::os::server::emit("alert", message.as_bytes());
    }};
}

/// Public alias for `__os_server_alert__`, emits a formatted alert event.
#[doc(inline)]
pub use __os_server_alert__ as alert;

/// Internal macro: bail early with a formatted I/O error.
///
/// # Usage
/// ```ignore
/// bail!("Invalid state: {}", reason);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __os_server_bail__ {
    ($($arg:tt)*) => {
        return Err(std::io::Error::other(format!($($arg)*)));
    };
}

/// Public alias for `__os_server_bail__`, returns a formatted `io::Error`.
#[doc(inline)]
pub use __os_server_bail__ as bail;
