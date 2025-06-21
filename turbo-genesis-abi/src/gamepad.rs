use crate::TurboButton;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize)]
pub struct TurboGamepad {
    /// The state of the up button.
    pub up: TurboButton,
    /// The state of the down button.
    pub down: TurboButton,
    /// The state of the left button.
    pub left: TurboButton,
    /// The state of the right button.
    pub right: TurboButton,
    /// The state of the A button.
    pub a: TurboButton,
    /// The state of the B button.
    pub b: TurboButton,
    /// The state of the X button.
    pub x: TurboButton,
    /// The state of the Y button.
    pub y: TurboButton,
    /// The state of the Start button.
    pub start: TurboButton,
    /// The state of the Select button.
    pub select: TurboButton,
}
