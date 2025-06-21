#[doc(inline)]
pub use crate::__log__ as log;

#[doc(hidden)]
#[macro_export]
macro_rules! __log__ {
    ($fmt:expr $(, $($arg:tt)*)?) => { $crate::sys::console::log(&format!($fmt, $($($arg)*)?)) };
}
