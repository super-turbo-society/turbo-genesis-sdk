use crate::ffi;

pub fn draw(
    color: u32,
    dx: i32,
    dy: i32,
    dw: u32,
    dh: u32,
    border_radius: u32,
    border_size: u32,
    border_color: u32,
    origin_x: i32,
    origin_y: i32,
    rotation_deg: i32,
    flags: u32,
) {
    let dest_xy = ((dx as u64) << 32) | (dy as u32 as u64);
    let dest_wh = ((dw as u64) << 32) | (dh as u32 as u64);
    let origin_xy = ((origin_x as u64) << 32) | (origin_y as u64 & 0xffffffff);
    let fill_ab = (color as u64) << 32;
    ffi::canvas::draw_quad2(
        dest_xy,
        dest_wh,
        0,
        0,
        0,
        fill_ab,
        border_radius,
        border_size,
        border_color,
        origin_xy,
        rotation_deg,
        flags,
    );
}
