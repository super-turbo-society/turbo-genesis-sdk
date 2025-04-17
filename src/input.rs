use num_traits::NumCast;

use crate::{bounds::Bounds, ffi};

pub fn gamepad(player: u32) -> Gamepad<Button> {
    let data = &mut [0; std::mem::size_of::<Gamepad<u8>>()];
    ffi::input::gamepad(player.into(), data.as_mut_ptr());
    let gamepad: Gamepad<u8> = *bytemuck::from_bytes(data);
    gamepad.into()
}

pub fn pointer() -> Pointer {
    let data = &mut [0; std::mem::size_of::<Mouse<u8>>()];
    ffi::input::mouse(0, data.as_mut_ptr());
    let mouse: Mouse<Button> = mouse(0).into();
    Pointer {
        x: mouse.position[0],
        y: mouse.position[1],
        state: mouse.left,
    }
}

/// Represents the state of an input (controller or mouse button) at a given moment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Button {
    Released = 0,
    JustPressed = 1,
    Pressed = 2,
    JustReleased = 3,
}

impl From<u8> for Button {
    /// Converts a u8 value into its corresponding Button.
    fn from(value: u8) -> Self {
        match value {
            0 => Button::Released,
            1 => Button::JustPressed,
            2 => Button::Pressed,
            3 => Button::JustReleased,
            _ => panic!("Invalid value for Button"),
        }
    }
}

impl Into<u8> for Button {
    /// Converts a Button into its corresponding u8 value.
    fn into(self) -> u8 {
        self as u8
    }
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

/// Represents the state of various gamepad buttons.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Gamepad<T: Copy> {
    /// The state of the up button.
    pub up: T,
    /// The state of the down button.
    pub down: T,
    /// The state of the left button.
    pub left: T,
    /// The state of the right button.
    pub right: T,
    /// The state of the A button.
    pub a: T,
    /// The state of the B button.
    pub b: T,
    /// The state of the X button.
    pub x: T,
    /// The state of the Y button.
    pub y: T,
    /// The state of the Start button.
    pub start: T,
    /// The state of the Select button.
    pub select: T,
}

impl Into<Gamepad<Button>> for Gamepad<u8> {
    fn into(self) -> Gamepad<Button> {
        Gamepad {
            up: self.up.into(),
            down: self.down.into(),
            left: self.left.into(),
            right: self.right.into(),
            a: self.a.into(),
            b: self.b.into(),
            x: self.x.into(),
            y: self.y.into(),
            start: self.start.into(),
            select: self.select.into(),
        }
    }
}

impl Into<Gamepad<u8>> for Gamepad<Button> {
    /// Converts Gamepad<Button> into Gamepad<u8>.
    fn into(self) -> Gamepad<u8> {
        Gamepad {
            up: self.up.into(),
            down: self.down.into(),
            left: self.left.into(),
            right: self.right.into(),
            a: self.a.into(),
            b: self.b.into(),
            x: self.x.into(),
            y: self.y.into(),
            start: self.start.into(),
            select: self.select.into(),
        }
    }
}

pub trait Point {
    fn intersects_bounds(&self, bound: Bounds) -> bool;
}
impl Point for (i32, i32) {
    fn intersects_bounds(&self, bounds: Bounds) -> bool {
        bounds.intersects_xy(*self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pointer {
    /// The x position of the mouse cursor or most recent touch event
    x: i32,
    /// The y position of the mouse cursor or most recent touch event
    y: i32,
    /// The state of the left mouse button or touch
    state: Button,
}
impl Pointer {
    pub fn relative_position(&self) -> (i32, i32) {
        let (x, y, z) = crate::canvas::camera::xyz();
        let (w, h) = crate::canvas::resolution();
        let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
        let (mx, my) = (self.x as f32, self.y as f32);
        let rel_x = ((mx - cx) / z + x).round() as i32;
        let rel_y = ((my - cy) / z + y).round() as i32;
        (rel_x, rel_y)
    }
    pub fn fixed_position(&self) -> (i32, i32) {
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

//------------------------------------------------------------------------------
// Everything below is deprecated and needs to get removed
//------------------------------------------------------------------------------

#[deprecated = "The Mouse API will be removed in a future version. Use the Pointer API."]
pub fn mouse(player: u32) -> Mouse<Button> {
    let data = &mut [0; std::mem::size_of::<Mouse<u8>>()];
    ffi::input::mouse(player.into(), data.as_mut_ptr());
    let mouse: Mouse<u8> = *bytemuck::from_bytes(data);
    mouse.into()
}

/// Represents the state of the left and right mouse buttons.
#[deprecated = "The Mouse API will be removed in a future version. Use the Pointer API."]
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Mouse<T: Copy> {
    /// The state of the left mouse button.
    pub left: T,
    /// The state of the right mouse button.
    pub right: T,
    /// The mouse wheel delta.
    pub wheel: [i32; 2],
    /// The position position.
    pub position: [i32; 2],
}

impl Into<Mouse<Button>> for Mouse<u8> {
    fn into(self) -> Mouse<Button> {
        Mouse {
            left: self.left.into(),
            right: self.right.into(),
            wheel: self.wheel,
            position: self.position,
        }
    }
}

impl Into<Mouse<u8>> for Mouse<Button> {
    /// Converts Mouse<Button> into Mouse<u8>.
    fn into(self) -> Mouse<u8> {
        Mouse {
            left: self.left.into(),
            right: self.right.into(),
            wheel: self.wheel,
            position: self.position,
        }
    }
}

/// Represents user input including button states, mouse button states, wheel delta, and position position.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PlayerInput<T: Copy + Clone> {
    /// The state of various buttons.
    pub gamepad: Gamepad<T>,
    /// The state of the left and right mouse buttons.
    pub mouse: Mouse<T>,
}

impl Into<PlayerInput<u8>> for PlayerInput<Button> {
    /// Converts PlayerInput<Button> into PlayerInput<u8>.
    fn into(self) -> PlayerInput<u8> {
        PlayerInput {
            gamepad: self.gamepad.into(),
            mouse: self.mouse.into(),
        }
    }
}

impl From<PlayerInput<u8>> for PlayerInput<Button> {
    /// Converts PlayerInput<u8> into PlayerInput<Button>.
    fn from(input: PlayerInput<u8>) -> Self {
        PlayerInput {
            gamepad: input.gamepad.into(),
            mouse: input.mouse.into(),
        }
    }
}

impl PlayerInput<Button> {
    /// Creates a new PlayerInput instance with default values for buttons, mouse, wheel, and position.
    pub fn new() -> Self {
        Self {
            gamepad: Gamepad {
                up: Button::Released,
                down: Button::Released,
                left: Button::Released,
                right: Button::Released,
                a: Button::Released,
                b: Button::Released,
                x: Button::Released,
                y: Button::Released,
                start: Button::Released,
                select: Button::Released,
            },
            mouse: Mouse {
                left: Button::Released,
                right: Button::Released,
                wheel: [0; 2],
                position: [0; 2],
            },
        }
    }

    /// Resets the input state of all buttons and mouse after main events have been cleared.
    pub fn main_events_cleared(&mut self) {
        self.gamepad.up.main_events_cleared();
        self.gamepad.down.main_events_cleared();
        self.gamepad.left.main_events_cleared();
        self.gamepad.right.main_events_cleared();
        self.gamepad.a.main_events_cleared();
        self.gamepad.b.main_events_cleared();
        self.gamepad.x.main_events_cleared();
        self.gamepad.y.main_events_cleared();
        self.gamepad.start.main_events_cleared();
        self.gamepad.select.main_events_cleared();

        // Mouse events don't happen every frame
        // So we have to manually transition from the Just* Buttons
        if let Button::JustPressed = self.mouse.left {
            self.mouse.left = self.mouse.left.next(true);
        }
        if let Button::JustReleased = self.mouse.left {
            self.mouse.left = self.mouse.left.next(false);
        }
        if let Button::JustPressed = self.mouse.right {
            self.mouse.right = self.mouse.right.next(true);
        }
        if let Button::JustReleased = self.mouse.right {
            self.mouse.right = self.mouse.right.next(false);
        }
        // Reset mouse wheel delta
        self.mouse.wheel = [0; 2];
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
        assert_eq!(Button::JustReleased.next(true), Button::JustPressed);
        assert_eq!(Button::JustReleased.next(false), Button::Released);
        assert_eq!(Button::JustPressed.next(true), Button::Pressed);
        assert_eq!(Button::JustPressed.next(false), Button::JustReleased);
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

    #[test]
    fn test_user_input_main_events_cleared() {
        let mut user_input = PlayerInput::new();

        // Simulate button and mouse state changes
        user_input.gamepad.a = Button::JustPressed;
        user_input.gamepad.x = Button::JustReleased;
        user_input.mouse.left = Button::JustPressed;
        user_input.mouse.right = Button::JustReleased;
        user_input.mouse.wheel = [1, -1];
        user_input.mouse.position = [10, 20];

        // Call main_events_cleared() method
        user_input.main_events_cleared();

        // Assert that button states have been updated
        assert_eq!(user_input.gamepad.a, Button::Pressed);
        assert_eq!(user_input.gamepad.x, Button::Released);

        // Assert that mouse states have been updated
        assert_eq!(user_input.mouse.left, Button::Pressed);
        assert_eq!(user_input.mouse.right, Button::Released);

        // Assert that wheel and position values have been reset
        let wheel = user_input.mouse.wheel;
        assert_eq!(wheel, [0, 0]);
        let position = user_input.mouse.position;
        assert_eq!(position, [10, 20]);
    }

    #[test]
    fn test_user_input_cast_to_u8_slice() {
        let mut user_input_buttons = PlayerInput::<Button>::new();
        user_input_buttons.gamepad.start = Button::Pressed;
        let user_input: PlayerInput<u8> = user_input_buttons.into();
        let slice = &[user_input];

        // Cast PlayerInput<Button> to &[u8]
        let bytes: &[u8] = bytemuck::cast_slice(slice);

        // Assert that the size of the bytes slice matches the size of the struct
        assert_eq!(bytes.len(), std::mem::size_of::<PlayerInput<Button>>());

        // Assert that the bytes slice has the same values as the struct
        let input_ref: PlayerInput<u8> = *bytemuck::from_bytes(bytes);
        assert_eq!(input_ref, user_input);

        let input_ref_buttons: PlayerInput<Button> = input_ref.into();
        assert_eq!(input_ref_buttons, user_input_buttons);
        assert_eq!(input_ref_buttons.gamepad.start, Button::Pressed);
    }
}
