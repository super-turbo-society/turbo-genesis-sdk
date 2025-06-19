use crate::Button;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gamepad {
    /// The state of the up button.
    pub up: Button,
    /// The state of the down button.
    pub down: Button,
    /// The state of the left button.
    pub left: Button,
    /// The state of the right button.
    pub right: Button,
    /// The state of the A button.
    pub a: Button,
    /// The state of the B button.
    pub b: Button,
    /// The state of the X button.
    pub x: Button,
    /// The state of the Y button.
    pub y: Button,
    /// The state of the Start button.
    pub start: Button,
    /// The state of the Select button.
    pub select: Button,
}
