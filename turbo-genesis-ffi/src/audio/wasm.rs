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
