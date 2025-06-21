use crate::TurboButton;
use borsh::{BorshDeserialize, BorshSerialize};
use num_traits::NumCast;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize)]
pub struct TurboPointer {
    /// The state of the left mouse button or touch
    pub state: TurboButton,
    /// The x position of the mouse cursor or most recent touch event
    pub x: i32,
    /// The y position of the mouse cursor or most recent touch event
    pub y: i32,
}
impl TurboPointer {
    pub fn intersects<X: NumCast, Y: NumCast, W: NumCast, H: NumCast>(
        &self,
        x: X,
        y: Y,
        w: W,
        h: H,
    ) -> bool {
        let x: i32 = NumCast::from(x).unwrap_or(0);
        let y: i32 = NumCast::from(y).unwrap_or(0);
        let w: i32 = NumCast::from(w).unwrap_or(0);
        let h: i32 = NumCast::from(h).unwrap_or(0);
        let left = x;
        let top = y;
        let right = x + w.saturating_sub(1);
        let bottom = y + h.saturating_sub(1);
        let (px, py) = self.xy();
        px >= left && px <= right && py >= top && py <= bottom
    }
    pub fn xy(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    pub fn pressed(&self) -> bool {
        self.state.pressed()
    }
    pub fn just_pressed(&self) -> bool {
        self.state.just_pressed()
    }
    pub fn released(&self) -> bool {
        self.state.released()
    }
    pub fn just_released(&self) -> bool {
        self.state.just_released()
    }
}
