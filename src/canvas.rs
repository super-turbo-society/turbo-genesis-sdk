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
        $($crate::paste::paste!{ [< $key >] = clear!(@coerce $key, $val); })*
        $crate::canvas::clear(color)
    }};
    (@coerce color, $val:expr) => { $val as u32; };
}

//------------------------------------------------------------------------------
// Camera
//------------------------------------------------------------------------------

#[deprecated(since = "0.6.0", note = "please use `get_camera2` instead")]
pub fn get_camera() -> [i32; 2] {
    let cam = ffi::canvas::get_camera();
    let x = ((cam >> 16) as i16) as i32;
    let y = (cam as i16) as i32;
    [x, y]
}

#[deprecated(since = "0.6.0", note = "please use `set_camera2` instead")]
pub fn set_camera(x: i32, y: i32) {
    ffi::canvas::set_camera(x, y)
}

pub fn get_camera2() -> (f32, f32, f32) {
    let mut cam: [f32; 3] = [0.; 3];
    ffi::canvas::get_camera2(cam.as_mut_ptr());
    (cam[0], cam[1], cam[2])
}

pub fn set_camera2(x: f32, y: f32, z: f32) {
    ffi::canvas::set_camera2(x, y, f32::max(z, 0.0));
}

#[macro_export]
macro_rules! cam {
    () => {{
        let (x, y, z) = $crate::canvas::get_camera2();
        (x as i32, y as i32, z)
    }};
}

#[macro_export]
macro_rules! set_cam {
    ($( $key:ident = $val:expr ),* $(,)*) => {{
        let (mut x, mut y, mut z) = $crate::canvas::get_camera2();
        $(paste::paste! { [< $key >] = set_cam!(@coerce $key, $val); })*
        $crate::canvas::set_camera2(x, y, z)
    }};
    (@coerce x, $val:expr) => { $val as f32; };
    (@coerce y, $val:expr) => { $val as f32; };
    (@coerce z, $val:expr) => { $val as f32; };
}

#[macro_export]
macro_rules! move_cam {
    ($( $key:ident = $val:expr ),* $(,)*) => {{
        let mut x: f32 = 0.;
        let mut y: f32 = 0.;
        let mut z: f32 = 0.;
        $(paste::paste! { [< $key >] = move_cam!(@coerce $key, $val); })*
        let (cx, cy, cz) = $crate::canvas::get_camera2();
        $crate::canvas::set_camera2(cx + x, cy + y, cz + z)
    }};
    (@coerce x, $val:expr) => { $val as f32; };
    (@coerce y, $val:expr) => { $val as f32; };
    (@coerce z, $val:expr) => { $val as f32; };
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
        let mut data: [u8; 4096] = [0; 4096]; // up to 4kb sprite data
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
    tx: i32,
    ty: i32,
    color: u32,
    background_color: u32,
    border_radius: u32,
    origin_x: i32,
    origin_y: i32,
    rotatation_deg: i32,
    flags: u32,
) {
    let dest_xy = ((dx as u64) << 32) | (dy as u64 & 0xffffffff);
    let dest_wh = ((dw as u64) << 32) | (dh as u32 as u64);
    let sprite_xy = ((sx as u64) << 32) | (sy as u64);
    let sprite_xy_offset = ((tx as u64) << 32) | (ty as u32 as u64);
    let sprite_wh = ((sw as u64) << 32) | (sh as u32 as u64);
    let origin_xy = ((origin_x as u64) << 32) | (origin_y as u64 & 0xffffffff);
    let fill_ab = (background_color as u64) << 32 | (color as u64 & 0xffffffff);
    ffi::canvas::draw_quad2(
        dest_xy,
        dest_wh,
        sprite_xy,
        sprite_wh,
        sprite_xy_offset,
        fill_ab,
        border_radius,
        0,
        0,
        origin_xy,
        rotatation_deg,
        flags,
    )
}

pub mod flags {
    // Repeats the sprite within the containing quad
    pub const SPRITE_REPEAT: u32 = 1 << 0;
    // Scales a sprite to fit the dimensions of the containing quad
    pub const SPRITE_COVER: u32 = 2 << 0;
}

#[macro_export]
macro_rules! sprite {
    ($name:expr) => {{
        $crate::sprite!($name,)
    }};
    ($name:expr, $( $key:ident = $val:expr ),* $(,)*) => {{
        if let Some(sprite_data) = &$crate::canvas::get_sprite_data($name) {
            let num_frames = sprite_data.frames.len();
            let default_sw = sprite_data.width;
            let default_sh = sprite_data.height;
            let mut sw: u32 = 0;
            let mut sh: u32 = 0;
            let mut sx: u32 = 0;
            let mut sy: u32 = 0;
            let mut tx: i32 = 0;
            let mut ty: i32 = 0;
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            let mut w: u32 = u32::MAX;
            let mut h: u32 = u32::MAX;
            let mut color: u32 = 0xffffffff;
            let mut background_color: u32 = 0x00000000;
            let mut border_radius: u32 = 0;
            let mut opacity: f32 = -1.0;
            let mut origin_x: i32 = 0;
            let mut origin_y: i32 = 0;
            let mut rotate: i32 = 0;
            let mut scale: f32 = 1.0;
            let mut scale_x: f32 = 1.0;
            let mut scale_y: f32 = 1.0;
            let mut flip_x: bool = false;
            let mut flip_y: bool = false;
            let mut fps: u32 = 0;
            let mut repeat: bool = false;
            $($crate::paste::paste!{ [< $key >] = sprite!(@coerce $key, $val); })*

            // Initialize flags
            let mut flags: u32 = 0;

            // Sprite repeat
            if repeat { flags |= $crate::canvas::flags::SPRITE_REPEAT; }

            // Set opacity
            if opacity >= 0.0 {
                // Apply gamma correction
                let gamma = 2.2;
                let linear_opacity = opacity.powf(1.0 / gamma);

                // Calculate the alpha value
                let alpha = (255.0 * linear_opacity) as u32;

                // Combine the alpha with the color
                // color = color << 8 | alpha;

                color = alpha << 32 | (color & 0xffffff00);
            }

            // Adjust source size based on source position
            let sw = if sw == 0 { default_sw - sx } else { sw };
            let sh = if sh == 0 { default_sh - sy } else { sh };

            // Set destination size
            let dw = if w == u32::MAX { sw } else { w };
            let dh = if h == u32::MAX { sh } else { h };

            // Update scale
            scale_x *= scale;
            scale_y *= scale;

            // Set the cover flag if scaling is used
            if scale_x != 1. || scale_y != 1. { flags |= $crate::canvas::flags::SPRITE_COVER; }

            // Scale destination width and height
            let dw = (dw as f32 * scale_x) as u32;
            let dh = (dh as f32 * scale_y) as u32;

            // Flip sprite
            let sw = if flip_x { -(sw as i32) } else { sw  as i32 };
            let sh = if flip_y { -(sh as i32) } else { sh  as i32 };

            // Set transform origin
            let origin_x = ((origin_x as f32) * scale_x) as i32;
            let origin_y = ((origin_y as f32) * scale_y) as i32;

            // Draw each frame at specified FPS
            if fps > 0 {
                let frame_rate = (60_usize).checked_div(fps as usize).unwrap_or(1);
                let frames_len = sprite_data.frames.len();
                let (sx, sy) = if frames_len == 1 {
                    let frames_len = sprite_data.width as usize / sw as usize;
                    let i = $crate::sys::tick().checked_div(frame_rate).unwrap_or(0) % frames_len;
                    let (sx, sy) = sprite_data.frames[0];
                    (sx + (i as u32 * sw as u32), sy)
                } else {
                    let i = $crate::sys::tick().checked_div(frame_rate).unwrap_or(0) % frames_len;
                    sprite_data.frames[i]
                };

                $crate::canvas::draw_sprite(
                    x, y, dw, dh,
                    sx, sy, sw, sh, tx, ty,
                    color, background_color,
                    border_radius,
                    origin_x, origin_y,
                    rotate,
                    flags
                );
            }
            // Draw all frames as one image
            else {
                let abs_sw = sw.abs() as u32;
                let mut cx = sx;
                let mut rem_sw = abs_sw;
                for i in 0..num_frames {
                    // Apply offset to sprite frame source position
                    let (fx, fy) = sprite_data.frames[i];
                    let sx = cx + fx;
                    let sy = sy + fy;

                    // Handle offsets when animation multiple frames
                    if num_frames > 1 {
                        rem_sw = rem_sw.saturating_sub(abs_sw);
                        cx = if cx > 0 { (cx - abs_sw).max(0) } else { (cx + abs_sw).min(0)};
                        if sx > abs_sw { continue; }
                    }

                    // Convert angle to radians for trigonometric functions
                    let angle_rad = (rotate as f32).to_radians();

                    // Calculate the components of the distance along the x and y axes
                    let dist = dw as f32 * i as f32;
                    let dx = dist * angle_rad.cos();
                    let dy = dist * angle_rad.sin();
                    let dx = (x as f32 + dx) as i32;
                    let dy = (y as f32 + dy) as i32;

                    // Draw
                    $crate::canvas::draw_sprite(
                        dx, dy, dw, dh,
                        sx, sy, sw, sh, tx, ty,
                        color, background_color,
                        border_radius,
                        origin_x, origin_y,
                        rotate,
                        flags
                    );

                    // Stop drawing if width has been reached
                    if rem_sw == 0 { break; }
                }
            };
        }
    }};
    // Parent quad position and size. Crops the inner sprite slice
    (@coerce x, $val:expr) => { $val as i32; };
    (@coerce y, $val:expr) => { $val as i32; };
    (@coerce w, $val:expr) => { $val as u32; };
    (@coerce h, $val:expr) => { $val as u32; };

    // Sprite slice position and size relative to spritesheet
    (@coerce sx, $val:expr) => { $val as u32; };
    (@coerce sy, $val:expr) => { $val as u32; };
    (@coerce sw, $val:expr) => { $val as u32; };
    (@coerce sh, $val:expr) => { $val as u32; };

    // Sprite slice translation
    (@coerce tx, $val:expr) => { $val as i32; };
    (@coerce ty, $val:expr) => { $val as i32; };
    (@coerce repeat, $val:expr) => { $val as bool; };

    (@coerce color, $val:expr) => { $val as u32; };
    (@coerce background_color, $val:expr) => { $val as u32; };
    (@coerce border_radius, $val:expr) => { $val as u32; };
    (@coerce opacity, $val:expr) => { $val as f32; };

    // Transforms
    (@coerce origin_x, $val:expr) => { $val as i32; };
    (@coerce origin_y, $val:expr) => { $val as i32; };
    (@coerce rotate, $val:expr) => { $val as i32; };
    (@coerce scale, $val:expr) => { $val as f32; };
    (@coerce scale_x, $val:expr) => { $val as f32; };
    (@coerce scale_y, $val:expr) => { $val as f32; };
    (@coerce flip_x, $val:expr) => { $val as bool; };
    (@coerce flip_y, $val:expr) => { $val as bool; };

    // Animation
    (@coerce fps, $val:expr) => { $val as u32; };
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

        $($crate::paste::paste!{ [< $key >] = rect!(@coerce $key, $val); })*

        w = (w as f32 * scale_x) as u32;
        h = (h as f32 * scale_y) as u32;

        $crate::canvas::draw_rect(
            color,
            x, y, w, h,
            border_radius, border_width, border_color,
            rotate
        )
    }};
    (@coerce color, $val:expr) => { $val as u32; };
    (@coerce x, $val:expr) => { $val as i32; };
    (@coerce y, $val:expr) => { $val as i32; };
    (@coerce w, $val:expr) => { $val as u32; };
    (@coerce h, $val:expr) => { $val as u32; };
    (@coerce border_radius, $val:expr) => { $val as u32; };
    (@coerce border_width, $val:expr) => { $val as u32; };
    (@coerce border_color, $val:expr) => { $val as u32; };
    (@coerce rotate, $val:expr) => { $val as i32; };
    (@coerce scale_x, $val:expr) => { $val as f32; };
    (@coerce scale_y, $val:expr) => { $val as f32; };
}

#[macro_export]
macro_rules! path {
    ($( $key:ident = $val:expr ),* $(,)*) => {{
        let mut start: (i32, i32) = (0, 0);
        let mut end: (i32, i32) = (0, 0);
        let mut color: u32 = 0xffffffff;
        let mut width: u32 = 1;
        let mut border_radius: u32 = 0;
        $($crate::paste::paste!{ [< $key >] = path!(@coerce $key, $val); })*

        // Calculate differences and distance
        let delta_x = (end.0 - start.0) as f64;
        let delta_y = (end.1 - start.1) as f64;
        // let distance = ((delta_x * delta_x + delta_y * delta_y) as f64).sqrt() as u32;
        let distance = (delta_x.powi(2) + delta_y.powi(2)).sqrt() as u32;

        // Calculate the angle in radians
        let angle = ((delta_y).atan2(delta_x) * (180.0 / std::f64::consts::PI)) as i32;

        // Calculate the midpoint for placing the rectangle
        let x = (start.0 + end.0) / 2;
        let y = (start.1 + end.1) / 2;

        // Draw the rectangle as a thin line with rotation around its center
        $crate::canvas::draw_rect(
            color,
            x - (distance / 2) as i32, // Adjust x to start from the midpoint
            y - (width / 2) as i32,   // Center y based on line width
            distance,                 // Width of the rectangle is the distance
            width,                    // Height of the rectangle is the line width
            border_radius,            // Border radius (if any)
            0,                        // Border width (none)
            0,                        // Border color (none)
            angle                     // Rotation angle
        )
    }};
    (@coerce start, $val:expr) => { ($val.0 as i32, $val.1 as i32); };
    (@coerce end, $val:expr) => { ($val.0 as i32, $val.1 as i32); };
    (@coerce color, $val:expr) => { $val as u32; };
    (@coerce width, $val:expr) => { $val as u32; };
    (@coerce border_radius, $val:expr) => { $val as u32; };
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
        $($crate::paste::paste!{ [< $key >] = circ!(@coerce $key, $val); })*
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
    (@coerce color, $val:expr) => { $val as u32; };
    (@coerce x, $val:expr) => { $val as i32; };
    (@coerce y, $val:expr) => { $val as i32; };
    (@coerce d, $val:expr) => { $val as u32; };
    (@coerce border_width, $val:expr) => { $val as u32; };
    (@coerce border_color, $val:expr) => { $val as u32; };
    (@coerce rotate, $val:expr) => { $val as i32; };
    (@coerce scale_x, $val:expr) => { $val as f32; };
    (@coerce scale_y, $val:expr) => { $val as f32; };
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
        $($crate::paste::paste!{ [< $key >] = ellipse!(@coerce $key, $val); })*
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
    (@coerce color, $val:expr) => { $val as u32; };
    (@coerce x, $val:expr) => { $val as i32; };
    (@coerce y, $val:expr) => { $val as i32; };
    (@coerce w, $val:expr) => { $val as u32; };
    (@coerce h, $val:expr) => { $val as u32; };
    (@coerce border_width, $val:expr) => { $val as u32; };
    (@coerce border_color, $val:expr) => { $val as u32; };
    (@coerce rotate, $val:expr) => { $val as i32; };
    (@coerce scale_x, $val:expr) => { $val as f32; };
    (@coerce scale_y, $val:expr) => { $val as f32; };
}

//------------------------------------------------------------------------------
// Text
//------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum Font {
    S = 0,
    M = 1,
    L = 2,
    XL = 3,
}
impl Font {
    pub const ALL: [Self; 4] = [Self::S, Self::M, Self::L, Self::XL];
}
impl From<u8> for Font {
    /// Converts a u8 value into its corresponding Font.
    fn from(value: u8) -> Self {
        Self::ALL[value as usize % Self::ALL.len()]
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
        $($crate::paste::paste!{ [< $key >] = text!(@coerce $key, $val); })*
        $crate::canvas::text(x, y, font, color, $text)
    }};
    ($text:expr, $( $arg:expr ),* ; $( $key:ident = $val:expr ),* $(,)*) => {{
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut font: Font = Font::M;
        let mut color: u32 = 0xffffffff;
        $(paste::paste! { [< $key >] = text!(@coerce $key, $val); })*
        $crate::canvas::text(x, y, font, color, &format!($text, $($arg),*))
    }};
    (@coerce x, $val:expr) => { $val as i32; };
    (@coerce y, $val:expr) => { $val as i32; };
    (@coerce font, $val:expr) => { $val as Font; };
    (@coerce color, $val:expr) => { $val as u32; };
}
