use crate::Button;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pointer {
    /// The x position of the mouse cursor or most recent touch event
    pub x: i32,
    /// The y position of the mouse cursor or most recent touch event
    pub y: i32,
    /// The state of the left mouse button or touch
    pub state: Button,
}
