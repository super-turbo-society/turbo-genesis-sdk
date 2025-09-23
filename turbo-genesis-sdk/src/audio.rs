//! Audio Utility Module
//!
//! Provides high-level functions for sound playback, control, and volume management
//! in the Turbo Genesis environment. Wraps FFI calls and maintains mute state for
//! seamless user experience.

use std::cell::RefCell;
use std::collections::HashMap;

use borsh::BorshDeserialize;
use turbo_genesis_abi::TurboSoundSetting;

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
    turbo_genesis_ffi::audio::get_volume(ptr, len)
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

    turbo_genesis_ffi::audio::set_volume(ptr, len, volume);
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


/// Get the loop region (start, end) for the sound data in seconds.
///
/// # Parameters
/// - `name`: Identifier of the sound asset.
///
/// # Returns
/// - A tuple `(start, end)` in seconds as `f64`s.
pub fn get_loop_region(name: &str) -> (f64, f64) {
    let key_bytes = name.as_bytes();
    let key_ptr = key_bytes.as_ptr();
    let key_len = key_bytes.len() as u32;

    // Prepare output buffer (2 * f64 = 16 bytes)
    let mut buf = [0u8; 16];
    let out_ptr = buf.as_mut_ptr();

    // Call into FFI (host writes [f64; 2] into buf)
 
     turbo_genesis_ffi::audio::get_loop_region(key_ptr, key_len, out_ptr as u32);
    

    // Convert raw bytes back into (f64, f64)
    let start = f64::from_le_bytes(buf[0..8].try_into().unwrap());
    let end   = f64::from_le_bytes(buf[8..16].try_into().unwrap());

    (start, end)
}

/// # Parameters
/// - `name`: Identifier of the sound asset.
/// - `start': start of loop region.
/// - 'end': end of loop region.
pub fn set_loop_region(name: &str, start:f64, end:f64) {
    let ptr = name.as_ptr();
    let len = name.len() as u32;

    turbo_genesis_ffi::audio::set_loop_region(ptr, len, start, end);
}


pub fn get_sound_setting(key: &str) -> Option<TurboSoundSetting> {
    // Prepare key
    let key_bytes = key.as_bytes();
    let key_ptr = key_bytes.as_ptr();
    let key_len = key_bytes.len() as u32;

    // Prepare buffer for result
    let mut data = vec![0u8; 256]; // Adjust capacity as needed
    let mut len: u32 = 0;
    let len_ptr: *mut u32 = &mut len;

    // Call FFI

    turbo_genesis_ffi::audio::get_sound_setting(key_ptr, key_len, data.as_mut_ptr(), len_ptr);

    if len == 0 || (len as usize) > data.len() {
        return None;
    }

    // Deserialize result
    let raw = &data[..len as usize];
    TurboSoundSetting::try_from_slice(raw).ok()
}

pub fn sound_settings() -> Vec<TurboSoundSetting> {
    let mut data = vec![0; 1024];
    let mut len: u32 = 0;
    let len_ptr: *mut u32 = &mut len;

    turbo_genesis_ffi::audio::sound_settings(data.as_mut_ptr(), len_ptr);

    let raw = &data[..len as usize];
    match Vec::<TurboSoundSetting>::try_from_slice(raw) {
        Ok(settings) => settings,
        Err(_e) => {
            vec![]
        }
    }
}
