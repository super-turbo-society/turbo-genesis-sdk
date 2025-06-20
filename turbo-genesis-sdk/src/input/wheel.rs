use crate::{ffi, serialize};
use serialize::Borsh;
use std::ops::Deref;
use turbo_types::Wheel as TurboWheel;

/// Wrapper around the ABI-defined `TurboWheel`, used for reading scroll wheel input.
#[derive(Debug)]
struct Wheel(TurboWheel);

/// Enables transparent access to the inner `TurboWheel` fields and methods.
impl Deref for Wheel {
    type Target = TurboWheel;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Retrieves the current scroll wheel state from the FFI layer and deserializes it.
pub fn get() -> Wheel {
    // Allocate a buffer sized for the ABI-encoded Wheel struct.
    let data = &mut [0; std::mem::size_of::<TurboWheel>()];

    // Call the FFI layer to write serialized Wheel data into the buffer.
    ffi::input::mouse(data.as_mut_ptr());

    // Deserialize the buffer into a `TurboWheel`.
    let inner = TurboWheel::try_from_slice(data).expect("Could not deserialize Wheel");

    // Wrap the deserialized value in our local `Wheel` type.
    Wheel(inner)
}
