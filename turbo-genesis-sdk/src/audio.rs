/// Plays a sound identified by the given `key`.
///
/// The key must match a known sound ID registered in the engine.
/// Internally, it forwards the string to the FFI layer.
pub fn play(key: &str) {
    let key_ptr = key.as_ptr();
    let key_len = key.len() as u32;
    turbo_genesis_ffi::audio::play_sound(key_ptr, key_len);
}

/// Pauses the currently playing sound associated with the given `key`.
pub fn pause(key: &str) {
    let key_ptr = key.as_ptr();
    let key_len = key.len() as u32;
    turbo_genesis_ffi::audio::pause_sound(key_ptr, key_len);
}

/// Stops the sound playback for the given `key`.
pub fn stop(key: &str) {
    let key_ptr = key.as_ptr();
    let key_len = key.len() as u32;
    turbo_genesis_ffi::audio::stop_sound(key_ptr, key_len);
}

/// Returns `true` if the sound with the given `key` is currently playing.
///
/// Internally calls an FFI function that returns 1 (true) or 0 (false).
pub fn is_playing(key: &str) -> bool {
    let key_ptr = key.as_ptr();
    let key_len = key.len() as u32;
    turbo_genesis_ffi::audio::is_sound_playing(key_ptr, key_len) == 1
}

/// Gets the current volume of the sound identified by `key`, expressed as a percentage (0.0 to 100.0).
///
/// Converts the internal decibel value to a linear scale using: P = 10^(AdB / 10) * 100
pub fn get_volume(key: &str) -> f32 {
    let key_ptr = key.as_ptr();
    let key_len = key.len() as u32;
    let db = turbo_genesis_ffi::audio::get_volume(key_ptr, key_len);
    let volume = 10f32.powf(db / 10.0) * 100.0;
    if volume <= 0.0001 {
        return 0.0;
    }
    return volume;
}

/// Sets the volume of the sound identified by `key` using a percentage (0.0 to 100.0).
///
/// Converts to decibels using: AdB = 10 * log10(P / 100)
/// Values <= 0.0 are clamped to -80.0 dB (mute).
pub fn set_volume(key: &str, percent: f32) {
    let key_ptr = key.as_ptr();
    let key_len = key.len() as u32;
    let db = if percent <= 0.0 {
        -80.0
    } else {
        10.0 * (percent / 100.0).log10()
    };
    turbo_genesis_ffi::audio::set_volume(key_ptr, key_len, db);
}

/// Mutes the sound identified by `key`.
///
/// Equivalent to setting volume to 0.0%.
pub fn mute(key: &str) {
    set_volume(key, 0.0);
}

/// Returns `true` if the sound identified by `key` is currently muted.
///
/// This checks whether the effective linear volume is 0.0%.
pub fn is_muted(key: &str) -> bool {
    get_volume(key) <= 0.0001 // -60db or lower is muted
}

/// Resets the volume of the sound identified by `key` to 100%.
pub fn reset_volume(key: &str) {
    set_volume(key, 100.0);
}
