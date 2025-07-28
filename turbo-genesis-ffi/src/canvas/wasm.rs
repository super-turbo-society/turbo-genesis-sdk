#[allow(clashing_extern_declarations)]
#[link(wasm_import_module = "@turbo_genesis/canvas")]
unsafe extern "C" {
    #[link_name = "clear"]
    unsafe fn _clear(fill: u32);
    #[link_name = "get_camera2"]
    unsafe fn _get_camera(out_ptr: *mut f32) -> i32;
    #[link_name = "set_camera2"]
    unsafe fn _set_camera(x: f32, y: f32, z: f32);
    #[link_name = "text3"]
    unsafe fn _text(
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
    #[link_name = "get_sprite_data_nonce_v1"]
    unsafe fn _get_sprite_data_nonce() -> u64;
    #[link_name = "get_sprite_data_v1"]
    unsafe fn _get_sprite_data(data_ptr: *mut u8, len_ptr: *mut u32);
    #[link_name = "draw_quad2"]
    unsafe fn _draw_quad(
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
    #[link_name = "set_surface_shader"]
    unsafe fn _set_surface_shader(key_ptr: *const u8, key_len: u32) -> u32;
    #[link_name = "get_surface_shader"]
    unsafe fn _get_surface_shader(ptr: *mut u8, len: *mut u32) -> i32;
    #[link_name = "reset_surface_shader"]
    unsafe fn _reset_surface_shader() -> u32;
}

pub fn clear(fill: u32) {
    unsafe { _clear(fill) }
}

pub fn get_camera(out_ptr: *mut f32) -> i32 {
    unsafe { _get_camera(out_ptr) }
}

pub fn set_camera(x: f32, y: f32, z: f32) {
    unsafe { _set_camera(x, y, z) }
}

pub fn text(
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
        _text(
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

pub fn get_sprite_data_nonce() -> u64 {
    unsafe { _get_sprite_data_nonce() }
}

pub fn get_sprite_data(data_ptr: *mut u8, len_ptr: *mut u32) {
    unsafe { _get_sprite_data(data_ptr, len_ptr) }
}

pub fn draw_quad(
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
        _draw_quad(
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
        )
    }
}

pub fn set_surface_shader(key_ptr: *const u8, key_len: u32) -> u32 {
    unsafe { _set_surface_shader(key_ptr, key_len) }
}

pub fn get_surface_shader(ptr: *mut u8, len: *mut u32) -> i32 {
    unsafe { _get_surface_shader(ptr, len) }
}

pub fn reset_surface_shader() -> u32 {
    unsafe { _reset_surface_shader() }
}
