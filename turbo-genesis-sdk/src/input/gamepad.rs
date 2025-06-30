use borsh::BorshDeserialize;
use std::ops::Deref;
use turbo_genesis_abi::TurboGamepad;

/// Wrapper around the ABI-defined `TurboGamepad` type with local extensions or trait impls.
#[derive(Debug)]
pub struct Gamepad(TurboGamepad);

/// Enables transparent access to fields and methods on the inner `TurboGamepad`.
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
