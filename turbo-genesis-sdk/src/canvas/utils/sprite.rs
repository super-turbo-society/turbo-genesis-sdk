use crate::sys::tick;
use crate::{canvas::flags, ffi};
use borsh::{BorshDeserialize, BorshSerialize};
use std::{collections::BTreeMap, ops::Div};

static mut TURBO_SPRITE_DATA_NONCE: u64 = 0;
static mut TURBO_SPRITE_DATA: BTreeMap<String, SpriteSourceData> = BTreeMap::new();

#[derive(Debug, Clone, Default, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct SpriteSourceData {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub animation_loop_count: u32,
    pub animation_direction: SpriteAnimationDirection,
    pub animation_frames: Vec<SpriteAnimationFrame>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum SpriteAnimationDirection {
    #[default]
    Forward,
    Reverse,
    PingPong,
    PingPongReverse,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct SpriteAnimationFrame {
    pub duration: f32,
}

pub fn get_source_data_nonce() -> u64 {
    unsafe { TURBO_SPRITE_DATA_NONCE }
}

pub fn get_source_data(name: &str) -> Option<SpriteSourceData> {
    #[allow(static_mut_refs)]
    unsafe {
        // Check latest sprite data nonce
        let nonce = ffi::canvas::get_sprite_data_nonce_v1();

        // If nonce has been updated, refresh data
        if TURBO_SPRITE_DATA_NONCE < nonce {
            // Get latest sprite data
            let mut data = vec![0; 1024 * 1024]; // up to 100kb of sprite data
            let data_ptr = data.as_mut_ptr();
            let mut len = data.len() as u32;
            let len_ptr = &mut len;
            ffi::canvas::get_sprite_data_v1(data_ptr, len_ptr);

            // Deserialize sprite data
            match <BTreeMap<String, SpriteSourceData>>::deserialize(&mut &data[..]) {
                // Update statics
                Ok(data) => {
                    TURBO_SPRITE_DATA_NONCE = nonce;
                    TURBO_SPRITE_DATA = data;
                }
                // Log the error
                Err(err) => {
                    crate::println!("Sprite data deserialization failed: {err:?}");
                }
            }
        }

        // Return the sprite data
        return TURBO_SPRITE_DATA.get(name).cloned();
    }
}

pub fn get_frame_index(sprite_data: &SpriteSourceData, speed: f32) -> usize {
    let elapsed_time = (tick() as f32 / 60.0) * 1000.0;
    let total_duration = sprite_data
        .animation_frames
        .iter()
        .map(|f| f.duration)
        .sum::<f32>()
        .div(speed);
    let animation_time = elapsed_time % total_duration;
    let mut accumulated_time = 0.0;
    let mut index = 0;
    for (i, frame) in sprite_data.animation_frames.iter().enumerate() {
        accumulated_time += frame.duration.div(speed);
        if animation_time < accumulated_time {
            index = i;
            break;
        }
    }
    index
}

pub fn draw(
    dx: i32,
    dy: i32,
    dw: u32,
    dh: u32,
    sx: u32,
    sy: u32,
    sw: i32,
    sh: i32,
    texture_x: i32,
    texture_y: i32,
    color: u32,
    background_color: u32,
    border_radius: u32,
    border_size: u32,
    border_color: u32,
    origin_x: i32,
    origin_y: i32,
    rotatation_deg: i32,
    flags: u32,
) {
    let dest_xy = ((dx as u64) << 32) | (dy as u64 & 0xffffffff);
    let dest_wh = ((dw as u64) << 32) | (dh as u32 as u64);
    let sprite_xy = ((sx as u64) << 32) | (sy as u64);
    let sprite_xy_offset = ((texture_x as u64) << 32) | (texture_y as u32 as u64);
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
        border_size,
        border_color,
        origin_xy,
        rotatation_deg,
        flags,
    )
}
