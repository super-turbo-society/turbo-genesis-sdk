/// Plays a sound identified by the given `key`.
///
/// The key must match a known sound ID registered in the engine.
/// Internally, it forwards the string to the FFI layer.
pub fn play(key: &str) {
    unsafe {
        let key_ptr = key.as_ptr();
        let key_len = key.len() as u32;
        turbo_genesis_ffi::audio::play_sound(key_ptr, key_len);
    }
}

/// Pauses the currently playing sound associated with the given `key`.
pub fn pause(key: &str) {
    unsafe {
        let key_ptr = key.as_ptr();
        let key_len = key.len() as u32;
        turbo_genesis_ffi::audio::pause_sound(key_ptr, key_len);
    }
}

/// Stops the sound playback for the given `key`.
pub fn stop(key: &str) {
    unsafe {
        let key_ptr = key.as_ptr();
        let key_len = key.len() as u32;
        turbo_genesis_ffi::audio::stop_sound(key_ptr, key_len);
    }
}

/// Returns `true` if the sound with the given `key` is currently playing.
///
/// Internally calls an FFI function that returns 1 (true) or 0 (false).
pub fn is_playing(key: &str) -> bool {
    unsafe {
        let key_ptr = key.as_ptr();
        let key_len = key.len() as u32;
        turbo_genesis_ffi::audio::is_sound_playing(key_ptr, key_len) == 1
    }
}
