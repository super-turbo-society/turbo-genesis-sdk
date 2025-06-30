pub fn clear(_fill: u32) {}

pub fn get_camera(_out_ptr: *mut f32) -> i32 {
    0
}

pub fn set_camera(_x: f32, _y: f32, _z: f32) {}

pub fn text(
    _x: i32,
    _y: i32,
    _color: u32,
    _scale: f32,
    _rotation: f32,
    _font_name_ptr: *const u8,
    _font_name_len: u32,
    _text_ptr: *const u8,
    _text_len: u32,
    _flags: u32,
) {
}

pub fn get_sprite_data_nonce() -> u64 {
    0
}

pub fn get_sprite_data(_data_ptr: *mut u8, _len_ptr: *mut u32) {}

pub fn draw_quad(
    _dest_xy: u64,
    _dest_wh: u64,
    _sprite_xy: u64,
    _sprite_wh: u64,
    _sprite_xy_offset: u64,
    _fill_ab: u64,
    _border_radius: u32,
    _border_size: u32,
    _border_color: u32,
    _origin_xy: u64,
    _rotation_deg: i32,
    _flags: u32,
) {
}

pub fn set_surface_shader(_key_ptr: *const u8, _key_len: u32) -> i32 {
    0
}

pub fn get_surface_shader(_ptr: *mut u8, _len: *mut u32) -> i32 {
    0
}

pub fn reset_surface_shader() -> i32 {
    0
}
