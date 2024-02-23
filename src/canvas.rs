use std::collections::HashMap;

use crate::ffi;
use borsh::{BorshDeserialize, BorshSerialize};

//------------------------------------------------------------------------------
// Canvas Size
//------------------------------------------------------------------------------

pub fn canvas_size() -> [u32; 2] {
    let res = ffi::sys::resolution();
    let w = res & 0xffff;
    let h = res >> 16;
    [w, h]
}

#[macro_export]
macro_rules! canvas_size {
    () => {{
        $crate::canvas::canvas_size()
    }};
}

//------------------------------------------------------------------------------
// Clear
//------------------------------------------------------------------------------

pub fn clear(color: u32) {
    ffi::canvas::clear(color)
}

#[macro_export]
macro_rules! clear {
    () => {{
        $crate::clear!(0x000000ff)
    }};
    (color = $color:expr) => {{
        $crate::clear!(color = $color,)
    }};
    ($color:expr) => {{
        $crate::clear!(color = $color,)
    }};
    ($( $key:ident = $val:expr ),* $(,)*) => {{
        let mut color: u32 = 0x000000ff;
        $($crate::paste::paste!{ [< $key >] = $val; })*
        $crate::canvas::clear(color)
    }};
}

//------------------------------------------------------------------------------
// Camera
//------------------------------------------------------------------------------

pub fn get_camera() -> [i32; 2] {
    let cam = ffi::canvas::get_camera();
    let x = (cam >> 16) as i32;
    let y = (cam & 0xffff) as i32;
    [x, y]
}

#[macro_export]
macro_rules! cam {
    () => {{
        $crate::canvas::get_camera()
    }};
}

pub fn set_camera(x: i32, y: i32) {
    ffi::canvas::set_camera(x, y)
}

#[macro_export]
macro_rules! set_cam {
    ($( $key:ident = $val:expr ),* $(,)*) => {{
        let [mut x, mut y] = $crate::canvas::get_camera();
        $($crate::paste::paste!{ [< $key >] = $val; })*
        $crate::canvas::set_camera(x, y)
    }};
}

#[macro_export]
macro_rules! move_cam {
    ($( $key:ident = $val:expr ),* $(,)*) => {{
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        $($crate::paste::paste!{ [< $key >] = $val; })*
        let [cx, cy] = $crate::canvas::get_camera();
        $crate::canvas::set_camera(cx + x, cy + y)
    }};
}

//------------------------------------------------------------------------------
// Sprite
//------------------------------------------------------------------------------

pub mod fps {
    pub const REALLY_SLOW: u32 = 1;
    pub const SLOW: u32 = 2;
    pub const MEDIUM: u32 = 4;
    pub const FAST: u32 = 10;
    pub const SUPER_FAST: u32 = 20;
}

#[derive(Debug, Clone, Default, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct SpriteSourceData {
    pub width: u32,
    pub height: u32,
    pub frames: Vec<(u32, u32)>,
}

pub fn get_sprite_data(name: &str) -> Option<SpriteSourceData> {
    unsafe {
        static mut SPRITE_DATA: Option<(u64, HashMap<String, SpriteSourceData>)> = None;
        if SPRITE_DATA == None {
            SPRITE_DATA = Some((0, HashMap::new()));
        }
        let prev_nonce = SPRITE_DATA.as_ref().unwrap().0;
        let nonce = ffi::canvas::get_sprite_data_nonce_v1();
        if prev_nonce >= nonce {
            return SPRITE_DATA.as_ref().unwrap().1.get(name).cloned();
        }
        let mut data: [u8; 2048] = [0; 2048]; // up to 2kb sprite data
        let data_ptr = data.as_mut_ptr();
        let mut len = data.len() as u32;
        let len_ptr = &mut len;
        ffi::canvas::get_sprite_data_v1(data_ptr, len_ptr);
        type SpriteData = Vec<(String, SpriteSourceData)>;
        let sprite_data = SpriteData::deserialize(&mut &data[..]);
        match sprite_data {
            Ok(data) => {
                SPRITE_DATA.as_mut().unwrap().0 = nonce;
                SPRITE_DATA.as_mut().unwrap().1 = data.into_iter().collect();
            }
            Err(err) => {
                crate::println!("Sprite data deserialization failed: {err:?}");
            }
        }
        return SPRITE_DATA.as_ref().unwrap().1.get(name).cloned();
    }
}

pub fn draw_sprite(
    dx: i32,
    dy: i32,
    dw: u32,
    dh: u32,
    sx: u32,
    sy: u32,
    sw: i32,
    sh: i32,
    color: u32,
    rotatation_deg: i32,
) {
    let dest_xy = ((dx as u64) << 32) | (dy as u64 & 0xffffffff);
    let dest_wh = ((dw as u64) << 32) | (dh as u32 as u64);
    let sprite_xy = ((sx as u64) << 32) | (sy as u64);
    let sprite_wh = ((sw as u64) << 32) | (sh as u32 as u64);
    let fill_ab = color as u64;
    ffi::canvas::draw_quad_v1(
        dest_xy,
        dest_wh,
        sprite_xy,
        sprite_wh,
        fill_ab,
        0,
        0,
        0,
        0,
        rotatation_deg,
    )
}

#[macro_export]
macro_rules! sprite {
    ($name:expr) => {{
        $crate::spr!($name,)
    }};
    ($name:expr, $( $key:ident = $val:expr ),* $(,)*) => {{
        if let Some(sprite_data) = &$crate::canvas::get_sprite_data($name) {
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            let mut w: u32 = 0;
            let mut h: u32 = 0;
            let mut color: u32 = 0xffffffff;
            let mut opacity: f32 = -1.0;
            let mut rotate: i32 = 0;
            let mut scale_x: f32 = 1.0;
            let mut scale_y: f32 = 1.0;
            let mut flip_x: bool = false;
            let mut flip_y: bool = false;
            let mut fps: u32 = 0;
            $($crate::paste::paste!{ [< $key >] = $val; })*
            if opacity >= 0.0 {
                let x = (255.0 * opacity);
                color = 0xffffffff << 8 | (x as u32);
            }
            let sw = sprite_data.width;
            let sh = sprite_data.height;
            let dw = if w == 0 { (sw as f32 * scale_x) as u32 } else { w };
            let dh = if h == 0 { (sh as f32 * scale_y) as u32 } else { h };
            let sw = if flip_x { -(sw as i32) } else { sw as i32 };
            let sh = if flip_y { -(sh as i32) } else { sh as i32 };
            // Draw each frame at specified FPS
            if fps > 0 {
                let frame_rate = (60_usize).checked_div(fps as usize).unwrap_or(1);
                let i = $crate::sys::tick().checked_div(frame_rate).unwrap_or(0) % sprite_data.frames.len();
                let (sx, sy) = sprite_data.frames[i];
                $crate::canvas::draw_sprite(x, y, dw, dh, sx, sy, sw, sh, color, rotate);
            }
            // Draw all frames as one image
            else {
                for i in 0..sprite_data.frames.len() {
                    let (sx, sy) = sprite_data.frames[i];

                    // Convert angle to radians for trigonometric functions
                    let angle_rad = (rotate as f32).to_radians();

                    // Calculate the components of the distance along the x and y axes
                    let dist = dw as f32 * i as f32;
                    let dx = dist * angle_rad.cos();
                    let dy = dist * angle_rad.sin();

                    let frame_x = (x as f32 + dx) as i32;
                    let frame_y = (y as f32 + dy) as i32;

                    $crate::canvas::draw_sprite(frame_x, frame_y, dw, dh, sx, sy, sw, sh, color, rotate);
                }
            };
        }
    }};
}

//------------------------------------------------------------------------------
// Rectangle
//------------------------------------------------------------------------------

pub fn draw_rect(
    color: u32,
    dx: i32,
    dy: i32,
    dw: u32,
    dh: u32,
    border_radius: u32,
    border_size: u32,
    border_color: u32,
    rotation_deg: i32,
) {
    let dest_xy = ((dx as u64) << 32) | (dy as u32 as u64);
    let dest_wh = ((dw as u64) << 32) | (dh as u32 as u64);
    let fill_ab = (color as u64) << 32;
    ffi::canvas::draw_quad_v1(
        dest_xy,
        dest_wh,
        0,
        0,
        fill_ab,
        border_radius,
        border_size,
        border_color,
        0,
        rotation_deg,
    )
}

#[macro_export]
macro_rules! rect {
    ($( $key:ident = $val:expr ),* $(,)*) => {{
        let mut color: u32 = 0xffffffff;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut w: u32 = 0;
        let mut h: u32 = 0;
        let mut border_radius: u32 = 0;
        let mut border_width: u32 = 0;
        let mut border_color: u32 = 0xffffffff;
        let mut rotate: i32 = 0;
        let mut scale_x: f32 = 1.0;
        let mut scale_y: f32 = 1.0;
        $($crate::paste::paste!{ [< $key >] = $val; })*
        w = (w as f32 * scale_x) as u32;
        h = (h as f32 * scale_y) as u32;
        $crate::canvas::draw_rect(
            color,
            x, y, w, h,
            border_radius, border_width, border_color,
            rotate
        )
    }};
}

//------------------------------------------------------------------------------
// Circle
//------------------------------------------------------------------------------

#[macro_export]
macro_rules! circ {
    ($( $key:ident = $val:expr ),* $(,)*) => {{
        let mut color: u32 = 0xffffffff;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut d: u32 = 0;
        let mut border_width: u32 = 0;
        let mut border_color: u32 = 0xffffffff;
        let mut rotate: i32 = 0;
        let mut scale_x: f32 = 1.0;
        let mut scale_y: f32 = 1.0;
        $($crate::paste::paste!{ [< $key >] = $val; })*
        let border_radius = d;
        let mut w = d;
        let mut h = d;
        w = (w as f32 * scale_x) as u32;
        h = (h as f32 * scale_y) as u32;
        $crate::canvas::draw_rect(
            color,
            x, y, w, h,
            border_radius, border_width, border_color,
            rotate
        )
    }};
}

//------------------------------------------------------------------------------
// Ellipse
//------------------------------------------------------------------------------

#[macro_export]
macro_rules! ellipse {
    ($( $key:ident = $val:expr ),* $(,)*) => {{
        let mut color: u32 = 0xffffffff;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut w: u32 = 0;
        let mut h: u32 = 0;
        let mut border_width: u32 = 0;
        let mut border_color: u32 = 0xffffffff;
        let mut rotate: i32 = 0;
        let mut scale_x: f32 = 1.0;
        let mut scale_y: f32 = 1.0;
        $($crate::paste::paste!{ [< $key >] = $val; })*
        w = (w as f32 * scale_x) as u32;
        h = (h as f32 * scale_y) as u32;
        let border_radius = w.max(h);
        $crate::canvas::draw_rect(
            color,
            x, y, w, h,
            border_radius, border_width, border_color,
            rotate
        )
    }};
}

//------------------------------------------------------------------------------
// Text
//------------------------------------------------------------------------------

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

pub fn text(x: i32, y: i32, font: Font, color: u32, text: &str) {
    let ptr = text.as_ptr();
    let len = text.len() as u32;
    ffi::canvas::text(x, y, font.into(), color, ptr, len)
}

#[macro_export]
macro_rules! text {
    ($text:expr) => {{
        $crate::text!($text,)
    }};
    ($text:expr, $( $key:ident = $val:expr ),* $(,)*) => {{
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut font: Font = Font::M;
        let mut color: u32 = 0xffffffff;
        $($crate::paste::paste!{ [< $key >] = $val; })*
        $crate::canvas::text(x, y, font, color, $text)
    }};
}
