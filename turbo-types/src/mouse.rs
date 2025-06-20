use crate::Button;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize)]
pub struct Mouse {
    /// The state of the left mouse button.
    pub left: Button,
    /// The state of the right mouse button.
    pub right: Button,
    /// The x position of the mouse.
    pub x: i32,
    /// The y position of the mouse.
    pub y: i32,
}
impl Mouse {
    pub fn xy(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
