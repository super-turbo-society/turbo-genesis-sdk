//! Gamepad input handling.
//!
//! Provides functionality to fetch and represent the current state of a gamepad
//! using the TurboGenesis ABI and FFI layer. This module allows access to gamepad
//! button states for an SNES-like controller layout in a type-safe manner.

use borsh::BorshDeserialize;
use std::ops::Deref;
use turbo_genesis_abi::TurboGamepad;

/// Represents the current state of a gamepad and provides access to its buttons.
#[derive(Debug)]
pub struct Gamepad(TurboGamepad);

impl Deref for Gamepad {
    type Target = TurboGamepad;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Fetches the current gamepad state from the FFI layer and deserializes it into a `Gamepad`.
pub fn get(gamepad_index: usize) -> Gamepad {
    // Allocate a fixed-size buffer matching the ABI size of Gamepad.
    let data = &mut [0; std::mem::size_of::<Gamepad>()];

    // Call the FFI function to populate the buffer with serialized gamepad data.
    turbo_genesis_ffi::input::gamepad(gamepad_index as u32, data.as_mut_ptr());

    // Deserialize the buffer into a TurboGamepad using the Borsh ABI.
    let inner = match TurboGamepad::deserialize(&mut &data[..]) {
        Err(err) => {
            crate::log!("[turbo] Could not deserialize Gamepad: {:?}", err);
            panic!()
        }
        Ok(inner) => inner,
    };

    // Wrap the ABI value in our local `Gamepad` type.
    Gamepad(inner)
}
