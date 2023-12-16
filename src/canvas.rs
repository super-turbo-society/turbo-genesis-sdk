use crate::ffi;
use default_args::default_args;

pub fn clear(color: u32) {
    ffi::canvas::clear(color)
}

pub fn get_camera() -> [i32; 2] {
    let cam = ffi::canvas::get_camera();
    let x = cam & 0xffff;
    let y = cam >> 16;
    [x, y]
}

pub fn set_camera(x: i32, y: i32) {
    ffi::canvas::set_camera(x, y)
}

pub fn rectfillv(
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    fill: u32,
    rotation_deg: i32,
    rotation_origin_x: i32,
    _rotation_origin_y: i32,
    border_radius_h: u32,
    _border_radius_v: u32,
    border_size: u32,
    border_color: u32,
) {
    // TODO: xy
    let rotation_origin = rotation_origin_x;
    // TODO: wh
    let border_radius = border_radius_h;
    ffi::canvas::quad(
        x << 16 | (y & 0xffff),
        w << 16 | h,
        fill,
        rotation_deg,
        rotation_origin,
        border_radius,
        border_size,
        border_color,
    )
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Border {
    pub radius: u32,
    pub size: u32,
    pub color: u32,
}

// NOTE: max 8 args
default_args! {
    export pub fn rectv(
        x: i32 = 0,
        y: i32 = 0,
        w: u32 = 0,
        h: u32 = 0,
        fill: u32 = 0xffffffff,
        rotation_deg: i32 = 0,
        rotation_origin: (i32, i32) = (0, 0),
        border: Border = Default::default(),
    ) {
        rectfillv(
            x, y, w, h, fill,
            rotation_deg, rotation_origin.0, rotation_origin.1,
            border.radius, border.radius, border.size, border.color
        )
    }
}

pub fn circfill(x: i32, y: i32, d: u32, fill: u32) {
    ffi::canvas::circfill(x, y, d, fill)
}

default_args! {
    export pub fn circ(d: u32 = 0, x: i32 = 0, y: i32 = 0, fill: u32 = 0xffffffff) {
        circfill(x, y, d, fill)
    }
}

pub fn rectfill(x: i32, y: i32, w: u32, h: u32, fill: u32) {
    ffi::canvas::rectfill(x, y, w, h, fill)
}

default_args! {
    export pub fn rect(w: u32 = 0, h: u32 = 0, x: i32 = 0, y: i32 = 0, fill: u32 = 0xffffffff) {
        rectfill(x, y, w, h, fill)
    }
}

pub fn subsprite(x: i32, y: i32, w: u32, h: u32, sx: u32, sy: u32, sw: u32, sh: u32) {
    ffi::canvas::subsprite(x, y, w, h, sx, sy, sw, sh)
}

default_args! {
    export pub fn subsprite(x: i32 = 0, y: i32 = 0, w: u32 = 0, h: u32 = 0, sx: u32 = 0, sy: u32 = 0, sw: u32 = 0, sh: u32 = 0) {
        let sw = if sw == 0 { w } else { sw };
        let sh = if sh == 0 { h } else { sh };
        subsprite(x, y, w, h, sx, sy, sw, sh)
    }
}

pub fn sprite(name: &str, x: i32, y: i32, fps: u32, deg: i32) {
    let ptr = name.as_ptr();
    let len = name.len() as u32;
    ffi::canvas::sprite_by_key(ptr, len, x, y, fps, deg)
}

default_args! {
    export pub fn sprite(name: &str, x: i32 = 0, y: i32 = 0, fps: u32 = fps::MEDIUM, deg: i32 = 0) {
        sprite(name, x, y, fps, deg)
    }
}

pub fn text(x: i32, y: i32, font: Font, color: u32, text: &str) {
    let ptr = text.as_ptr();
    let len = text.len() as u32;
    ffi::canvas::text(x, y, font.into(), color, ptr, len)
}

default_args! {
    export pub fn text(txt: &str, x: i32 = 0, y: i32 = 0, font: Font = Font::M, color: u32 = 0xffffffff) {
        text(x, y, font, color, txt)
    }
}

pub mod fps {
    pub const REALLY_SLOW: u32 = 1;
    pub const SLOW: u32 = 2;
    pub const MEDIUM: u32 = 4;
    pub const FAST: u32 = 10;
    pub const SUPER_FAST: u32 = 20;
}

pub enum Font {
    S = 0,
    M = 1,
    L = 2,
}

impl From<u8> for Font {
    /// Converts a u8 value into its corresponding Font.
    fn from(value: u8) -> Self {
        match value {
            0 => Font::S,
            1 => Font::M,
            2 => Font::L,
            n => Self::from(n % 3),
        }
    }
}

impl Into<u8> for Font {
    /// Converts a Font into its corresponding u8 value.
    fn into(self) -> u8 {
        self as u8
    }
}
