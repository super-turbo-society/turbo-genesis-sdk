use num_traits::NumCast;

use crate::bounds::Bounds;

/// Retrieves the current camera position as an (x, y, z) tuple.
/// The values are filled by calling the FFI function `get_camera2`.
pub fn xyz() -> (f32, f32, f32) {
    let mut cam: [f32; 3] = [0.; 3];
    crate::ffi::canvas::get_camera2(cam.as_mut_ptr());
    (cam[0], cam[1], cam[2])
}

/// Retrieves the current camera position as an (x, y) tuple, ignoring z.
pub fn xy() -> (f32, f32) {
    let (x, y, _z) = xyz();
    (x, y)
}

/// Returns the current camera's x coordinate.
pub fn x() -> f32 {
    let (x, _y, _z) = xyz();
    x
}

/// Returns the current camera's y coordinate.
pub fn y() -> f32 {
    let (_x, y, _z) = xyz();
    y
}

/// Returns the current camera's z coordinate, which represents the zoom.
pub fn z() -> f32 {
    let (_x, _y, z) = xyz();
    z
}

/// Returns the current zoom level (alias for z()).
pub fn zoom() -> f32 {
    z()
}

/// Sets the camera's position to (x, y, z).
/// The x and y values are converted to f32; z is clamped to a minimum of 0.0.
pub fn set_xyz<X: NumCast, Y: NumCast>(x: X, y: Y, z: f32) {
    let x: f32 = NumCast::from(x).unwrap_or(0.0);
    let y: f32 = NumCast::from(y).unwrap_or(0.0);
    let z = f32::max(z, 0.0);
    crate::ffi::canvas::set_camera2(x, y, z);
}

/// Sets the camera's x and y coordinates while retaining the current z (zoom) value.
pub fn set_xy<X: NumCast, Y: NumCast>(x: X, y: Y) {
    let (_x, _y, z) = xyz();
    let x: f32 = NumCast::from(x).unwrap_or(0.0);
    let y: f32 = NumCast::from(y).unwrap_or(0.0);
    set_xyz(x, y, z);
}

/// Sets the camera's x coordinate, leaving y and z unchanged.
pub fn set_x<X: NumCast>(x: X) {
    let (_, y, z) = xyz();
    let x: f32 = NumCast::from(x).unwrap_or(0.0);
    set_xyz(x, y, z);
}

/// Sets the camera's y coordinate, leaving x and z unchanged.
pub fn set_y<Y: NumCast>(y: Y) {
    let (x, _y, z) = xyz();
    let y: f32 = NumCast::from(y).unwrap_or(0.0);
    set_xyz(x, y, z);
}

/// Sets the camera's z coordinate (zoom), leaving x and y unchanged.
pub fn set_z(z: f32) {
    let (x, y, _z) = xyz();
    set_xyz(x, y, z);
}

/// Moves the camera by the specified deltas in x, y, and z.
/// The current camera position is retrieved, the deltas are added, and then the new position is set.
pub fn move_xyz<X: NumCast, Y: NumCast>(delta_x: X, delta_y: Y, delta_z: f32) {
    let (x, y, z) = xyz();
    let delta_x: f32 = NumCast::from(delta_x).unwrap_or(0.0);
    let delta_y: f32 = NumCast::from(delta_y).unwrap_or(0.0);
    set_xyz(x + delta_x, y + delta_y, z + delta_z);
}

/// Moves the camera in the x and y directions by the specified deltas.
pub fn move_xy<X: NumCast, Y: NumCast>(delta_x: X, delta_y: Y) {
    let (x, y) = xy();
    let delta_x: f32 = NumCast::from(delta_x).unwrap_or(0.0);
    let delta_y: f32 = NumCast::from(delta_y).unwrap_or(0.0);
    set_xy(x + delta_x, y + delta_y);
}

/// Moves the camera in the x direction by the specified delta.
pub fn move_x<X: NumCast>(delta_x: X) {
    let delta_x: f32 = NumCast::from(delta_x).unwrap_or(0.0);
    set_x(x() + delta_x);
}

/// Moves the camera in the y direction by the specified delta.
pub fn move_y<Y: NumCast>(delta_y: Y) {
    let delta_y: f32 = NumCast::from(delta_y).unwrap_or(0.0);
    set_y(y() + delta_y);
}

/// Sets the camera's zoom (z value) to the given value.
pub fn set_zoom(z: f32) {
    let (x, y, _z) = xyz();
    set_xyz(x, y, z);
}

/// Moves the camera's zoom by the specified delta.
pub fn move_zoom(delta_z: f32) {
    set_z(z() + delta_z);
}

/// Resets the camera's x and y position to the center of the viewport.
/// The screen size is obtained from the parent module.
pub fn reset() {
    let (w, h) = crate::canvas::resolution();
    let x = (w / 2) as f32;
    let y = (h / 2) as f32;
    set_xyz(x, y, 1.)
}

/// Resets the camera's x coordinate to the horizontal center of the screen.
pub fn reset_x() {
    let x = (crate::canvas::resolution().0 / 2) as f32;
    set_x(x)
}

/// Resets the camera's y coordinate to the vertical center of the screen.
pub fn reset_y() {
    let y = (crate::canvas::resolution().1 / 2) as f32;
    set_y(y)
}

/// Resets both the camera's x and y coordinates to the center of the screen.
pub fn reset_xy() {
    let (w, h) = crate::canvas::resolution();
    let x = (w / 2) as f32;
    let y = (h / 2) as f32;
    set_xy(x, y)
}

/// Resets the camera's z coordinate (zoom) to 1.0 while keeping x and y centered.
pub fn reset_z() {
    let (w, h) = crate::canvas::resolution();
    let x = (w / 2) as f32;
    let y = (h / 2) as f32;
    set_xyz(x, y, 1.0)
}

/// Resets the camera's zoom to the default value (alias for reset_z).
pub fn reset_zoom() {
    reset_z()
}

/// Centers the camera on a target rectangle defined by (x, y, w, h).
///
/// # Parameters
/// - `x`, `y`: The top-left coordinates of the target rectangle.
/// - `w`, `h`: The width and height of the target rectangle.
pub fn focus_rect<X: NumCast, Y: NumCast, W: NumCast, H: NumCast>(x: X, y: Y, w: W, h: H) {
    let x: f32 = NumCast::from(x).unwrap_or(0.0);
    let y: f32 = NumCast::from(y).unwrap_or(0.0);
    let w: f32 = NumCast::from(w).unwrap_or(0.0);
    let h: f32 = NumCast::from(h).unwrap_or(0.0);
    // Compute the center of the target rectangle.
    let target_x = x + w / 2.0;
    let target_y = y + h / 2.0;
    // Center the camera on the computed target center.
    set_xy(target_x, target_y);
}

/// Centers the camera on a target Bounds.
pub fn focus_bounds(bounds: &Bounds) {
    let x = bounds.x as f32;
    let y = bounds.y as f32;
    let w = bounds.w as f32;
    let h = bounds.h as f32;
    // Compute the center of the target rectangle.
    let target_x = x + w / 2.0;
    let target_y = y + h / 2.0;
    // Center the camera on the computed target center.
    set_xy(target_x, target_y);
}
