pub fn play_sound(_: *const u8, _: u32) -> u32 {
    0
}

pub fn pause_sound(_: *const u8, _: u32) -> u32 {
    0
}

pub fn stop_sound(_: *const u8, _: u32) -> u32 {
    0
}

pub fn is_sound_playing(_: *const u8, _: u32) -> u32 {
    0
}

pub fn get_volume(key_ptr: *const u8, key_len: u32) -> f32 {
    0.0
}

pub fn set_volume(key_ptr: *const u8, key_len: u32, decibels: f32) {}

pub fn set_panning(key_ptr: *const u8, key_len: u32, panning: f32) {}

pub fn sound_settings(_data_ptr: *mut u8, _len_ptr: *mut u32) {}

pub fn get_loop_region(key_ptr: *const u8, key_len: u32, out_ptr: u32) {}

pub fn set_loop_region(key_ptr: *const u8, key_len: u32, start: f64, end:f64) {}

pub fn get_sound_setting(key_ptr: *const u8, key_len: u32, data_ptr: *mut u8, len_ptr: *mut u32) {}
