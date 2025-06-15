#[cfg(not(target_family = "wasm"))]
pub fn clear(fill: u32) {}
#[cfg(target_family = "wasm")]
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
#[cfg(target_family = "wasm")]
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
#[cfg(target_family = "wasm")]
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
#[cfg(target_family = "wasm")]
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
#[cfg(target_family = "wasm")]
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
#[cfg(target_family = "wasm")]
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
#[cfg(target_family = "wasm")]
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
#[cfg(target_family = "wasm")]
pub fn get_sprite_data_nonce_v1() -> u64 {
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
#[cfg(target_family = "wasm")]
pub fn get_sprite_data_v1(data_ptr: *mut u8, len_ptr: *mut u32) {
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
#[cfg(target_family = "wasm")]
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
#[cfg(target_family = "wasm")]
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
#[cfg(target_family = "wasm")]
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
#[cfg(target_family = "wasm")]
pub fn reset_surface_shader() {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/canvas")]
        extern "C" {
            fn reset_surface_shader() -> u32;
        }
        reset_surface_shader();
    }
}
