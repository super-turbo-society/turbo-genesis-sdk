use crate::TurboButton;
use borsh::{BorshDeserialize, BorshSerialize};
use num_traits::NumCast;

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize,
)]
pub struct TurboMouse {
    /// The state of the left mouse button.
    pub left: TurboButton,
    /// The state of the right mouse button.
    pub right: TurboButton,
    /// The x position of the mouse.
    pub x: i32,
    /// The y position of the mouse.
    pub y: i32,
    /// The x scroll delta
    pub delta_x: i32,
    /// The y scroll delta
    pub delta_y: i32,
}
impl TurboMouse {
    pub fn main_events_cleared(&mut self) {
        self.left.main_events_cleared();
        self.right.main_events_cleared();
    }
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
    pub fn scroll_xy(&self) -> (i32, i32) {
        (self.delta_x, self.delta_y)
    }
    pub fn pressed(&self) -> bool {
        self.left.pressed() || self.right.pressed()
    }
    pub fn just_pressed(&self) -> bool {
        self.left.just_pressed() || self.right.just_pressed()
    }
    pub fn released(&self) -> bool {
        self.left.released() || self.right.released()
    }
    pub fn just_released(&self) -> bool {
        self.left.just_released() || self.right.just_released()
    }
}
