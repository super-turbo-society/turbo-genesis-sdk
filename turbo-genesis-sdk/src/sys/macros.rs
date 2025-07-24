//! Logging Macros Module
//!
//! Provides a convenient `log!` macro that formats its arguments and
//! dispatches the resulting string to the runtime’s console logging API.
//! Internally, the implementation macro is hidden; users should invoke `log!` directly.

/// Public re-export of the internal `__log__` macro as `log!`.
///
/// ## Usage
///
/// ```ignore
/// log!("Player score: {}", score);
/// ```
#[doc(inline)]
pub use crate::__log__ as log;

/// Internal implementation of the `log!` macro.
///
/// - Takes a format string and optional arguments.  
/// - Expands to a call to `sys::console::log(...)` with the formatted message.
#[doc(hidden)]
#[macro_export]
macro_rules! __log__ {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        $crate::sys::console::log(&format!($fmt, $($($arg)*)?))
    };
}
