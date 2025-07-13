/// Returns the number of game ticks that have occurred since startup.
///
/// Internally uses a monotonic counter provided by the runtime.
///
/// ## Implementation Details
/// - **Standard mode (`cfg(not(turbo_no_run))`):** Uses the system's tick counter
///   via `turbo_genesis_ffi::sys::tick()` which provides a monotonic counter
///   that increments at a fixed rate (typically 60 Hz for games). This is ideal
///   for game logic that needs consistent timing regardless of real-world time.
/// - **Server mode (`cfg(turbo_no_run)`):** Currently no alternative implementation
///   is provided for this function in server mode. This suggests that
///   game tick counting may not be available or relevant in server contexts
///   where the turbo_no_run configuration is used.
#[cfg(not(turbo_no_run))]
pub fn tick() -> usize {
    turbo_genesis_ffi::sys::tick() as usize
}

/// Returns the current time in milliseconds since the Unix epoch.
///
/// This is typically used for timestamps or measuring real-world elapsed time.
///
/// ## Implementation Details
/// - **Standard mode (`cfg(not(turbo_no_run))`):** Uses the system's high-resolution
///   timer via `turbo_genesis_ffi::sys::millis_since_unix_epoch()` which provides
///   millisecond precision for accurate timing measurements.
/// - **Server mode (`cfg(turbo_no_run)`):** Uses an alternative implementation
///   that calls `turbo_genesis_ffi::os::server::secs_since_unix_epoch()` through
///   the OS server interface. Note that this version only provides second-level
///   precision (not milliseconds), which may result in less accurate timing
///   in server environments. The `unsafe` block is required because the server
///   interface may involve FFI calls that can't be statically verified as safe.
#[cfg(not(turbo_no_run))]
pub fn now() -> u64 {
    turbo_genesis_ffi::sys::millis_since_unix_epoch()
}

/// Alternative implementation for getting Unix timestamp in server mode.
///
/// **Important**: This version only provides second-level precision, unlike
/// the standard implementation which provides millisecond precision. This means
/// that in server mode, timing measurements will be less granular and may
/// not be suitable for high-precision timing requirements.
#[cfg(turbo_no_run)]
pub fn now() -> u64 {
    turbo_genesis_ffi::os::server::secs_since_unix_epoch() as u64
}
