//! Audio Utility Module
//!
//! Provides high-level functions for sound playback, control, and volume management
//! in the Turbo Genesis environment. Wraps FFI calls and maintains mute state for
//! seamless user experience.

use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    /// Stores the previous volumes of muted audio tracks by key,
    /// so they can be restored upon unmuting.
    static UNMUTE_VOLUMES: RefCell<HashMap<String, f32>> = RefCell::new(HashMap::new());
}

/// Play a sound by its registered name.
///
/// # Parameters
/// - `name`: Identifier of the sound asset (must match engine registry).
pub fn play(name: &str) {
    let ptr = name.as_ptr();
    let len = name.len() as u32;
    turbo_genesis_ffi::audio::play_sound(ptr, len);
}

/// Pause playback of the specified sound.
///
/// # Parameters
/// - `name`: Identifier of the sound to pause.
pub fn pause(name: &str) {
    let ptr = name.as_ptr();
    let len = name.len() as u32;
    turbo_genesis_ffi::audio::pause_sound(ptr, len);
}

/// Stop and reset the specified sound playback.
///
/// # Parameters
/// - `name`: Identifier of the sound to stop.
pub fn stop(name: &str) {
    let ptr = name.as_ptr();
    let len = name.len() as u32;
    turbo_genesis_ffi::audio::stop_sound(ptr, len);
}

/// Query whether a sound is currently playing.
///
/// # Parameters
/// - `name`: Identifier of the sound asset.
///
/// # Returns
/// - `true` if playing, `false` otherwise.
pub fn is_playing(name: &str) -> bool {
    let ptr = name.as_ptr();
    let len = name.len() as u32;
    turbo_genesis_ffi::audio::is_sound_playing(ptr, len) == 1
}

/// Get the current volume of a sound in linear scale (0.0 to 1.0).
///
/// Internally converts decibels to linear percentage via `10^(dB/10)`.
/// Values below 0.0001 are clamped to 0.0.
///
/// # Parameters
/// - `name`: Identifier of the sound asset.
///
/// # Returns
/// - `f32` volume in [0.0, 1.0].
pub fn get_volume(name: &str) -> f32 {
    let ptr = name.as_ptr();
    let len = name.len() as u32;
    let db = turbo_genesis_ffi::audio::get_volume(ptr, len);
    let vol = 10f32.powf(db / 10.0);
    if vol <= 0.0001 {
        0.0
    } else {
        vol
    }
}

/// Set the volume of a sound using linear percentage (0.0 to 100.0).
///
/// Converts to decibels via `dB = 10 * log10(P)`. Values ≤ 0.0 are clamped to -80 dB.
///
/// # Parameters
/// - `name`: Identifier of the sound asset.
/// - `volume`: Desired volume percentage.
pub fn set_volume(name: &str, volume: f32) {
    let ptr = name.as_ptr();
    let len = name.len() as u32;
    let db = if volume <= 0.0 {
        -80.0
    } else {
        10.0 * volume.log10()
    };
    turbo_genesis_ffi::audio::set_volume(ptr, len, db);
}

/// Mute a sound, saving its prior volume for restoration.
///
/// # Parameters
/// - `name`: Identifier of the sound asset.
pub fn mute(name: &str) {
    UNMUTE_VOLUMES.with(|map| {
        let current = get_volume(name);
        let prev = if current <= 0.0001 { 1.0 } else { current };
        map.borrow_mut().insert(name.to_string(), prev);
    });
    set_volume(name, 0.0);
}

/// Check if a sound is muted (effective volume ≈ 0.0%).
///
/// # Parameters
/// - `name`: Identifier of the sound asset.
///
/// # Returns
/// - `true` if muted, `false` otherwise.
pub fn is_muted(name: &str) -> bool {
    get_volume(name) <= 0.0001
}

/// Unmute a sound, restoring its last saved volume.
///
/// # Parameters
/// - `name`: Identifier of the sound asset.
pub fn unmute(name: &str) {
    let vol = UNMUTE_VOLUMES.with(|map| *map.borrow().get(name).unwrap_or(&1.0));
    set_volume(name, vol);
}

/// Get the playback position.
///
///
/// # Parameters
/// - `name`: Identifier of the sound asset.
///
/// # Returns
/// - `f32` for seconds.
pub fn get_playback_position(name: &str) -> f32 {
    let ptr = name.as_ptr();
    let len = name.len() as u32;
    turbo_genesis_ffi::audio::get_playback_position(ptr, len)
}
