//! Custom Sound Settings for Kira Static Handle.
//!

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct TurboSoundSetting {
    pub loop_region: [f64; 2],
    pub volume: f32,
    pub panning: f32,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct KeySetting {
    pub key: String,
    pub setting: TurboSoundSetting,
}

impl TurboSoundSetting {
    pub fn new() -> Self {
        Self {
            loop_region: [0.0, 0.0],
            volume: 1.0,
            panning: 0.0,
        }
    }
}
