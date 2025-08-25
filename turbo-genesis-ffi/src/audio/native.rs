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

pub fn get_sound_setting(_data_ptr: *mut u8, _len_ptr: *mut u32) {}
