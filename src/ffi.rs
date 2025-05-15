#[allow(unused)]
pub mod sys {

    #[cfg(not(target_family = "wasm"))]
    pub fn tick() -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn tick() -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn tick() -> u32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/sys")]
            extern "C" {
                fn tick() -> u32;
            }
            tick()
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn rand() -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn rand() -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn rand() -> u32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/sys")]
            extern "C" {
                fn rand() -> u32;
            }
            rand()
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn log(ptr: *const u8, len: u32) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn log(ptr: *const u8, len: u32) {}
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn log(ptr: *const u8, len: u32) {
        // #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/sys")]
            extern "C" {
                fn log(ptr: *const u8, len: u32);
            }
            log(ptr, len)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn env_get(
        key_ptr: *const u8,
        key_len: u32,
        out_var_ptr: *mut u8,
        out_var_len: *mut u32,
    ) -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn env_get(
        key_ptr: *const u8,
        key_len: u32,
        out_var_ptr: *mut u8,
        out_var_len: *mut u32,
    ) -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn env_get(
        key_ptr: *const u8,
        key_len: u32,
        out_var_ptr: *mut u8,
        out_var_len: *mut u32,
    ) -> u32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/sys")]
            extern "C" {
                fn env_get(
                    key_ptr: *const u8,
                    key_len: u32,
                    out_var_ptr: *mut u8,
                    out_var_len: *mut u32,
                ) -> u32;
            }
            env_get(key_ptr, key_len, out_var_ptr, out_var_len)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn resolution() -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn resolution() -> u32 {
        super::internal::read_snapshot_resolution()
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn resolution() -> u32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/sys")]
            extern "C" {
                fn resolution() -> u32;
            }
            resolution()
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn save(ptr: *const u8, len: u32) -> i32 {
        -1
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn save(ptr: *const u8, len: u32) -> i32 {
        let mut state = super::internal::read_snapshot_state();
        unsafe { std::ptr::copy(ptr, state.as_mut_ptr(), len as usize) };
        super::internal::write_snapshot_state(&state) as i32
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn save(ptr: *const u8, len: u32) -> i32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/sys")]
            extern "C" {
                fn save(ptr: *const u8, len: u32) -> i32;
            }
            save(ptr, len)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn load(ptr: *mut u8, len: *mut u32) -> i32 {
        return -1;
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn load(ptr: *mut u8, len: *mut u32) -> i32 {
        let state = super::internal::read_snapshot_state();
        unsafe {
            std::ptr::copy(state.as_ptr(), ptr, state.len());
            *len = state.len() as u32;
        };
        0
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn load(ptr: *mut u8, len: *mut u32) -> i32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/sys")]
            extern "C" {
                fn load(ptr: *mut u8, len: *mut u32) -> i32;
            }
            load(ptr, len)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn set_local_storage(ptr: *const u8, len: u32) -> i32 {
        -1
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn set_local_storage(ptr: *const u8, len: u32) -> i32 {
        let mut state = super::internal::read_snapshot_state();
        unsafe { std::ptr::copy(ptr, state.as_mut_ptr(), len as usize) };
        super::internal::write_snapshot_state(&state) as i32
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn set_local_storage(ptr: *const u8, len: u32) -> i32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/sys")]
            extern "C" {
                fn set_local_storage(ptr: *const u8, len: u32) -> i32;
            }
            set_local_storage(ptr, len)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn get_local_storage(ptr: *mut u8, len: *mut u32) -> i32 {
        return -1;
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn get_local_storage(ptr: *mut u8, len: *mut u32) -> i32 {
        let state = super::internal::read_snapshot_state();
        unsafe {
            std::ptr::copy(state.as_ptr(), ptr, state.len());
            *len = state.len() as u32;
        };
        0
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn get_local_storage(ptr: *mut u8, len: *mut u32) -> i32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/sys")]
            extern "C" {
                fn get_local_storage(ptr: *mut u8, len: *mut u32) -> i32;
            }
            get_local_storage(ptr, len)
        }
    }
}

#[allow(unused)]
pub(crate) mod audio {
    #[cfg(not(target_family = "wasm"))]
    pub fn play_sound(key_ptr: *const u8, key_len: u32) -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn play_sound(key_ptr: *const u8, key_len: u32) -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn play_sound(key_ptr: *const u8, key_len: u32) -> u32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/audio")]
            extern "C" {
                fn play_sound(key_ptr: *const u8, key_len: u32) -> u32;
            }
            play_sound(key_ptr, key_len)
        }
    }
    #[cfg(not(target_family = "wasm"))]
    pub fn pause_sound(key_ptr: *const u8, key_len: u32) -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn pause_sound(key_ptr: *const u8, key_len: u32) -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn pause_sound(key_ptr: *const u8, key_len: u32) -> u32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/audio")]
            extern "C" {
                fn pause_sound(key_ptr: *const u8, key_len: u32) -> u32;
            }
            pause_sound(key_ptr, key_len)
        }
    }
    #[cfg(not(target_family = "wasm"))]
    pub fn stop_sound(key_ptr: *const u8, key_len: u32) -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn stop_sound(key_ptr: *const u8, key_len: u32) -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn stop_sound(key_ptr: *const u8, key_len: u32) -> u32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/audio")]
            extern "C" {
                fn stop_sound(key_ptr: *const u8, key_len: u32) -> u32;
            }
            stop_sound(key_ptr, key_len)
        }
    }
    #[cfg(not(target_family = "wasm"))]
    pub fn is_sound_playing(key_ptr: *const u8, key_len: u32) -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn is_sound_playing(key_ptr: *const u8, key_len: u32) -> u32 {
        0
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn is_sound_playing(key_ptr: *const u8, key_len: u32) -> u32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/audio")]
            extern "C" {
                fn is_sound_playing(key_ptr: *const u8, key_len: u32) -> u32;
            }
            is_sound_playing(key_ptr, key_len)
        }
    }
}

#[allow(unused)]
pub mod input {
    #[cfg(not(target_family = "wasm"))]
    pub fn gamepad(player: u32, out_ptr: *mut u8) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn gamepad(player: u32, out_ptr: *mut u8) {
        let gamepad = super::internal::read_snapshot_gamepad(player as usize);
        unsafe { std::ptr::copy(gamepad.as_ptr(), out_ptr, gamepad.len()) };
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn gamepad(player: u32, out_ptr: *mut u8) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/input")]
            extern "C" {
                fn gamepad(player: u32, out_ptr: *mut u8);
            }
            return gamepad(player, out_ptr);
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn mouse(player: u32, out_ptr: *mut u8) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn mouse(player: u32, out_ptr: *mut u8) {}
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn mouse(player: u32, out_ptr: *mut u8) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/input")]
            extern "C" {
                fn mouse(player: u32, out_ptr: *mut u8);
            }
            mouse(player, out_ptr)
        }
    }
}

#[allow(unused)]
pub mod canvas {
    #[cfg(not(target_family = "wasm"))]
    pub fn clear(fill: u32) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn clear(fill: u32) {}
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn clear(fill: u32) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn clear(fill: u32);
            }
            clear(fill)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn set_camera(x: i32, y: i32) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn set_camera(x: i32, y: i32) {}
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn set_camera(x: i32, y: i32) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn set_camera(x: i32, y: i32);
            }
            set_camera(x, y)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn get_camera() -> i32 {
        0
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn get_camera() -> i32 {
        0
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn get_camera() -> i32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn get_camera() -> i32;
            }
            get_camera()
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn get_camera2(out_ptr: *mut f32) -> i32 {
        0
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn get_camera2(out_ptr: *mut f32) -> i32 {
        0
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn get_camera2(out_ptr: *mut f32) -> i32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn get_camera2(out_ptr: *mut f32) -> i32;
            }
            get_camera2(out_ptr)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn set_camera2(x: f32, y: f32, z: f32) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn set_camera2(x: f32, y: f32, z: f32) {}
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn set_camera2(x: f32, y: f32, z: f32) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn set_camera2(x: f32, y: f32, z: f32);
            }
            set_camera2(x, y, z)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn text2(
        x: i32,
        y: i32,
        color: u32,
        scale: f32,
        rotation: f32,
        font_name_ptr: *const u8,
        font_name_len: u32,
        text_ptr: *const u8,
        text_len: u32,
    ) {
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn text2(
        x: i32,
        y: i32,
        color: u32,
        scale: f32,
        rotation: f32,
        font_name_ptr: *const u8,
        font_name_len: u32,
        text_ptr: *const u8,
        text_len: u32,
    ) {
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn text2(
        x: i32,
        y: i32,
        color: u32,
        scale: f32,
        rotation: f32,
        font_name_ptr: *const u8,
        font_name_len: u32,
        text_ptr: *const u8,
        text_len: u32,
    ) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn text2(
                    x: i32,
                    y: i32,
                    color: u32,
                    scale: f32,
                    rotation: f32,
                    font_name_ptr: *const u8,
                    font_name_len: u32,
                    text_ptr: *const u8,
                    text_len: u32,
                );
            }
            text2(
                x,
                y,
                color,
                scale,
                rotation,
                font_name_ptr,
                font_name_len,
                text_ptr,
                text_len,
            )
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn text3(
        x: i32,
        y: i32,
        color: u32,
        scale: f32,
        rotation: f32,
        font_name_ptr: *const u8,
        font_name_len: u32,
        text_ptr: *const u8,
        text_len: u32,
        flags: u32,
    ) {
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn text3(
        x: i32,
        y: i32,
        color: u32,
        scale: f32,
        rotation: f32,
        font_name_ptr: *const u8,
        font_name_len: u32,
        text_ptr: *const u8,
        text_len: u32,
        flags: u32,
    ) {
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn text3(
        x: i32,
        y: i32,
        color: u32,
        scale: f32,
        rotation: f32,
        font_name_ptr: *const u8,
        font_name_len: u32,
        text_ptr: *const u8,
        text_len: u32,
        flags: u32,
    ) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn text3(
                    x: i32,
                    y: i32,
                    color: u32,
                    scale: f32,
                    rotation: f32,
                    font_name_ptr: *const u8,
                    font_name_len: u32,
                    text_ptr: *const u8,
                    text_len: u32,
                    flags: u32,
                );
            }
            text3(
                x,
                y,
                color,
                scale,
                rotation,
                font_name_ptr,
                font_name_len,
                text_ptr,
                text_len,
                flags,
            )
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn get_sprite_data_nonce_v1() -> u64 {
        0
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn get_sprite_data_nonce_v1() -> u64 {}
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn get_sprite_data_nonce_v1() -> u64 {
        #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn get_sprite_data_nonce_v1() -> u64;
            }
            return get_sprite_data_nonce_v1();
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn get_sprite_data_v1(data_ptr: *mut u8, len_ptr: *mut u32) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn get_sprite_data_v1(data_ptr: *mut u8, len_ptr: *mut u32) {}
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn get_sprite_data_v1(data_ptr: *mut u8, len_ptr: *mut u32) {
        #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn get_sprite_data_v1(data_ptr: *mut u8, len_ptr: *mut u32);
            }
            return get_sprite_data_v1(data_ptr, len_ptr);
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn draw_quad2(
        dest_xy: u64,
        dest_wh: u64,
        sprite_xy: u64,
        sprite_wh: u64,
        sprite_xy_offset: u64,
        fill_ab: u64,
        border_radius: u32,
        border_size: u32,
        border_color: u32,
        origin_xy: u64,
        rotation_deg: i32,
        flags: u32,
    ) {
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn draw_quad2(
        dest_xy: u64,
        dest_wh: u64,
        sprite_xy: u64,
        sprite_wh: u64,
        sprite_xy_offset: u64,
        fill_ab: u64,
        border_radius: u32,
        border_size: u32,
        border_color: u32,
        origin_xy: u64,
        rotation_deg: i32,
        flags: u32,
    ) {
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn draw_quad2(
        dest_xy: u64,
        dest_wh: u64,
        sprite_xy: u64,
        sprite_wh: u64,
        sprite_xy_offset: u64,
        fill_ab: u64,
        border_radius: u32,
        border_size: u32,
        border_color: u32,
        origin_xy: u64,
        rotation_deg: i32,
        flags: u32,
    ) {
        #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn draw_quad2(
                    dest_xy: u64,
                    dest_wh: u64,
                    sprite_xy: u64,
                    sprite_wh: u64,
                    sprite_xy_offset: u64,
                    fill_ab: u64,
                    border_radius: u32,
                    border_size: u32,
                    border_color: u32,
                    origin_xy: u64,
                    rotation_deg: i32,
                    flags: u32,
                );
            }
            return draw_quad2(
                dest_xy,
                dest_wh,
                sprite_xy,
                sprite_wh,
                sprite_xy_offset,
                fill_ab,
                border_radius,
                border_size,
                border_color,
                origin_xy,
                rotation_deg,
                flags,
            );
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn set_surface_shader(key_ptr: *const u8, key_len: u32) {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn set_surface_shader(key_ptr: *const u8, key_len: u32) {}
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn set_surface_shader(key_ptr: *const u8, key_len: u32) {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn set_surface_shader(key_ptr: *const u8, key_len: u32) -> u32;
            }
            set_surface_shader(key_ptr, key_len);
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn get_surface_shader(ptr: *mut u8, len: *mut u32) -> i32 {
        0
    }
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn get_surface_shader(ptr: *mut u8, len: *mut u32) -> i32 {
        0
    }
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn get_surface_shader(ptr: *mut u8, len: *mut u32) -> i32 {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn get_surface_shader(ptr: *mut u8, len: *mut u32) -> i32;
            }
            get_surface_shader(ptr, len)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn reset_surface_shader() {}
    #[cfg(all(target_family = "wasm", feature = "no-host"))]
    pub fn reset_surface_shader() {}
    #[cfg(all(target_family = "wasm", not(feature = "no-host")))]
    pub fn reset_surface_shader() {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/canvas")]
            extern "C" {
                fn reset_surface_shader() -> u32;
            }
            reset_surface_shader();
        }
    }
}
