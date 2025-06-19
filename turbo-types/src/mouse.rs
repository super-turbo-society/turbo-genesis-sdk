use crate::Button;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mouse {
    /// The state of the left mouse button.
    pub left: Button,
    /// The state of the right mouse button.
    pub right: Button,
    /// The mouse wheel delta.
    pub wheel: [i32; 2],
    /// The position position.
    pub position: [i32; 2],
}
