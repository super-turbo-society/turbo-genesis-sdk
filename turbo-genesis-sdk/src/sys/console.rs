//! Logging Module
//!
//! Provides a unified API for sending runtime log messages in both client and server contexts.
//!
//! - In standard mode (`cfg(not(turbo_no_run))`), messages are forwarded directly to the host’s console
//!   via the FFI `sys::log` function (no-op on non-WASM platforms unless implemented).
//! - In server mode (`cfg(turbo_no_run)`), messages are routed through the OS server interface
//!   (`os::server::log`), where they become part of the command/channel logs.
//!
//! # Usage
//! ```ignore
//! log("Player connected");
//! ```  


/// Sends a log message to the Turbo runtime's console output.
///
/// This is a direct FFI call that passes a UTF-8 encoded string to the host.
/// On non-WASM platforms, this may no-op depending on the stub implementation.
#[cfg(not(turbo_no_run))]
pub fn log(text: &str) {
    let ptr = text.as_ptr();
    let len = text.len() as u32;
    turbo_genesis_ffi::sys::log(ptr, len)
}

/// Alternative implementation for logging in server mode.
///
/// This version routes log messages through the OS server interface instead
/// of directly to the system console. Such logs be stored in command and channel logs.
#[cfg(turbo_no_run)]
pub fn log(message: &str) {
    turbo_genesis_ffi::os::server::log(message.as_ptr(), message.len());
}
