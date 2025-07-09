use crate::{bounds::Bounds, canvas};
use borsh::BorshDeserialize;
use num_traits::NumCast;
use std::ops::Deref;
use turbo_genesis_abi::TurboMouse;

/// A wrapper around `TurboMouse` for screen-space (fixed-pixel) mouse data.
#[derive(Debug)]
pub struct ScreenMouse(TurboMouse);

impl ScreenMouse {
    /// Returns whether the mouse is currently intersecting a given bounding box.
    pub fn intersects_bounds(&self, bounds: Bounds) -> bool {
        bounds.intersects_xy(self.xy())
    }
    /// Returns whether the mouse is currently intersecting a given screen-space bounding box and its left button was just pressed.
    pub fn left_clicked_bounds(&self, bounds: Bounds) -> bool {
        self.left.just_pressed() && bounds.intersects_xy(self.xy())
    }
    /// Returns whether the mouse is currently intersecting a given screen-space bounding box and its right button was just pressed.
    pub fn right_clicked_bounds(&self, bounds: Bounds) -> bool {
        self.right.just_pressed() && bounds.intersects_xy(self.xy())
    }
}

/// Enables transparent access to the inner `TurboMouse` fields and methods.
impl Deref for ScreenMouse {
    type Target = TurboMouse;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A wrapper around `TurboMouse` for camera-relative (world-space) mouse data.
#[derive(Debug)]
pub struct WorldMouse(TurboMouse);

impl WorldMouse {
    /// Returns whether the mouse is currently intersecting a given world-space bounding box.
    pub fn intersects_bounds(&self, bounds: Bounds) -> bool {
        bounds.intersects_xy(self.xy())
    }
    /// Returns whether the mouse is currently intersecting a given world-space bounding box and its left button was just pressed.
    pub fn left_clicked_bounds(&self, bounds: Bounds) -> bool {
        self.left.just_pressed() && bounds.intersects_xy(self.xy())
    }
    /// Returns whether the mouse is currently intersecting a given world-space bounding box and its right button was just pressed.
    pub fn right_clicked_bounds(&self, bounds: Bounds) -> bool {
        self.right.just_pressed() && bounds.intersects_xy(self.xy())
    }
}

/// Enables transparent access to the inner `TurboMouse` fields and methods.
impl Deref for WorldMouse {
    type Target = TurboMouse;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Fetches mouse position in fixed screen coordinates (raw pixels).
pub fn screen() -> ScreenMouse {
    // Allocate a buffer the size of the serialized TurboMouse ABI.
    let data = &mut [0; std::mem::size_of::<TurboMouse>()];

    // Call the FFI function to populate the buffer with mouse data.
    turbo_genesis_ffi::input::mouse(data.as_mut_ptr());

    // Deserialize the buffer into a TurboMouse and wrap it.
    let inner = match TurboMouse::deserialize(&mut &data[..]) {
        Err(err) => {
            crate::log!("[turbo] Could not deserialize Mouse: {:?}", err);
            panic!()
        }
        Ok(inner) => inner,
    };

    // Wrap and return the camera-fixed mouse.
    ScreenMouse(inner)
}

/// Fetches mouse position transformed into camera-relative (world-space) coordinates.
pub fn world() -> WorldMouse {
    // Allocate a buffer the size of the serialized TurboMouse ABI.
    let data = &mut [0; std::mem::size_of::<TurboMouse>()];

    // Populate the buffer via FFI.
    turbo_genesis_ffi::input::mouse(data.as_mut_ptr());

    // Deserialize into a `TurboMouse`.
    let mut inner = match TurboMouse::deserialize(&mut &data[..]) {
        Err(err) => {
            crate::log!("[turbo] Could not deserialize Mouse: {:?}", err);
            panic!()
        }
        Ok(inner) => inner,
    };

    // Get current camera transform: position (x, y) and zoom (z).
    let (x, y, z) = canvas::camera::xyz();

    // Get the canvas resolution in pixels.
    let (w, h) = canvas::resolution();
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);

    // Convert screen coords to world-space coords.
    let (mx, my) = (inner.x as f32, inner.y as f32);
    let rel_x = ((mx - cx) / z + x).round() as i32;
    let rel_y = ((my - cy) / z + y).round() as i32;

    // Store transformed coordinates back into the mouse struct.
    inner.x = rel_x;
    inner.y = rel_y;

    // Wrap and return the camera-relative mouse.
    WorldMouse(inner)
}
