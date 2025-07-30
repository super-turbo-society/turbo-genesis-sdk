//! Camera Control Module
//!
//! Provides a comprehensive API for querying and manipulating the in-game camera:
//!
//! - **Position Accessors**:  
//!   - `xyz()`, `xy()`, `x()`, `y()`, `z()` to read the current camera coordinates and zoom.  
//! - **Immediate Setters**:  
//!   - `set_xyz`, `set_xy`, `set_x`, `set_y`, `set_z` for instant camera moves.  
//!   - `move_xyz`, `move_xy`, `move_x`, `move_y`, `move_z` to adjust relative to the current position.  
//!   - `reset`, `reset_x`, `reset_y`, `reset_xy`, `reset_z` to center or reset zoom.  
//!   - Deprecated focus helpers (`focus_rect`, `focus_bounds`, `focus`) for centering on targets.
//!
//! - **Shake Effect**:  
//!   - `shake(amount)`, `is_shaking()`, `shake_amount()`, `remove_shake()` for screen‐space camera shake.  
//!   - Automatically restores original origin and applies randomized offsets.
//!
//! - **Pan/Tween**:  
//!   - `pan_xyz`, `pan_xy`, `pan_x`, `pan_y` to smoothly interpolate the camera over a duration with easing.  
//!   - `update()` to drive ongoing tweens and apply shake each frame.
//!
//! Internals use FFI calls to `turbo_genesis_ffi::canvas` for actual camera updates and
//! a `Tween<(f32, f32, f32)>` under the hood for timed animations.  

use crate::{sys, time, tween::*, Easing};
use borsh::{BorshDeserialize, BorshSerialize};
use num_traits::NumCast;

static mut CAMERA_STATE: CameraState = CameraState::new();

#[derive(Clone, Copy, Debug, Default, BorshDeserialize, BorshSerialize)]
struct CameraState {
    /// Optional camera shake effect
    shake_effect: Option<CameraShakeEffect>,
    /// Optional tween state for smooth camera pans.
    position_tween: Option<Tween<(f32, f32, f32)>>,
}
impl CameraState {
    const fn new() -> Self {
        Self {
            shake_effect: None,
            position_tween: None,
        }
    }
    fn get_mut() -> &'static mut Self {
        unsafe { &mut CAMERA_STATE }
    }
    fn save() -> Result<(), std::io::Error> {
        let cam_state = Self::get_mut();
        let data = borsh::to_vec(cam_state)?;
        sys::internal::set("camera", &data)
    }
    fn load() -> Result<(), std::io::Error> {
        let data = sys::internal::get("camera")?;
        let cam_state = Self::try_from_slice(&data)?;
        unsafe {
            CAMERA_STATE = cam_state;
        }
        Ok(())
    }
}

/// Stores the last tick and the original camera center (before shake)
#[derive(Clone, Copy, Debug, Default, BorshDeserialize, BorshSerialize)]
struct CameraShakeEffect {
    origin: (f32, f32),
    amount: usize,
}

/// Retrieves the current camera position as an (x, y, z) tuple.
/// The values are filled by calling the FFI function `get_camera2`.
pub fn xyz() -> (f32, f32, f32) {
    let mut cam: [f32; 3] = [0.; 3];
    turbo_genesis_ffi::canvas::get_camera(cam.as_mut_ptr());
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

/// Sets the camera's position to (x, y, z).
/// The x and y values are converted to f32; z is clamped to a minimum of 0.0.
pub fn set_xyz<X: NumCast, Y: NumCast>(x: X, y: Y, z: f32) {
    reset_camera_tween();
    let x: f32 = NumCast::from(x).unwrap_or(0.0);
    let y: f32 = NumCast::from(y).unwrap_or(0.0);
    let z = f32::max(z, 0.0);
    turbo_genesis_ffi::canvas::set_camera(x, y, z);
    update_shake_origin();
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

/// Moves the camera's zoom by the specified delta.
pub fn move_z(delta_z: f32) {
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

/// Centers the camera on a target x and y position.
pub fn focus((x, y): (i32, i32)) {
    set_xy(x, y);
}

/// Applies a screen-space shake amount (in pixels) around the last known stable camera position.
pub fn shake<N: NumCast>(amount: N) {
    let amount = NumCast::from(amount).unwrap_or_default();
    let cam_state = CameraState::get_mut();
    cam_state.shake_effect = match cam_state.shake_effect.take() {
        // Revert to shake origin
        Some(shake) if amount == 0 => {
            let (x, y) = shake.origin;
            turbo_genesis_ffi::canvas::set_camera(x, y, z());
            None
        }
        // Update shake amount
        Some(shake) => Some(CameraShakeEffect { amount, ..shake }),
        // There is no shake to unset (no-op)
        None if amount == 0 => None,
        // Create a shake effect and set the origin
        None => Some(CameraShakeEffect {
            origin: xy(),
            amount,
        }),
    }
}

/// Returns the current shake intensity (in pixels).
pub fn shake_amount() -> usize {
    let cam_state = CameraState::get_mut();
    cam_state.shake_effect.map_or(0, |shake| shake.amount)
}

/// Returns `true` if a shake effect is currently active.
pub fn is_shaking() -> bool {
    shake_amount() > 0
}

/// Stops any ongoing camera shake and restores the stable position.
pub fn remove_shake() {
    shake(0)
}

/// Eases the camera toward `target` over `duration` ticks using `easing`.
/// Returns `true` when the transition is complete.
pub fn pan_xyz<X: NumCast, Y: NumCast>(
    target: (X, Y, f32),
    duration: usize,
    easing: Easing,
) -> bool {
    // Get current camera position
    let curr = xyz();
    let cam_state = CameraState::get_mut();
    match cam_state.position_tween.as_mut() {
        None => {
            // Hot reload or initial frame
            if duration == 0 {
                let x: f32 = NumCast::from(target.0).unwrap_or(curr.0);
                let y: f32 = NumCast::from(target.1).unwrap_or(curr.1);
                let z = target.2;
                cam_state.position_tween = Some(Tween::new((x, y, z)).duration(0));
                turbo_genesis_ffi::canvas::set_camera(x, y, z);
                return true;
            }
            cam_state.position_tween = Some(Tween::new(curr).duration(0));
            return false;
        }
        Some(tween) => {
            // Initialize on first tick after manual reset
            if time::tick() == 0 {
                *tween = Tween::new(curr).duration(0);
                return false;
            }
            // Update Tween
            let x: f32 = NumCast::from(target.0).unwrap_or(curr.0);
            let y: f32 = NumCast::from(target.1).unwrap_or(curr.1);
            let z = target.2;
            tween.duration(duration);
            tween.ease(easing);
            tween.set((x, y, z));
            let (x, y, z) = tween.get();
            turbo_genesis_ffi::canvas::set_camera(x, y, z);
            return tween.done();
        }
    }
}

/// Eases the camera in x and y toward `target`, preserving the current zoom.
/// Returns `true` when the transition is complete.
pub fn pan_xy<X: NumCast, Y: NumCast>(target: (X, Y), duration: usize, easing: Easing) -> bool {
    pan_xyz((target.0, target.1, z()), duration, easing)
}

/// Eases the camera’s x coordinate toward `x`, preserving y and zoom.
pub fn pan_x<X: NumCast>(x: X, duration: usize, easing: Easing) -> bool {
    pan_xyz((x, y(), z()), duration, easing)
}

/// Eases the camera’s y coordinate toward `y`, preserving x and zoom.
pub fn pan_y<Y: NumCast>(y: Y, duration: usize, easing: Easing) -> bool {
    pan_xyz((x(), y, z()), duration, easing)
}

/// Eases the camera’s zoom to the `z` zoom level, preserving x and y position.
pub fn pan_z(z: f32, duration: usize, easing: Easing) -> bool {
    let (x, y) = xy();
    pan_xyz((x, y, z), duration, easing)
}

/// Unsets the camera position tween so camera movement is no longer eased
fn reset_camera_tween() {
    let cam_state = CameraState::get_mut();
    cam_state.position_tween = None;
}

/// Updates the shake origin to the current camera position.
fn update_shake_origin() {
    let cam_state = CameraState::get_mut();
    if let Some(shake) = cam_state.shake_effect.as_mut() {
        shake.origin = xy();
    }
}

/// Caches the CameraState before a hot reload
pub(crate) fn on_before_hot_reload() -> Result<(), std::io::Error> {
    CameraState::save()
}

/// Hydrates the CameraState after a hot reload
pub(crate) fn on_after_hot_reload() -> Result<(), std::io::Error> {
    CameraState::load()
}

/// Resets the CameraState when the game is manually reset
pub(crate) fn on_reset() -> Result<(), std::io::Error> {
    unsafe { CAMERA_STATE = CameraState::new() };
    Ok(())
}

/// Internal update loop for active tweens and shakes.
/// Should be called once per frame to drive smooth camera movement.
pub(crate) fn on_update() -> Result<(), std::io::Error> {
    match CameraState::get_mut() {
        CameraState {
            shake_effect: None,
            position_tween: None,
        } => {
            // no-op
        }
        CameraState {
            shake_effect: Some(shake),
            position_tween: None,
        } => {
            let (x, y) = shake.origin;
            let amount = shake.amount as f32;
            let x = x + crate::random::between(-amount, amount);
            let y = y + crate::random::between(-amount, amount);
            turbo_genesis_ffi::canvas::set_camera(x, y, z());
        }
        CameraState {
            shake_effect: None,
            position_tween: Some(position),
        } => {
            let (x, y, z) = position.get();
            turbo_genesis_ffi::canvas::set_camera(x, y, z);
        }
        CameraState {
            shake_effect: Some(shake),
            position_tween: Some(position),
        } => {
            let (mut x, mut y, z) = position.get();
            shake.origin = (x, y);
            let amount = shake.amount as f32;
            x += crate::random::between(-amount, amount);
            y += crate::random::between(-amount, amount);
            turbo_genesis_ffi::canvas::set_camera(x, y, z);
        }
    }
    Ok(())
}
