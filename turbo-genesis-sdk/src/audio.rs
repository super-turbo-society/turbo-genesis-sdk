use super::*;

/// Plays a sound identified by the given `key`.
///
/// The key must match a known sound ID registered in the engine.
/// Internally, it forwards the string to the FFI layer.
pub fn play(key: &str) {
    unsafe {
        let key_ptr = key.as_ptr();
        let key_len = key.len() as u32;

        // Temporary buffer (unused, may be for ABI size alignment or FFI stub compatibility)
        let mut data = [0; 1024];

        // Call FFI to play the sound
        ffi::audio::play_sound(key_ptr, key_len);
    }
}

/// Pauses the currently playing sound associated with the given `key`.
pub fn pause(key: &str) {
    unsafe {
        let key_ptr = key.as_ptr();
        let key_len = key.len() as u32;

        // Temporary buffer, may be a placeholder for ABI layout consistency
        let mut data = [0; 1024];

        // Call FFI to pause the sound
        ffi::audio::pause_sound(key_ptr, key_len);
    }
}

/// Stops the sound playback for the given `key`.
pub fn stop(key: &str) {
    unsafe {
        let key_ptr = key.as_ptr();
        let key_len = key.len() as u32;

        // Placeholder buffer, not currently used
        let mut data = [0; 1024];

        // Call FFI to stop the sound
        ffi::audio::stop_sound(key_ptr, key_len);
    }
}

/// Returns `true` if the sound with the given `key` is currently playing.
///
/// Internally calls an FFI function that returns 1 (true) or 0 (false).
pub fn is_playing(key: &str) -> bool {
    unsafe {
        let key_ptr = key.as_ptr();
        let key_len = key.len() as u32;

        // Unused buffer — may be reserved for future ABI needs
        let mut data: [i32; 1024] = [0; 1024];

        // Check if the sound is playing
        ffi::audio::is_sound_playing(key_ptr, key_len) == 1
    }
}
