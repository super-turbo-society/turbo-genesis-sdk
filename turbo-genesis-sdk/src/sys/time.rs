/// Returns the number of game ticks that have occurred since startup.
///
/// Internally uses a monotonic counter provided by the runtime.
pub fn tick() -> usize {
    turbo_genesis_ffi::sys::tick() as usize
}

/// Returns the current time in milliseconds since the Unix epoch.
///
/// This is typically used for timestamps or measuring real-world elapsed time.
pub fn now() -> u64 {
    turbo_genesis_ffi::sys::millis_since_unix_epoch()
}
