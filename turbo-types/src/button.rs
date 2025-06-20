use borsh::{BorshDeserialize, BorshSerialize};

/// Represents the state of an input (controller or mouse button) at a given moment.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize)]
#[borsh(use_discriminant = true)]
pub enum Button {
    Released = 0,
    JustPressed = 1,
    Pressed = 2,
    JustReleased = 3,
}

impl Button {
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
            Button::JustPressed => Button::Pressed,
            Button::JustReleased => Button::Released,
            a => a,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_state_transitions() {
        // Test next() method of Button enum
        assert_eq!(Button::Released.next(true), Button::JustPressed);
        assert_eq!(Button::Released.next(false), Button::Released);
        assert_eq!(
            Button::JustReleased.next(true),
            Button::JustPressed
        );
        assert_eq!(Button::JustReleased.next(false), Button::Released);
        assert_eq!(Button::JustPressed.next(true), Button::Pressed);
        assert_eq!(
            Button::JustPressed.next(false),
            Button::JustReleased
        );
        assert_eq!(Button::Pressed.next(true), Button::Pressed);
        assert_eq!(Button::Pressed.next(false), Button::JustReleased);
    }

    #[test]
    fn test_input_state_helper_methods() {
        // Test helper methods of Button enum
        let state = Button::JustPressed;
        assert!(state.just_pressed());
        assert!(state.pressed());
        assert!(!state.just_released());
        assert!(!state.released());

        let state = Button::Released;
        assert!(!state.just_pressed());
        assert!(!state.pressed());
        assert!(!state.just_released());
        assert!(state.released());
    }
}
