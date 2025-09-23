//! Custom Sound Settings for Kira Static Handle.
//!

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug)]

/// Based on winit KeyCode which is based on the w3c UI Events spec.
/// See [`KeyboardEvent.code`](https://w3c.github.io/uievents-code/#code-value-tables)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize)]
pub enum TurboSoundSettingKey {
    LoopRegionStart,
    LoopRegionEnd,
    Volume,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct TurboSoundSetting {
    /// Unique name/key for the sound
    pub name: String,
    pub loop_region: [f64; 2],
    pub volume: f32,
    pub panning: f32,
}

pub struct Keys {
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
            name: "".to_owned(),
            loop_region: [0.0, 0.0],
            volume: 100.0,
            panning: 0,
        }
    }
}
