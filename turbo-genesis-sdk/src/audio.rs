use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::LazyLock;

thread_local! {
    // Stores volumes of audio tracks by their key when they are muted so they can be restored when unmuted
    static UNMUTE_VOLUMES: RefCell<HashMap<String, f32>> = RefCell::new(HashMap::new());
}

/// Plays a sound identified by the given `name`.
///
/// The key must match a known sound ID registered in the engine.
/// Internally, it forwards the string to the FFI layer.
pub fn play(name: &str) {
    let key_ptr = name.as_ptr();
    let key_len = name.len() as u32;
    turbo_genesis_ffi::audio::play_sound(key_ptr, key_len);
}

/// Pauses the currently playing sound associated with the given `name`.
pub fn pause(name: &str) {
    let key_ptr = name.as_ptr();
    let key_len = name.len() as u32;
    turbo_genesis_ffi::audio::pause_sound(key_ptr, key_len);
}

/// Stops the sound playback for the given `name`.
pub fn stop(name: &str) {
    let key_ptr = name.as_ptr();
    let key_len = name.len() as u32;
    turbo_genesis_ffi::audio::stop_sound(key_ptr, key_len);
}

/// Returns `true` if the sound with the given `name` is currently playing.
///
/// Internally calls an FFI function that returns 1 (true) or 0 (false).
pub fn is_playing(name: &str) -> bool {
    let key_ptr = name.as_ptr();
    let key_len = name.len() as u32;
    turbo_genesis_ffi::audio::is_sound_playing(key_ptr, key_len) == 1
}

/// Gets the current volume of the sound identified by `name`, expressed as a percentage (0.0 to 1.0).
///
/// Converts the internal decibel value to a linear scale using: P = 10^(AdB / 10)
pub fn get_volume(name: &str) -> f32 {
    let key_ptr = name.as_ptr();
    let key_len = name.len() as u32;
    let db = turbo_genesis_ffi::audio::get_volume(key_ptr, key_len);
    let volume = 10f32.powf(db / 10.0);
    if volume <= 0.0001 {
        return 0.0;
    }
    return volume;
}

/// Sets the volume of the sound identified by `name` using a percentage (0.0 to 100.0).
///
/// Converts to decibels using: AdB = 10 * log10(P)
/// Values <= 0.0 are clamped to -80.0 dB.
pub fn set_volume(name: &str, volume: f32) {
    let key_ptr = name.as_ptr();
    let key_len = name.len() as u32;
    let db = if volume <= 0.0 {
        -80.0
    } else {
        10.0 * volume.log10()
    };
    turbo_genesis_ffi::audio::set_volume(key_ptr, key_len, db);
}

/// Mutes the sound identified by `name`.
///
/// Equivalent to setting volume to 0.0%.
pub fn mute(name: &str) {
    unsafe {
        UNMUTE_VOLUMES.with(|a| {
            a.borrow_mut().insert(name.to_string(), get_volume(name));
        })
    }
    set_volume(name, 0.0);
}

/// Returns `true` if the sound identified by `name` is currently muted.
///
/// This checks whether the effective linear volume is 0.0%.
pub fn is_muted(name: &str) -> bool {
    get_volume(name) <= 0.0001 // -60db or lower is muted
}

/// Resets the sound's volume to the last volume before it was muted
pub fn unmute(name: &str) {
    let volume =
        unsafe { UNMUTE_VOLUMES.with(|a| *a.borrow().get(&name.to_string()).unwrap_or(&1.0)) };
    set_volume(name, volume);
}
