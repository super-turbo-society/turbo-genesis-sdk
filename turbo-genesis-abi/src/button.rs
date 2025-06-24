use borsh::{BorshDeserialize, BorshSerialize};

/// Represents the state of an input (controller or mouse button) at a given moment.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize)]
#[borsh(use_discriminant = true)]
pub enum TurboButton {
    #[default]
    Released = 0,
    JustPressed = 1,
    Pressed = 2,
    JustReleased = 3,
}

impl TurboButton {
    /// Checks if the input state is JustReleased.
    pub fn just_released(&self) -> bool {
        *self == Self::JustReleased
    }

    /// Checks if the input state is Released or JustReleased.
    pub fn released(&self) -> bool {
        *self == Self::JustReleased || *self == Self::Released
    }

    /// Checks if the input state is JustPressed.
    pub fn just_pressed(&self) -> bool {
        *self == Self::JustPressed
    }

    /// Checks if the input state is Pressed or JustPressed.
    pub fn pressed(&self) -> bool {
        *self == Self::JustPressed || *self == Self::Pressed
    }

    /// Calculates the next input state based on the current state and the given `ElementState`.
    pub fn next(self, pressed: bool) -> Self {
        match (self, pressed) {
            (Self::Released, false) => Self::Released,
            (Self::Released, true) => Self::JustPressed,
            (Self::JustReleased, false) => Self::Released,
            (Self::JustReleased, true) => Self::JustPressed,
            (Self::JustPressed, false) => Self::JustReleased,
            (Self::JustPressed, true) => Self::Pressed,
            (Self::Pressed, false) => Self::JustReleased,
            (Self::Pressed, true) => Self::Pressed,
        }
    }

    /// Resets the input state to the appropriate values after main events have been cleared.
    pub fn main_events_cleared(&mut self) {
        *self = match *self {
            TurboButton::JustPressed => TurboButton::Pressed,
            TurboButton::JustReleased => TurboButton::Released,
            a => a,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_state_transitions() {
        // Test next() method of TurboButton enum
        assert_eq!(TurboButton::Released.next(true), TurboButton::JustPressed);
        assert_eq!(TurboButton::Released.next(false), TurboButton::Released);
        assert_eq!(
            TurboButton::JustReleased.next(true),
            TurboButton::JustPressed
        );
        assert_eq!(TurboButton::JustReleased.next(false), TurboButton::Released);
        assert_eq!(TurboButton::JustPressed.next(true), TurboButton::Pressed);
        assert_eq!(
            TurboButton::JustPressed.next(false),
            TurboButton::JustReleased
        );
        assert_eq!(TurboButton::Pressed.next(true), TurboButton::Pressed);
        assert_eq!(TurboButton::Pressed.next(false), TurboButton::JustReleased);
    }

    #[test]
    fn test_input_state_helper_methods() {
        // Test helper methods of TurboButton enum
        let state = TurboButton::JustPressed;
        assert!(state.just_pressed());
        assert!(state.pressed());
        assert!(!state.just_released());
        assert!(!state.released());

        let state = TurboButton::Released;
        assert!(!state.just_pressed());
        assert!(!state.pressed());
        assert!(!state.just_released());
        assert!(state.released());
    }
}
