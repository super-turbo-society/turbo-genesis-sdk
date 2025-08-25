//! Custom Sound Settings for Kira Static Handle.
//!

use std::collections::HashMap;

use borsh::BorshDeserialize;

#[derive(Debug)]
pub struct TurboSoundSetting {
    pub loop_region: [f64; 2],
    pub volume: f32,
}

/// Based on winit KeyCode which is based on the w3c UI Events spec.
/// See [`KeyboardEvent.code`](https://w3c.github.io/uievents-code/#code-value-tables)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize)]
pub enum TurboSoundSettingKey {
    LoopRegionStart,
    LoopRegionEnd,
    Volume,
}

// pub struct TurboSoundSetting {
//     : BTreeMap<TurboKeyCode, TurboButton>,
// }

impl TurboSoundSetting {
    pub fn new() -> Self {
        Self {
            loop_region: [0.0, 0.],
            volume: 0.0,
        }
    }
    pub fn update(&mut self, updates: HashMap<TurboSoundSettingKey, f64>) {
        for (key, value) in updates {
            match key {
                TurboSoundSettingKey::LoopRegionStart => self.loop_region[0] = value,
                TurboSoundSettingKey::LoopRegionEnd => self.loop_region[1] = value,
                TurboSoundSettingKey::Volume => self.volume = value as f32,
                _ => {} // ignores unknown keys
            }
        }
    }
}
