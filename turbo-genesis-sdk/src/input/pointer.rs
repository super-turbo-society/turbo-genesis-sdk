use crate::{bounds::Bounds, canvas, ffi, serialize};
use num_traits::NumCast;
use serialize::Borsh;
use std::ops::Deref;
use turbo_genesis_abi::TurboPointer;

/// Wrapper around the ABI-defined `TurboPointer`, representing a pointer (e.g. touch or mouse) in fixed screen-space (pixels).
#[derive(Debug)]
pub struct FixedPointer(TurboPointer);

impl FixedPointer {
    /// Returns whether the pointer is currently intersecting a given screen-space bounding box.
    fn intersects_bounds(&self, bounds: Bounds) -> bool {
        bounds.intersects_xy(self.xy())
    }
}

/// Enables transparent access to fields and methods on the inner `TurboPointer`.
impl Deref for FixedPointer {
    type Target = TurboPointer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Wrapper around the ABI-defined `TurboPointer`, transformed into relative (camera/world-space) coordinates.
#[derive(Debug)]
pub struct RelativePointer(TurboPointer);

impl RelativePointer {
    /// Returns whether the pointer is currently intersecting a given world-space bounding box.
    fn intersects_bounds(&self, bounds: Bounds) -> bool {
        bounds.intersects_xy(self.xy())
    }
}

/// Enables transparent access to fields and methods on the inner `TurboPointer`.
impl Deref for RelativePointer {
    type Target = TurboPointer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Retrieves the pointer position in fixed screen-space (pixel) coordinates.
pub fn fixed() -> FixedPointer {
    // Allocate a fixed-size buffer to hold the serialized TurboPointer data.
    let data = &mut [0; std::mem::size_of::<TurboPointer>()];

    // Call FFI to fill the buffer with the current pointer state.
    ffi::input::mouse(data.as_mut_ptr());

    // Deserialize from bytes into a TurboPointer.
    let inner = TurboPointer::try_from_slice(data).expect("Could not deserialize Pointer");

    // Wrap in the FixedPointer type and return.
    FixedPointer(inner)
}

/// Retrieves the pointer position transformed into world-space (camera-relative) coordinates.
pub fn relative() -> RelativePointer {
    // Allocate buffer for raw FFI pointer data.
    let data = &mut [0; std::mem::size_of::<TurboPointer>()];

    // Populate buffer from FFI.
    ffi::input::mouse(data.as_mut_ptr());

    // Deserialize raw pointer data.
    let mut inner = TurboPointer::try_from_slice(data).expect("Could not deserialize Pointer");

    // Get current camera transform: position (x, y) and zoom (z).
    let (x, y, z) = canvas::camera::xyz();

    // Get screen resolution in pixels.
    let (w, h) = canvas::resolution();
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);

    // Convert pointer's screen coordinates to world-space.
    let (mx, my) = (inner.x as f32, inner.y as f32);
    let rel_x = ((mx - cx) / z + x).round() as i32;
    let rel_y = ((my - cy) / z + y).round() as i32;

    // Overwrite pointer coordinates with transformed values.
    inner.x = rel_x;
    inner.y = rel_y;

    // Wrap and return the camera-relative pointer.
    RelativePointer(inner)
}
