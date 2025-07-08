use super::*;

pub fn play(key: &str) {
    unsafe {
        let key_ptr = key.as_ptr();
        let key_len = key.len() as u32;
        let mut data = [0; 1024];
        ffi::audio::play_sound(key_ptr, key_len);
    }
}

pub fn pause(key: &str) {
    unsafe {
        let key_ptr = key.as_ptr();
        let key_len = key.len() as u32;
        let mut data = [0; 1024];
        ffi::audio::pause_sound(key_ptr, key_len);
    }
}

pub fn stop(key: &str) {
    unsafe {
        let key_ptr = key.as_ptr();
        let key_len = key.len() as u32;
        let mut data = [0; 1024];
        ffi::audio::stop_sound(key_ptr, key_len);
    }
}

pub fn is_playing(key: &str) -> bool {
    unsafe {
        let key_ptr = key.as_ptr();
        let key_len = key.len() as u32;
        let mut data: [i32; 1024] = [0; 1024];
        ffi::audio::is_sound_playing(key_ptr, key_len) == 1
    }
}

pub fn get_sound_state(key: &str) -> f32 {
    unsafe {
        let key_ptr = key.as_ptr();
        let key_len = key.len() as u32;
        ffi::audio::get_sound_state(key_ptr, key_len)
    }
}
