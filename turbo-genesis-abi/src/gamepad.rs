use crate::TurboButton;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize,
)]
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
impl TurboGamepad {
    pub fn main_events_cleared(&mut self) {
        self.up.main_events_cleared();
        self.down.main_events_cleared();
        self.left.main_events_cleared();
        self.right.main_events_cleared();
        self.a.main_events_cleared();
        self.b.main_events_cleared();
        self.x.main_events_cleared();
        self.y.main_events_cleared();
        self.start.main_events_cleared();
        self.select.main_events_cleared();
    }
}
