use crate::{bounds::Bounds, canvas};
use borsh::BorshDeserialize;
use num_traits::NumCast;
use std::ops::Deref;
use turbo_genesis_abi::{TurboMouse, TurboPointer};

/// A pointer input (e.g. mouse or touch) with position in fixed screen-space pixel coordinates.
#[derive(Debug)]
pub struct ScreenPointer(TurboPointer);

impl ScreenPointer {
    /// Returns whether the pointer is currently intersecting a given screen-space bounding box.
    pub fn intersects_bounds(&self, bounds: Bounds) -> bool {
        bounds.intersects_xy(self.xy())
    }
    /// Returns whether the pointer is currently intersecting a given world-space bounding box and it was just pressed.
    pub fn just_pressed_bounds(&self, bounds: Bounds) -> bool {
        self.just_pressed() && bounds.intersects_xy(self.xy())
    }
    /// Returns whether the pointer is currently intersecting a given world-space bounding box and it was pressed.
    pub fn pressed_bounds(&self, bounds: Bounds) -> bool {
        self.pressed() && bounds.intersects_xy(self.xy())
    }
}

/// Enables transparent access to fields and methods on the inner `TurboPointer`.
impl Deref for ScreenPointer {
    type Target = TurboPointer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A pointer input transformed into world-space coordinates, relative to the current camera view.
#[derive(Debug)]
pub struct WorldPointer(TurboPointer);

impl WorldPointer {
    /// Returns whether the pointer is currently intersecting a given world-space bounding box.
    pub fn intersects_bounds(&self, bounds: Bounds) -> bool {
        bounds.intersects_xy(self.xy())
    }
    /// Returns whether the pointer is currently intersecting a given world-space bounding box and it was just pressed.
    pub fn just_pressed_bounds(&self, bounds: Bounds) -> bool {
        self.just_pressed() && bounds.intersects_xy(self.xy())
    }
    /// Returns whether the pointer is currently intersecting a given world-space bounding box and it was pressed.
    pub fn pressed_bounds(&self, bounds: Bounds) -> bool {
        self.pressed() && bounds.intersects_xy(self.xy())
    }
}

/// Enables transparent access to fields and methods on the inner `TurboPointer`.
impl Deref for WorldPointer {
    type Target = TurboPointer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Retrieves the pointer position in fixed screen-space (pixel) coordinates.
pub fn screen() -> ScreenPointer {
    // Allocate a fixed-size buffer to hold the serialized TurboMouse data.
    let data = &mut [0; std::mem::size_of::<TurboMouse>()];

    // Call FFI to fill the buffer with the current pointer state.
    turbo_genesis_ffi::input::mouse(data.as_mut_ptr());

    // Deserialize from bytes into a TurboMouse.
    let mouse = match TurboMouse::deserialize(&mut &data[..]) {
        Err(err) => {
            crate::log!("[turbo] Could not deserialize Mouse: {:?}", err);
            panic!()
        }
        Ok(inner) => inner,
    };

    // Convert mouse to pointer
    let inner = TurboPointer {
        state: mouse.left,
        x: mouse.x,
        y: mouse.y,
    };

    // Wrap in the ScreenPointer type and return.
    ScreenPointer(inner)
}

/// Retrieves the pointer position transformed into world-space (camera-relative) coordinates.
pub fn world() -> WorldPointer {
    // Allocate buffer for raw FFI pointer data.
    let data = &mut [0; std::mem::size_of::<TurboMouse>()];

    // Populate buffer from FFI.
    turbo_genesis_ffi::input::mouse(data.as_mut_ptr());

    // Deserialize raw pointer data.
    let mouse = match TurboMouse::deserialize(&mut &data[..]) {
        Err(err) => {
            crate::log!("[turbo] Could not deserialize Mouse: {:?}", err);
            panic!()
        }
        Ok(inner) => inner,
    };

    // Convert mouse to pointer
    let mut inner = TurboPointer {
        state: mouse.left,
        x: mouse.x,
        y: mouse.y,
    };

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
    WorldPointer(inner)
}
