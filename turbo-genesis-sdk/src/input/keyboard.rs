use borsh::BorshDeserialize;
use std::ops::Deref;
use turbo_genesis_abi::{TurboKeyCode, TurboKeyboard};

/// Wrapper for the ABI-defined `TurboKeyCode`, allowing local trait impls and extensions.
#[derive(Debug)]
pub struct KeyCode(TurboKeyCode);

/// Enables transparent access to the inner `TurboKeyCode`.
impl Deref for KeyCode {
    type Target = TurboKeyCode;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Wrapper for the ABI-defined `TurboKeyboard`, enabling local methods and trait impls.
#[derive(Debug)]
pub struct Keyboard(TurboKeyboard);

/// Allows direct field and method access on the inner `TurboKeyboard`.
impl Deref for Keyboard {
    type Target = TurboKeyboard;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Retrieves the current keyboard state from the FFI layer and deserializes it into a `Keyboard`.
pub fn get() -> Keyboard {
    // Preallocate a buffer for serialized keyboard data.
    let data = &mut vec![0; 1024];

    // Prepare a mutable length pointer for the FFI call to populate.
    let mut len = 0;
    let len_ptr = &mut len;

    // Call into FFI to fill `data` with serialized keyboard bytes and update `len`.
    turbo_genesis_ffi::input::keyboard(data.as_mut_ptr(), len_ptr);

    // Deserialize the ABI bytes into a `TurboKeyboard`.
    let inner = TurboKeyboard::try_from_slice(data).expect("Could not deserialize Keyboard");

    // Wrap in local `Keyboard` type for ergonomic use.
    Keyboard(inner)
}
