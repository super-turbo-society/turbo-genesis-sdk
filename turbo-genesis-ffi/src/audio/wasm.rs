#[allow(clashing_extern_declarations)]
#[link(wasm_import_module = "@turbo_genesis/audio")]
unsafe extern "C" {
    #[link_name = "play_sound"]
    unsafe fn _play_sound(key_ptr: *const u8, key_len: u32) -> u32;
    #[link_name = "pause_sound"]
    unsafe fn _pause_sound(key_ptr: *const u8, key_len: u32) -> u32;
    #[link_name = "stop_sound"]
    unsafe fn _stop_sound(key_ptr: *const u8, key_len: u32) -> u32;
    #[link_name = "is_sound_playing"]
    unsafe fn _is_sound_playing(key_ptr: *const u8, key_len: u32) -> u32;
    #[link_name = "get_volume"]
    unsafe fn _get_volume(key_ptr: *const u8, key_len: u32) -> f32;
    #[link_name = "set_volume"]
    unsafe fn _set_volume(key_ptr: *const u8, key_len: u32, decibels: f32);
    #[link_name = "set_loop_region"]
    unsafe fn _set_loop_region(key_ptr: *const u8, key_len: u32, start: f64, end: f64);
    #[link_name = "get_loop_region"]
    unsafe fn _get_loop_region(key_ptr: *const u8, key_len: u32, start: f64, end: f64);
    #[link_name = "set_sound_setting"]
    unsafe fn _set_sound_setting(
        key_ptr: *const u8,
        key_len: u32,
        setting_ptr: *const u8,
        setting_len: u32,
    );
}

pub fn play_sound(key_ptr: *const u8, key_len: u32) -> u32 {
    unsafe { _play_sound(key_ptr, key_len) }
}

pub fn pause_sound(key_ptr: *const u8, key_len: u32) -> u32 {
    unsafe { _pause_sound(key_ptr, key_len) }
}

pub fn stop_sound(key_ptr: *const u8, key_len: u32) -> u32 {
    unsafe { _stop_sound(key_ptr, key_len) }
}

pub fn is_sound_playing(key_ptr: *const u8, key_len: u32) -> u32 {
    unsafe { _is_sound_playing(key_ptr, key_len) }
}

pub fn get_volume(key_ptr: *const u8, key_len: u32) -> f32 {
    unsafe { _get_volume(key_ptr, key_len) }
}

pub fn set_volume(key_ptr: *const u8, key_len: u32, decibels: f32) {
    unsafe { _set_volume(key_ptr, key_len, decibels) }
}

pub fn set_loop_region(key_ptr: *const u8, key_len: u32, start: f64, end: f64) {
    unsafe { _set_loop_region(key_ptr, key_len, start, end) }
}

pub fn get_loop_region(key_ptr: *const u8, key_len: u32) -> LoopRange {
    unsafe { _get_loop_region(key_ptr, key_len) }
}

pub fn get_sound_settings(key_ptr: *const u8, key_len: u32) -> AudioSettings {
    unsafe { _get_sound_settings(key_ptr, key_len) }
}

pub fn set_sound_setting(
    key_ptr: *const u8,
    key_len: u32,
    setting_ptr: *const u8,
    setting_len: u32,
) {
}
