use crate::bounds::Bounds;
use num_traits::NumCast;

/// Creates and returns a new sprite from the given file stem.
/// Internally, this calls `sprite::Sprite::new(name)`.
pub fn sprite(file_stem: &str) -> sprite::Sprite {
    sprite::Sprite::new(file_stem)
}

/// Creates and returns a new nine-slice sprite from the given file stem and margins.
/// Internally, this calls `sprite::Sprite::new(file_stem)` and wraps it in a `NineSliceSprite`.
pub fn nine_slice(file_stem: &str, margins: (u32, u32, u32, u32)) -> sprite::NineSliceSprite {
    sprite::NineSliceSprite::new(sprite::Sprite::new(file_stem), margins)
}

/// Creates a new rectangle with the specified width and height.
/// The width and height are converted using `NumCast`.
/// If conversion fails, zero is used as a default.
pub fn rect<W: NumCast, H: NumCast>(w: W, h: H) -> rect::Rectangle {
    rect::Rectangle::new().size(NumCast::from(w).unwrap_or(0), NumCast::from(h).unwrap_or(0))
}

/// Creates a new ellipse with the specified width and height.
/// The width and height are converted using `NumCast`.
/// If conversion fails, zero is used as a default.
pub fn ellipse<W: NumCast, H: NumCast>(w: W, h: H) -> ellipse::Ellipse {
    ellipse::Ellipse::new().size(NumCast::from(w).unwrap_or(0), NumCast::from(h).unwrap_or(0))
}

/// Creates a new circle with the specified diameter.
/// The diameter is converted using `NumCast`.
/// If conversion fails, zero is used as a default.
pub fn circ<D: NumCast + Copy>(d: D) -> circ::Circle {
    circ::Circle::new().size(NumCast::from(d).unwrap_or(0))
}

/// Creates a new line from the given start and end coordinates.
/// The coordinates are converted using `NumCast`.
/// Note: The generic type parameters represent the coordinate types.
/// (There is a potential typo in the type parameters for start_y and end_x.)
pub fn path<X0: NumCast + Copy, Y0: NumCast + Copy, X1: NumCast + Copy, Y1: NumCast + Copy>(
    start_x: X0,
    start_y: Y0,
    end_x: X1,
    end_y: Y1,
) -> path::Path {
    path::Path::new()
        .start_position(start_x, start_y)
        .end_position(end_x, end_y)
}

/// Creates a new text object from the given string.
/// Internally, this calls `text::Text::new(string)`.
pub fn text(string: &str) -> text::Text {
    text::Text::new(string)
}

/// Retrieves (or creates if not present) a sprite animation associated with the given key.
/// This ensures that an animation exists in the global animation map.
pub fn animation(key: &str) -> &mut animation::SpriteAnimation {
    animation::SpriteAnimation::get_or_insert(key)
}

/// Returns the current viewport bounds.
/// This is typically used to get the canvas or screen boundaries.
pub fn viewport() -> Bounds {
    let (w, h) = crate::canvas::resolution();
    let (x, y) = crate::canvas::camera::xy();
    let x = (x as f32 - (w as f32 / 2.)) as i32;
    let y = (y as f32 - (h as f32 / 2.)) as i32;
    Bounds::new(x, y, w, h)
}

/// Returns the current resolution as a tuple (width, height).
/// The resolution is fetched from the system as a single integer:
/// - The lower 16 bits represent the width.
/// - The upper bits (shifted right 16) represent the height.
pub fn resolution() -> (u32, u32) {
    let res = crate::ffi::sys::resolution();
    let w = res & 0xffff; // Extract the lower 16 bits for width.
    let h = res >> 16; // Extract the upper bits for height.
    (w, h)
}

/// Clears the canvas using the specified color.
/// The `color` is a packed big-endian RGBA value (e.g., `0x000000ff` is black).
pub fn clear(color: u32) {
    crate::ffi::canvas::clear(color)
}

//------------------------------------------------------------------------------
// CAMERA
//------------------------------------------------------------------------------
pub mod camera {
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
}

//------------------------------------------------------------------------------
// ANIMATION
//------------------------------------------------------------------------------
pub mod animation {
    use super::*;
    use sprite::Sprite;
    use std::collections::BTreeMap;
    use utils::sprite::{SpriteAnimationDirection, SpriteAnimationFrame};

    static mut TURBO_ANIMATIONS: BTreeMap<u64, SpriteAnimation> = BTreeMap::new();
    static mut TURBO_ANIMATIONS_LAST_GC_AT: usize = 0;

    #[derive(Debug, Clone, Copy, Default)]
    pub struct SpriteAnimationProps {
        /// Unique identifier for the sprite (typically a hash of its name).
        sprite_id: u64,
        /// Nonce for sprite data; used to detect changes in the sprite source.
        sprite_data_nonce: u64,
        /// Current frame index of the animation.
        frame: usize,
        /// Tick (or timestamp) when the animation started.
        started_at: usize,
        /// Number of complete animation cycles that have been executed.
        iterations: usize,
        /// Number of times the animation should repeat before stopping.
        repeat: usize,
        /// If true, the animation holds on the last frame when completed.
        fill_forwards: bool,
        /// Direction in which the animation is played.
        direction: SpriteAnimationDirection,
        /// Playback speed factor; higher values speed up the animation.
        speed: f32,
        /// Delay (in milliseconds) before the animation starts (applied only on the first iteration).
        delay: f32,
        /// Timestamp when the animation was paused; `None` if the animation is running.
        paused_at: Option<usize>,
        /// Duration (in milliseconds) of the current pause period.
        curr_pause_dur: f32,
        /// Accumulated duration (in milliseconds) from previous pause periods.
        prev_pause_dur: f32,
        /// Indicates whether the animation has finished all its iterations.
        done: bool,
    }
    impl SpriteAnimationProps {
        /// Creates a new SpriteAnimationProps with default settings.
        /// - `sprite_id`: a unique identifier for the sprite.
        /// - `sprite_data_nonce`: a nonce used to track changes in sprite source data.
        /// - Initializes frame, timing, pause, and control fields.
        pub fn new(sprite_id: u64) -> Self {
            Self {
                sprite_id,
                sprite_data_nonce: utils::sprite::get_source_data_nonce(),
                frame: 0,
                started_at: crate::sys::tick(),
                iterations: 0,
                repeat: 0,
                direction: SpriteAnimationDirection::Forward,
                fill_forwards: false,
                speed: 1.0,
                delay: 0.0,
                paused_at: None,
                prev_pause_dur: 0.0,
                curr_pause_dur: 0.0,
                done: false,
            }
        }

        /// Initializes SpriteAnimationProps based on sprite source data.
        /// Returns `None` if no sprite source data is found for the given name.
        pub fn from_sprite_name(name: &str) -> Option<Self> {
            // Attempt to retrieve sprite source data using the provided name.
            let Some(sprite_data) = utils::sprite::get_source_data(name) else {
                return None;
            };

            // Generate a sprite_id by hashing the name.
            let sprite_id = utils::hash::fnv1a(name.as_bytes());
            // Create default properties.
            let mut props = Self::new(sprite_id);

            // Set the repeat field:
            // - If animation_loop_count is 0, treat as infinite repeats.
            // - Otherwise, repeat is loop count minus one (e.g., loop count 1 means no repeats).
            props.repeat = if sprite_data.animation_loop_count == 0 {
                usize::MAX
            } else {
                sprite_data.animation_loop_count as usize - 1
            };

            // Set the animation direction based on the source data.
            props.direction = sprite_data.animation_direction;

            // Return the configured properties.
            Some(props)
        }

        /// Returns the tick at which the animation started.
        pub fn started_at(&self) -> usize {
            self.started_at
        }

        /// Returns the number of full animation iterations completed.
        pub fn iterations(&self) -> usize {
            self.iterations
        }

        /// Returns the number of times the animation will repeat.
        pub fn repeat(&self) -> usize {
            self.repeat
        }

        /// Returns the current frame.
        pub fn frame(&self) -> usize {
            self.frame
        }

        /// Sets the repeat count for the animation.
        pub fn set_repeat(&mut self, repeat: usize) {
            self.repeat = repeat;
        }

        /// Returns the current animation direction (e.g., forward or reverse).
        pub fn direction(&self) -> SpriteAnimationDirection {
            self.direction
        }

        /// Sets the animation direction.
        pub fn set_direction(&mut self, direction: SpriteAnimationDirection) {
            self.direction = direction;
        }

        /// Returns `true` if the animation will hold on the last frame when finished.
        /// If `false`, the animation will loop back to the first frame.
        pub fn fill_forwards(&self) -> bool {
            self.fill_forwards
        }

        /// Sets whether the animation should fill forwards when complete.
        pub fn set_fill_forwards(&mut self, fill_forwards: bool) {
            self.fill_forwards = fill_forwards;
        }

        /// Returns the current playback speed.
        /// - 1.0: normal speed; less than 1.0: slower; greater than 1.0: faster.
        pub fn speed(&self) -> f32 {
            self.speed
        }

        /// Sets the playback speed.
        pub fn set_speed(&mut self, speed: f32) {
            self.speed = speed;
        }

        /// Returns the delay (in milliseconds) before the animation starts on the first iteration.
        pub fn delay(&self) -> f32 {
            self.delay
        }

        /// Sets the initial delay (in milliseconds) before the animation starts.
        pub fn set_delay(&mut self, delay: f32) {
            self.delay = delay;
        }

        /// Returns `true` if the animation is currently paused.
        pub fn paused(&self) -> bool {
            self.paused_at.is_some()
        }

        /// Sets the paused state.
        /// - If `paused` is true, pauses the animation.
        /// - If false, resumes the animation.
        pub fn set_paused(&mut self, paused: bool) {
            if paused {
                self.pause();
            } else {
                self.resume();
            }
        }

        /// Pauses the animation by recording the current tick if not already paused.
        pub fn pause(&mut self) {
            if self.paused_at.is_none() {
                self.paused_at = Some(crate::sys::tick());
            }
        }

        /// Resumes the animation by accumulating the pause duration and clearing the paused state.
        pub fn resume(&mut self) {
            self.prev_pause_dur += self.curr_pause_dur;
            self.curr_pause_dur = 0.0;
            self.paused_at = None;
        }

        /// Returns `true` if the animation is finished.
        pub fn done(&self) -> bool {
            self.done
        }

        /// Restarts the animation, resetting time, frame, pause durations, and iterations.
        pub fn restart(&mut self) {
            self.started_at = crate::sys::tick();
            self.frame = 0;
            self.iterations = 0;
            self.paused_at = None;
            self.prev_pause_dur = 0.0;
            self.curr_pause_dur = 0.0;
            self.done = false;
        }

        /// Stops the animation immediately by restarting it and then pausing it.
        pub fn stop(&mut self) {
            self.restart();
            self.pause();
        }

        /// Updates the animation based on the elapsed time and provided frames.
        /// - Handles delay on the first iteration.
        /// - Adjusts for pause durations.
        /// - Increments frame and iteration counts.
        /// - Marks the animation as done if repeat count is reached.
        /// - Supports a variety of animation directions.
        fn update(&mut self, frames: &[SpriteAnimationFrame]) {
            // If the animation is marked as done or there are no frames, exit early.
            if self.done {
                return;
            }
            let num_frames = frames.len();
            if num_frames == 0 {
                return;
            }
            let now = crate::sys::tick();

            // If the animation is paused, update the current pause duration and do not advance.
            if let Some(paused_at) = self.paused_at {
                self.curr_pause_dur = (now.saturating_sub(paused_at) as f32 / 60.0) * 1000.0;
                return;
            }

            // Calculate elapsed time in ms since the animation started, excluding past pauses.
            let elapsed_ms =
                (now.saturating_sub(self.started_at) as f32 / 60.0) * 1000.0 - self.prev_pause_dur;
            let mut effective_elapsed = elapsed_ms;

            // For the first iteration, honor the delay before starting the animation.
            if self.iterations == 0 {
                if effective_elapsed < self.delay {
                    return;
                } else {
                    effective_elapsed -= self.delay;
                }
            }

            match self.direction {
                // ------------------ FORWARD ------------------
                SpriteAnimationDirection::Forward => {
                    let mut accumulated_time = 0.0;
                    for (i, frame) in frames.iter().enumerate() {
                        accumulated_time += frame.duration / self.speed;
                        if effective_elapsed
                            % (frames.iter().map(|f| f.duration).sum::<f32>() / self.speed)
                            < accumulated_time
                        {
                            // If the new index is less than the previous frame, a wrap-around occurred.
                            if i < self.frame {
                                self.iterations += 1;
                            }
                            self.frame = i;
                            break;
                        }
                    }
                }
                // ------------------ REVERSE ------------------
                SpriteAnimationDirection::Reverse => {
                    let mut accumulated_time = 0.0;
                    for i in (0..num_frames).rev() {
                        accumulated_time += frames[i].duration / self.speed;
                        if effective_elapsed
                            % (frames.iter().map(|f| f.duration).sum::<f32>() / self.speed)
                            < accumulated_time
                        {
                            // For reverse, a wrap-around is detected when the new index is higher than the previous.
                            if i > self.frame {
                                self.iterations += 1;
                            }
                            self.frame = i;
                            break;
                        }
                    }
                }
                // -------------- PINGPONG & PINGPONG REVERSE --------------
                SpriteAnimationDirection::PingPong | SpriteAnimationDirection::PingPongReverse => {
                    // Calculate the forward duration (the sum of all frame durations, adjusted by speed).
                    let base_duration: f32 =
                        frames.iter().map(|f| f.duration).sum::<f32>() / self.speed;
                    // In ping-pong, a full cycle is going forward and then backward.
                    let cycle_duration = 2.0 * base_duration;
                    // Count full cycles completed.
                    self.iterations = (effective_elapsed / cycle_duration) as usize;
                    // Determine the time within the current cycle.
                    let time_in_cycle = effective_elapsed % cycle_duration;

                    // Compute a forward frame index based on a time value t in the range [0, base_duration].
                    // For both pingpong variants, we compute a "forward" index first.
                    let t = if time_in_cycle < base_duration {
                        // First half of the cycle.
                        time_in_cycle
                    } else {
                        // Second half: t counts from 0 to base_duration again.
                        time_in_cycle - base_duration
                    };

                    let mut accumulated_time = 0.0;
                    let mut forward_index = 0;
                    for (i, frame) in frames.iter().enumerate() {
                        accumulated_time += frame.duration / self.speed;
                        if t < accumulated_time {
                            forward_index = i;
                            break;
                        }
                    }

                    // For PingPong: first half plays forward, second half plays reverse.
                    // For PingPongReverse: first half plays reverse, second half plays forward.
                    self.frame = match self.direction {
                        SpriteAnimationDirection::PingPong => {
                            if time_in_cycle < base_duration {
                                forward_index
                            } else {
                                // Reverse order: invert the forward index.
                                num_frames.saturating_sub(1) - forward_index
                            }
                        }
                        SpriteAnimationDirection::PingPongReverse => {
                            if time_in_cycle < base_duration {
                                // Reverse order first.
                                num_frames.saturating_sub(1) - forward_index
                            } else {
                                forward_index
                            }
                        }
                        _ => unreachable!(),
                    };
                }
            }

            // If a repeat limit is set and the number of iterations reaches or exceeds it, mark the animation as done.
            if self.iterations > 0 && self.iterations >= self.repeat {
                self.done = true;
                // Determine the final frame based on both direction and the fill_forwards flag.
                self.frame = match (self.direction, self.fill_forwards) {
                    // For forward and PingPong modes: if fill_forwards is true, keep the last frame; otherwise, reset to first.
                    (SpriteAnimationDirection::Forward, true)
                    | (SpriteAnimationDirection::PingPong, true) => num_frames.saturating_sub(1),
                    (SpriteAnimationDirection::Forward, false)
                    | (SpriteAnimationDirection::PingPong, false) => 0,
                    // For reverse and PingPongReverse: if fill_forwards is true, keep the first frame; otherwise, reset to last.
                    (SpriteAnimationDirection::Reverse, true)
                    | (SpriteAnimationDirection::PingPongReverse, true) => 0,
                    (SpriteAnimationDirection::Reverse, false)
                    | (SpriteAnimationDirection::PingPongReverse, false) => {
                        num_frames.saturating_sub(1)
                    }
                };
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct SpriteAnimation {
        /// The last tick when the animation was updated. Used for garbage collection.
        updated_at: usize,
        /// Time-to-live in frames: how long an animation can remain unused before being GC'ed.
        ttl: usize,
        /// Optional animation properties; if None, the animation hasn't been initialized yet.
        props: Option<SpriteAnimationProps>,
    }

    impl SpriteAnimation {
        /// Retrieves an existing animation by key or inserts a new one if it doesn't exist.
        /// Also performs garbage collection on stale animations.
        pub fn get_or_insert(key: &str) -> &mut Self {
            #[allow(static_mut_refs)]
            unsafe {
                let t = crate::sys::tick(); // Get the current tick count.
                let hash = utils::hash::fnv1a(key.as_bytes()); // Hash the sprite name.

                // Retrieve the animation from the global map or create a new one.
                let anim = TURBO_ANIMATIONS.entry(hash).or_insert_with(|| Self {
                    updated_at: t,
                    ttl: 1,
                    props: None,
                });

                // Run garbage collection if at least one frame has elapsed since the last GC.
                if t.abs_diff(TURBO_ANIMATIONS_LAST_GC_AT) >= 1 {
                    let sprite_data_nonce = utils::sprite::get_source_data_nonce();
                    // Retain animations that are still active and update properties if necessary.
                    TURBO_ANIMATIONS.retain(|_key, anim| {
                        // Remove the entry if it hasn't been updated within its TTL.
                        if t.abs_diff(anim.updated_at) > anim.ttl {
                            return false;
                        }
                        // Update animation properties if the sprite data has changed.
                        anim.props = anim.props.and_then(|props| {
                            if props.sprite_data_nonce == sprite_data_nonce {
                                // Sprite data unchanged; retain existing properties.
                                return Some(props);
                            }
                            // If sprite data changed, attempt to refresh properties.
                            utils::hash::lookup_fnv1a(props.sprite_id)
                                .and_then(|bytes| std::str::from_utf8(bytes).ok())
                                .and_then(SpriteAnimationProps::from_sprite_name)
                                .map(|next| {
                                    // Update properties if repeat or direction have changed.
                                    if props.repeat != next.repeat {
                                        return next;
                                    }
                                    if props.direction != next.direction {
                                        return next;
                                    }
                                    // Otherwise, retain the current properties.
                                    props
                                })
                        });
                        // Update the last updated tick for this animation.
                        anim.updated_at = t;
                        true // Keep this animation.
                    });
                    // Record the tick of the last garbage collection.
                    TURBO_ANIMATIONS_LAST_GC_AT = t;
                }

                // If the animation properties exist, update the animation state.
                if let Some(props) = &mut anim.props {
                    // If a hot-reload occurred (e.g., system tick reset), restart the animation.
                    if t == 0 && props.started_at > 0 {
                        props.restart();
                    }
                    // Look up the sprite source data and update the animation frame.
                    if let Some(sprite_data) = utils::hash::lookup_fnv1a(props.sprite_id)
                        .and_then(|bytes| std::str::from_utf8(bytes).ok())
                        .and_then(utils::sprite::get_source_data)
                    {
                        props.update(&sprite_data.animation_frames);
                    }
                }
                anim
            }
        }

        /// Sets the time-to-live for the animation.
        pub fn set_ttl(&mut self, ttl: usize) {
            self.ttl = ttl;
        }

        /// Returns the number of completed iterations (cycles) of the animation.
        pub fn iterations(&self) -> usize {
            self.props
                .as_ref()
                .map_or(0, SpriteAnimationProps::iterations)
        }

        /// Returns the repeat count configured for the animation.
        pub fn repeat(&self) -> usize {
            self.props.as_ref().map_or(0, SpriteAnimationProps::repeat)
        }

        /// Sets the repeat count for the animation.
        pub fn set_repeat(&mut self, repeat: usize) {
            if let Some(props) = &mut self.props {
                props.set_repeat(repeat);
            }
        }

        /// Returns the current animation direction.
        pub fn direction(&self) -> SpriteAnimationDirection {
            self.props.as_ref().map_or(
                SpriteAnimationDirection::Forward,
                SpriteAnimationProps::direction,
            )
        }

        /// Returns true if the animation has completed its target loop count or has not yet started.
        pub fn done(&self) -> bool {
            self.props.as_ref().map_or(true, SpriteAnimationProps::done)
        }

        /// Sets the animation's direction.
        pub fn set_direction(&mut self, direction: SpriteAnimationDirection) {
            if let Some(props) = &mut self.props {
                props.set_direction(direction);
            }
        }

        /// Sets the animation's direction to forward.
        pub fn forward(&mut self) {
            if let Some(props) = &mut self.props {
                props.set_direction(SpriteAnimationDirection::Forward);
            }
        }

        /// Sets the animation's direction to reverse.
        pub fn reverse(&mut self) {
            if let Some(props) = &mut self.props {
                props.set_direction(SpriteAnimationDirection::Reverse);
            }
        }

        /// Sets the animation's direction to pingpong.
        pub fn ping_pong(&mut self) {
            if let Some(props) = &mut self.props {
                props.set_direction(SpriteAnimationDirection::PingPong);
            }
        }

        /// Sets the animation's direction to pingpong reverse.
        pub fn ping_pong_reverse(&mut self) {
            if let Some(props) = &mut self.props {
                props.set_direction(SpriteAnimationDirection::PingPongReverse);
            }
        }

        /// Indicates if the animation should "fill forwards" upon completion.
        pub fn fill_forwards(&self) -> bool {
            self.props
                .as_ref()
                .map_or(false, SpriteAnimationProps::fill_forwards)
        }

        /// Sets whether the animation should fill forwards when complete.
        pub fn set_fill_forwards(&mut self, fill_forwards: bool) {
            if let Some(props) = &mut self.props {
                props.set_fill_forwards(fill_forwards);
            }
        }

        /// Returns the current speed (rate) of the animation.
        pub fn speed(&self) -> f32 {
            self.props.as_ref().map_or(1.0, SpriteAnimationProps::speed)
        }

        /// Sets the animation speed (rate).
        pub fn set_speed(&mut self, speed: f32) {
            if let Some(props) = &mut self.props {
                props.set_speed(speed);
            }
        }

        /// Returns the current delay (in milliseconds) before starting the animation.
        pub fn delay(&self) -> f32 {
            self.props.as_ref().map_or(0.0, SpriteAnimationProps::delay)
        }

        /// Sets the delay (in milliseconds) before the animation starts.
        pub fn set_delay(&mut self, delay: f32) {
            if let Some(props) = &mut self.props {
                props.set_delay(delay);
            }
        }

        /// Returns whether the animation is currently paused.
        pub fn paused(&self) -> bool {
            self.props
                .as_ref()
                .map_or(false, SpriteAnimationProps::paused)
        }

        /// Sets the paused state of the animation.
        pub fn set_paused(&mut self, paused: bool) {
            if let Some(props) = &mut self.props {
                props.set_paused(paused);
            }
        }

        /// Pauses the animation.
        pub fn pause(&mut self) {
            if let Some(props) = &mut self.props {
                props.pause();
            }
        }

        /// Resumes the animation if it is paused.
        pub fn resume(&mut self) {
            if let Some(props) = &mut self.props {
                props.resume();
            }
        }

        /// Immediately stops the animation.
        pub fn stop(&mut self) {
            if let Some(props) = &mut self.props {
                props.stop();
            }
        }

        /// Restarts the animation from the beginning.
        pub fn restart(&mut self) {
            if let Some(props) = &mut self.props {
                props.restart();
            }
        }

        /// Gets the current frame of the animation.
        pub fn frame(&self) -> Option<usize> {
            self.props.map(|p| p.frame)
        }

        /// Updates the animation with data based on the given sprite key.
        pub fn set_sprite_name(&mut self, name: &str) {
            let sprite_id = utils::hash::fnv1a(name.as_bytes());
            // Insert new properties if they are missing.
            let props = self.props.get_or_insert_with(|| {
                SpriteAnimationProps::from_sprite_name(name)
                    .unwrap_or_else(|| SpriteAnimationProps::new(sprite_id))
            });
            // If the sprite_id stored in properties doesn't match the new hash, update properties.
            if props.sprite_id != sprite_id {
                if let Some(next_props) = SpriteAnimationProps::from_sprite_name(name) {
                    *props = next_props;
                }
            }
        }

        /// Gets the current sprite name of the animation. Returns and empty str if no sprite is set.
        pub fn sprite_name<'a>(&self) -> &'a str {
            return self
                .props
                .and_then(|props| utils::hash::lookup_fnv1a(props.sprite_id))
                .map(std::str::from_utf8)
                .and_then(Result::ok)
                .unwrap_or_default();
        }

        /// Retrieves a sprite instance with its current sprite name.
        /// If the sprite's properties don't match the name, they are updated.
        pub fn sprite<'a>(&self) -> Sprite<'a> {
            // Update the animation with sprite data.
            let name = self.sprite_name();
            // Return a sprite with the current frame from the animation properties.
            return sprite(name).frame(self.props.as_ref().map_or(0, SpriteAnimationProps::frame));
        }
    }
}

//------------------------------------------------------------------------------
// SPRITE
//------------------------------------------------------------------------------
pub mod sprite {
    use super::*;
    use crate::bounds::*;
    use num_traits::NumCast;

    pub mod flags {
        // Repeats the sprite within the containing quad
        pub const SPRITE_REPEAT: u32 = 1 << 0;
        // Scales a sprite to fit the dimensions of the containing quad
        pub const SPRITE_COVER: u32 = 2 << 0;
    }

    #[derive(Debug, Clone, Copy)]
    pub struct SpriteProps {
        /// X coordinate for the sprite's position.
        x: i32,
        /// Y coordinate for the sprite's position.
        y: i32,
        /// Width of the sprite.
        w: u32,
        /// Height of the sprite.
        h: u32,
        /// X offset of the sprite texture.
        texture_x: i32,
        /// Y offset of the sprite texture.
        texture_y: i32,
        /// Primary color overlay (typically a packed ARGB/RGBA value).
        color: u32,
        /// Background color of the sprite.
        background_color: u32,
        /// Border radius for rounded corners.
        border_radius: u32,
        /// X coordinate of the origin (pivot) used for transformations.
        origin_x: i32,
        /// Y coordinate of the origin (pivot) used for transformations.
        origin_y: i32,
        /// Rotation angle of the sprite (e.g., in degrees).
        rotation: i32,
        /// Horizontal scale factor.
        scale_x: f32,
        /// Vertical scale factor.
        scale_y: f32,
        /// Flip the sprite horizontally.
        flip_x: bool,
        /// Flip the sprite vertically.
        flip_y: bool,
        /// Whether the sprite texture should be repeated.
        repeat: bool,
        /// Indicates if the sprite's position is absolute.
        absolute: bool,
        /// Opacity level (0.0 = fully transparent, 1.0 = fully opaque).
        opacity: f32,
        /// Speed factor for sprite animations.
        animation_speed: f32,
        /// Current animation frame index, if applicable.
        frame: Option<usize>,
    }
    impl Default for SpriteProps {
        fn default() -> Self {
            Self {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
                texture_x: 0,
                texture_y: 0,
                color: 0xffffffff,            // Default color overlay is white
                background_color: 0x00000000, // Default background is transparent
                border_radius: 0,
                origin_x: 0,
                origin_y: 0,
                rotation: 0,
                scale_x: 1.0, // Default scale is 1.0
                scale_y: 1.0, // Default scale is 1.0
                flip_x: false,
                flip_y: false,
                repeat: false,
                absolute: false,
                opacity: 1.0,
                animation_speed: 1.0, // Default animation speed is 1.0
                frame: None,
            }
        }
    }
    impl SpriteProps {
        /// Creates new sprite properties with default values.
        pub fn new() -> Self {
            Self::default()
        }

        /// Sets the position of the sprite.
        pub fn position(mut self, x: i32, y: i32) -> Self {
            self.x = x;
            self.y = y;
            self
        }

        /// Sets the size of the sprite.
        pub fn size(mut self, w: u32, h: u32) -> Self {
            self.w = w;
            self.h = h;
            self
        }

        /// Translates the sprite’s position by the given delta.
        pub fn offset(mut self, dx: i32, dy: i32) -> Self {
            self.x += dx;
            self.y += dy;
            self
        }

        /// Sets the inner texture position for the sprite.
        pub fn tex_position(mut self, texture_x: i32, texture_y: i32) -> Self {
            self.texture_x = texture_x;
            self.texture_y = texture_y;
            self
        }

        /// Sets the primary color of the sprite.
        pub fn color(mut self, color: u32) -> Self {
            self.color = color;
            self
        }

        /// Sets the background color of the sprite.
        pub fn background_color(mut self, background_color: u32) -> Self {
            self.background_color = background_color;
            self
        }

        /// Sets the border radius for the sprite.
        pub fn border_radius(mut self, radius: u32) -> Self {
            self.border_radius = radius;
            self
        }

        /// Sets the origin point for transformations.
        pub fn origin(mut self, origin_x: i32, origin_y: i32) -> Self {
            self.origin_x = origin_x;
            self.origin_y = origin_y;
            self
        }

        /// Sets the rotation (in degrees) for the sprite.
        pub fn rotation(mut self, angle: i32) -> Self {
            self.rotation = angle;
            self
        }

        /// Sets the scale factors for the sprite.
        pub fn scale(mut self, scale_x: f32, scale_y: f32) -> Self {
            self.scale_x = scale_x;
            self.scale_y = scale_y;
            self
        }

        /// Sets the flip flags for the sprite.
        pub fn flip(mut self, flip_x: bool, flip_y: bool) -> Self {
            self.flip_x = flip_x;
            self.flip_y = flip_y;
            self
        }

        /// Enables or disables texture repeating.
        pub fn repeat(mut self, repeat: bool) -> Self {
            self.repeat = repeat;
            self
        }

        /// Sets the sprite to use absolute positioning.
        pub fn absolute(mut self, absolute: bool) -> Self {
            self.absolute = absolute;
            self
        }

        /// Sets the opacity of the sprite (0.0 to 1.0).
        pub fn opacity(mut self, opacity: f32) -> Self {
            self.opacity = opacity;
            self
        }

        /// Sets the playback speed of the sprite animation.
        pub fn animation_speed(mut self, speed: f32) -> Self {
            self.animation_speed = speed;
            self
        }

        /// Sets a fixed frame for the sprite animation.
        pub fn frame(mut self, frame: usize) -> Self {
            self.frame = Some(frame);
            self
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct Sprite<'a> {
        name: &'a str,
        props: SpriteProps,
    }
    impl<'a> Sprite<'a> {
        /// Creates a new sprite with the given name and default properties.
        pub fn new(name: &'a str) -> Self {
            Self {
                name,
                props: SpriteProps::default(),
            }
        }

        /// Sets the sprite’s position.
        pub fn position<X: NumCast, Y: NumCast>(mut self, x: X, y: Y) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.props.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.props.y);
            self.props = self.props.position(x, y);
            self
        }

        /// Sets the sprite’s position.
        pub fn set_position<X: NumCast, Y: NumCast>(&mut self, x: X, y: Y) {
            let x: i32 = NumCast::from(x).unwrap_or(self.props.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.props.y);
            self.props = self.props.position(x, y);
        }

        /// Sets the sprite’s x position.
        pub fn position_x<X: NumCast>(mut self, x: X) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.props.x);
            self.props = self.props.position(x, self.props.y);
            self
        }

        /// Sets the sprite’s x position.
        pub fn set_position_x<X: NumCast>(&mut self, x: X) {
            let x: i32 = NumCast::from(x).unwrap_or(self.props.x);
            self.props = self.props.position(x, self.props.y);
        }

        /// Sets the sprite’s y position.
        pub fn position_y<Y: NumCast>(mut self, y: Y) -> Self {
            let y: i32 = NumCast::from(y).unwrap_or(self.props.y);
            self.props = self.props.position(self.props.x, y);
            self
        }

        /// Sets the sprite’s y position.
        pub fn set_position_y<Y: NumCast>(&mut self, y: Y) {
            let y: i32 = NumCast::from(y).unwrap_or(self.props.y);
            self.props = self.props.position(self.props.x, y);
        }

        /// Sets the sprite’s x and y position.
        pub fn position_xy<X: NumCast, Y: NumCast>(mut self, (x, y): (X, Y)) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.props.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.props.y);
            self.props = self.props.position(x, y);
            self
        }

        /// Sets the sprite’s x and y position.
        pub fn set_position_xy<X: NumCast, Y: NumCast>(&mut self, (x, y): (X, Y)) {
            let x: i32 = NumCast::from(x).unwrap_or(self.props.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.props.y);
            self.props = self.props.position(x, y);
        }

        /// Sets the sprite’s size.
        pub fn size<W: NumCast, H: NumCast>(mut self, w: W, h: H) -> Self {
            let w: u32 = NumCast::from(w).unwrap_or(self.props.w);
            let h: u32 = NumCast::from(h).unwrap_or(self.props.h);
            self.props = self.props.size(w, h);
            self
        }

        /// Sets the sprite’s size.
        pub fn set_size<W: NumCast, H: NumCast>(&mut self, w: W, h: H) {
            let w: u32 = NumCast::from(w).unwrap_or(self.props.w);
            let h: u32 = NumCast::from(h).unwrap_or(self.props.h);
            self.props = self.props.size(w, h);
        }

        /// Sets the sprite’s width.
        pub fn set_size_w<W: NumCast>(&mut self, w: W) {
            let w: u32 = NumCast::from(w).unwrap_or(self.props.w);
            self.props = self.props.size(w, self.props.h);
        }

        /// Sets the sprite’s height.
        pub fn size_h<H: NumCast>(mut self, h: H) -> Self {
            let h: u32 = NumCast::from(h).unwrap_or(self.props.h);
            self.props = self.props.size(self.props.w, h);
            self
        }

        /// Sets the sprite’s height.
        pub fn set_size_h<H: NumCast>(&mut self, h: H) {
            let h: u32 = NumCast::from(h).unwrap_or(self.props.h);
            self.props = self.props.size(self.props.w, h);
        }

        /// Sets the sprite’s width and height.
        pub fn size_wh<W: NumCast, H: NumCast>(mut self, (w, h): (W, H)) -> Self {
            let w: u32 = NumCast::from(w).unwrap_or(self.props.w);
            let h: u32 = NumCast::from(h).unwrap_or(self.props.h);
            self.props = self.props.size(w, h);
            self
        }

        /// Sets the sprite’s width and height.
        pub fn set_size_wh<W: NumCast, H: NumCast>(&mut self, (w, h): (W, H)) {
            let w: u32 = NumCast::from(w).unwrap_or(self.props.w);
            let h: u32 = NumCast::from(h).unwrap_or(self.props.h);
            self.props = self.props.size(w, h);
        }

        /// Sets the sprite’s width.
        pub fn width<W: NumCast>(mut self, w: W) -> Self {
            let w: u32 = NumCast::from(w).unwrap_or(self.props.w);
            self.props = self.props.size(w, self.props.h);
            self
        }

        /// Sets the sprite’s width.
        pub fn set_width<W: NumCast>(&mut self, w: W) {
            let w: u32 = NumCast::from(w).unwrap_or(self.props.w);
            self.props = self.props.size(w, self.props.h);
        }

        /// Sets the sprite’s height.
        pub fn height<H: NumCast>(mut self, h: H) -> Self {
            let h: u32 = NumCast::from(h).unwrap_or(self.props.h);
            self.props = self.props.size(self.props.w, h);
            self
        }

        /// Sets the sprite’s height.
        pub fn set_height<H: NumCast>(&mut self, h: H) {
            let h: u32 = NumCast::from(h).unwrap_or(self.props.h);
            self.props = self.props.size(self.props.w, h);
        }

        /// Translates the sprite’s position by the given delta.
        pub fn offset<DX: NumCast, DY: NumCast>(mut self, dx: DX, dy: DY) -> Self {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.props = self.props.offset(dx, dy);
            self
        }

        /// Translates the sprite’s position by the given delta.
        pub fn set_offset<DX: NumCast, DY: NumCast>(&mut self, dx: DX, dy: DY) {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.props = self.props.offset(dx, dy);
        }

        /// Translates the sprite’s x position by the given delta.
        pub fn offset_x<DX: NumCast>(mut self, dx: DX) -> Self {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            self.props = self.props.offset(dx, 0);
            self
        }

        /// Translates the sprite’s x position by the given delta.
        pub fn set_offset_x<DX: NumCast>(&mut self, dx: DX) {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            self.props = self.props.offset(dx, 0);
        }

        /// Translates the sprite’s position by the given delta.
        pub fn offset_y<DY: NumCast>(mut self, dy: DY) -> Self {
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.props = self.props.offset(0, dy);
            self
        }

        /// Translates the sprite’s position by the given delta.
        pub fn set_offset_y<DY: NumCast>(&mut self, dy: DY) {
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.props = self.props.offset(0, dy);
        }

        /// Translates the sprite’s position by the given delta.
        pub fn offset_xy<DX: NumCast, DY: NumCast>(mut self, (dx, dy): (DX, DY)) -> Self {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.props = self.props.offset(dx, dy);
            self
        }

        /// Translates the sprite’s position by the given delta.
        pub fn set_offset_xy<DX: NumCast, DY: NumCast>(&mut self, (dx, dy): (DX, DY)) {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.props = self.props.offset(dx, dy);
        }

        /// Sets the inner texture position for the sprite.
        pub fn tex_position<TX: NumCast, TY: NumCast>(
            mut self,
            texture_x: TX,
            texture_y: TY,
        ) -> Self {
            let texture_x: i32 = NumCast::from(texture_x).unwrap_or(self.props.texture_x);
            let texture_y: i32 = NumCast::from(texture_y).unwrap_or(self.props.texture_y);
            self.props = self.props.tex_position(texture_x, texture_y);
            self
        }

        /// Sets the inner texture position for the sprite.
        pub fn set_tex_position<TX: NumCast, TY: NumCast>(&mut self, texture_x: TX, texture_y: TY) {
            let texture_x: i32 = NumCast::from(texture_x).unwrap_or(self.props.texture_x);
            let texture_y: i32 = NumCast::from(texture_y).unwrap_or(self.props.texture_y);
            self.props = self.props.tex_position(texture_x, texture_y);
        }

        /// Sets the inner texture x position for the sprite.
        pub fn tex_position_x<TX: NumCast>(mut self, texture_x: TX) -> Self {
            let texture_x: i32 = NumCast::from(texture_x).unwrap_or(self.props.texture_x);
            self.props = self.props.tex_position(texture_x, self.props.texture_y);
            self
        }

        /// Sets the inner texture x position for the sprite.
        pub fn set_tex_position_x<TX: NumCast>(&mut self, texture_x: TX) {
            let texture_x: i32 = NumCast::from(texture_x).unwrap_or(self.props.texture_x);
            self.props = self.props.tex_position(texture_x, self.props.texture_y);
        }

        /// Sets the inner texture y position for the sprite.
        pub fn tex_position_y<TY: NumCast>(mut self, texture_y: TY) -> Self {
            let texture_y: i32 = NumCast::from(texture_y).unwrap_or(self.props.texture_y);
            self.props = self.props.tex_position(self.props.texture_x, texture_y);
            self
        }

        /// Sets the inner texture y position for the sprite.
        pub fn set_tex_position_y<TY: NumCast>(&mut self, texture_y: TY) {
            let texture_y: i32 = NumCast::from(texture_y).unwrap_or(self.props.texture_y);
            self.props = self.props.tex_position(self.props.texture_x, texture_y);
        }

        /// Sets the inner texture x and y position for the sprite.
        pub fn tex_position_xy<TX: NumCast, TY: NumCast>(
            mut self,
            (texture_x, texture_y): (TX, TY),
        ) -> Self {
            let texture_x: i32 = NumCast::from(texture_x).unwrap_or(self.props.texture_x);
            let texture_y: i32 = NumCast::from(texture_y).unwrap_or(self.props.texture_y);
            self.props = self.props.tex_position(texture_x, texture_y);
            self
        }

        /// Sets the inner texture x and y position for the sprite.
        pub fn set_tex_position_xy<TX: NumCast, TY: NumCast>(
            &mut self,
            (texture_x, texture_y): (TX, TY),
        ) {
            let texture_x: i32 = NumCast::from(texture_x).unwrap_or(self.props.texture_x);
            let texture_y: i32 = NumCast::from(texture_y).unwrap_or(self.props.texture_y);
            self.props = self.props.tex_position(texture_x, texture_y);
        }

        /// Sets the color to blend with the sprite's texture.
        pub fn color(mut self, color: u32) -> Self {
            self.props = self.props.color(color);
            self
        }

        /// Sets the color to blend with the sprite's texture.
        pub fn set_color(&mut self, color: u32) {
            self.props = self.props.color(color);
        }

        /// Sets the background color.
        pub fn background_color(mut self, bg: u32) -> Self {
            self.props = self.props.background_color(bg);
            self
        }

        /// Sets the background color.
        pub fn set_background_color(&mut self, bg: u32) {
            self.props = self.props.background_color(bg);
        }

        /// Sets the border radius.
        pub fn border_radius<R: NumCast>(mut self, radius: R) -> Self {
            let radius: u32 = NumCast::from(radius).unwrap_or(0);
            self.props = self.props.border_radius(radius);
            self
        }

        /// Sets the border radius.
        pub fn set_border_radius<R: NumCast>(&mut self, radius: R) {
            let radius: u32 = NumCast::from(radius).unwrap_or(0);
            self.props = self.props.border_radius(radius);
        }

        /// Sets the origin for transformations.
        pub fn origin<X: NumCast, Y: NumCast>(mut self, origin_x: X, origin_y: Y) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.props.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.props.origin_y);
            self.props = self.props.origin(origin_x, origin_y);
            self
        }

        /// Sets the origin for transformations.
        pub fn set_origin<X: NumCast, Y: NumCast>(&mut self, origin_x: X, origin_y: Y) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.props.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.props.origin_y);
            self.props = self.props.origin(origin_x, origin_y);
        }

        /// Sets the x origin for transformations.
        pub fn origin_x<X: NumCast>(mut self, origin_x: X) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.props.origin_x);
            self.props = self.props.origin(origin_x, self.props.origin_y);
            self
        }

        /// Sets the x origin for transformations.
        pub fn set_origin_x<X: NumCast>(&mut self, origin_x: X) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.props.origin_x);
            self.props = self.props.origin(origin_x, self.props.origin_y);
        }

        /// Sets the y origin for transformations.
        pub fn origin_y<Y: NumCast>(mut self, origin_y: Y) -> Self {
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.props.origin_y);
            self.props = self.props.origin(self.props.origin_x, origin_y);
            self
        }

        /// Sets the y origin for transformations.
        pub fn set_origin_y<Y: NumCast>(&mut self, origin_y: Y) {
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.props.origin_y);
            self.props = self.props.origin(self.props.origin_x, origin_y);
        }

        /// Sets the x and y origin for transformations.
        pub fn origin_xy<X: NumCast, Y: NumCast>(mut self, (origin_x, origin_y): (X, Y)) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.props.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.props.origin_y);
            self.props = self.props.origin(origin_x, origin_y);
            self
        }

        /// Sets the x and y origin for transformations.
        pub fn set_origin_xy<X: NumCast, Y: NumCast>(&mut self, (origin_x, origin_y): (X, Y)) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.props.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.props.origin_y);
            self.props = self.props.origin(origin_x, origin_y);
        }

        /// Sets the rotation angle by degrees.
        pub fn rotation_deg<A: NumCast>(mut self, degrees: A) -> Self {
            let degrees: i32 = NumCast::from(degrees).unwrap_or(0);
            self.props = self.props.rotation(degrees);
            self
        }

        /// Sets the rotation angle by degrees.
        pub fn set_rotation_deg<A: NumCast>(&mut self, degrees: A) {
            let degrees: i32 = NumCast::from(degrees).unwrap_or(0);
            self.props = self.props.rotation(degrees);
        }

        /// Sets the rotation angle by radians.
        pub fn rotation_rad<R: NumCast>(mut self, radians: R) -> Self {
            let radian: f32 = NumCast::from(radians).unwrap_or(0.);
            let angle = (radian * 180. / std::f32::consts::PI).round() as i32;
            self.props = self.props.rotation(angle);
            self
        }

        /// Sets the rotation angle by radians.
        pub fn set_rotation_rad<R: NumCast>(&mut self, radians: R) {
            let radian: f32 = NumCast::from(radians).unwrap_or(0.);
            let angle = (radian * 180. / std::f32::consts::PI).round() as i32;
            self.props = self.props.rotation(angle);
        }

        /// Sets the scale factor.
        pub fn scale<S: NumCast>(mut self, scale: S) -> Self {
            let scale: f32 = NumCast::from(scale).unwrap_or(1.0);
            self.props = self.props.scale(scale, scale);
            self
        }

        /// Sets the scale factor.
        pub fn set_scale<S: NumCast>(&mut self, scale: S) {
            let scale: f32 = NumCast::from(scale).unwrap_or(1.0);
            self.props = self.props.scale(scale, scale);
        }

        /// Sets the x scale factor.
        pub fn scale_x<SX: NumCast>(mut self, scale_x: SX) -> Self {
            let scale_x: f32 = NumCast::from(scale_x).unwrap_or(1.0);
            self.props = self.props.scale(scale_x, self.props.scale_y);
            self
        }

        /// Sets the x scale factor.
        pub fn set_scale_x<SX: NumCast>(&mut self, scale_x: SX) {
            let scale_x: f32 = NumCast::from(scale_x).unwrap_or(1.0);
            self.props = self.props.scale(scale_x, self.props.scale_y);
        }

        /// Sets the y scale factors.
        pub fn scale_y<SY: NumCast>(mut self, scale_y: SY) -> Self {
            let scale_y: f32 = NumCast::from(scale_y).unwrap_or(1.0);
            self.props = self.props.scale(self.props.scale_x, scale_y);
            self
        }

        /// Sets the y scale factors.
        pub fn set_scale_y<SY: NumCast>(&mut self, scale_y: SY) {
            let scale_y: f32 = NumCast::from(scale_y).unwrap_or(1.0);
            self.props = self.props.scale(self.props.scale_x, scale_y);
        }

        /// Sets the x and y scale factors.
        pub fn scale_xy<SX: NumCast, SY: NumCast>(mut self, (scale_x, scale_y): (SX, SY)) -> Self {
            let scale_x: f32 = NumCast::from(scale_x).unwrap_or(1.0);
            let scale_y: f32 = NumCast::from(scale_y).unwrap_or(1.0);
            self.props = self.props.scale(scale_x, scale_y);
            self
        }

        /// Sets the x and y scale factors.
        pub fn set_scale_xy<SX: NumCast, SY: NumCast>(&mut self, (scale_x, scale_y): (SX, SY)) {
            let scale_x: f32 = NumCast::from(scale_x).unwrap_or(1.0);
            let scale_y: f32 = NumCast::from(scale_y).unwrap_or(1.0);
            self.props = self.props.scale(scale_x, scale_y);
        }

        /// Flips the sprite horizontally and/or vertically.
        pub fn flip(mut self, flip_x: bool, flip_y: bool) -> Self {
            self.props = self.props.flip(flip_x, flip_y);
            self
        }

        /// Flips the sprite horizontally and/or vertically.
        pub fn set_flip(&mut self, flip_x: bool, flip_y: bool) {
            self.props = self.props.flip(flip_x, flip_y);
        }

        /// Flips the sprite horizontally.
        pub fn flip_x(mut self, flip_x: bool) -> Self {
            self.props = self.props.flip(flip_x, self.props.flip_y);
            self
        }

        /// Flips the sprite horizontally.
        pub fn set_flip_x(&mut self, flip_x: bool) {
            self.props = self.props.flip(flip_x, self.props.flip_y);
        }

        /// Flips the sprite vertically.
        pub fn flip_y(mut self, flip_y: bool) -> Self {
            self.props = self.props.flip(self.props.flip_x, flip_y);
            self
        }

        /// Flips the sprite vertically.
        pub fn set_flip_y(&mut self, flip_y: bool) {
            self.props = self.props.flip(self.props.flip_x, flip_y);
        }

        /// Enables or disables texture repeating.
        pub fn repeat(mut self, repeat: bool) -> Self {
            self.props = self.props.repeat(repeat);
            self
        }

        /// Enables or disables texture repeating.
        pub fn set_repeat(&mut self, repeat: bool) {
            self.props = self.props.repeat(repeat);
        }

        /// Enables or disables absolute positioning.
        pub fn absolute(mut self, absolute: bool) -> Self {
            self.props = self.props.absolute(absolute);
            self
        }

        /// Enables or disables absolute positioning.
        pub fn set_absolute(mut self, absolute: bool) {
            self.props = self.props.absolute(absolute);
        }

        /// Sets the opacity.
        pub fn opacity(mut self, opacity: f32) -> Self {
            self.props = self.props.opacity(opacity);
            self
        }

        /// Sets the opacity.
        pub fn set_opacity(&mut self, opacity: f32) {
            self.props = self.props.opacity(opacity);
        }

        /// Sets the animation speed.
        pub fn animation_speed(mut self, speed: f32) -> Self {
            self.props = self.props.animation_speed(speed);
            self
        }

        /// Sets the animation speed.
        pub fn set_animation_speed(&mut self, speed: f32) {
            self.props = self.props.animation_speed(speed);
        }

        /// Sets a fixed frame.
        pub fn frame(mut self, frame: usize) -> Self {
            self.props = self.props.frame(frame);
            self
        }

        /// Sets a fixed frame.
        pub fn set_frame(&mut self, frame: usize) {
            self.props = self.props.frame(frame);
        }

        /// Uses an animation key to set the sprite's animation frame
        pub fn animation_key(mut self, animation_key: &str) -> Self {
            let sprite = crate::canvas::animation(animation_key).sprite();
            self.props.frame = sprite.props.frame;
            self
        }

        // Draws the sprite
        pub fn draw(&self) {
            // Attempt to retrieve sprite source data using the sprite's name.
            // If not found, exit early.
            let Some(sprite_data) = utils::sprite::get_source_data(&self.name) else {
                return;
            };

            // Initialize flags used to modify drawing behavior.
            let mut flags: u32 = 0;

            // Set initial destination coordinates from sprite properties.
            let mut dx = self.props.x;
            let mut dy = self.props.y;

            // If absolute positioning is enabled, adjust coordinates relative to the camera.
            if self.props.absolute {
                let (cx, cy) = crate::canvas::camera::xy(); // Retrieve camera coordinates.
                let (w, h) = crate::canvas::resolution(); // Get canvas dimensions.
                dx += cx as i32 - (w as i32 / 2); // Center the sprite horizontally.
                dy += cy as i32 - (h as i32 / 2); // Center the sprite vertically.
            }

            // Determine the destination width (dw) and height (dh) by either using provided dimensions
            // or falling back to the sprite data dimensions, then applying scaling factors.
            let dw = ((if self.props.w == 0 {
                sprite_data.width
            } else {
                self.props.w
            }) as f32
                * self.props.scale_x) as u32;
            let dh = ((if self.props.h == 0 {
                sprite_data.height
            } else {
                self.props.h
            }) as f32
                * self.props.scale_y) as u32;

            // If scaling is applied (i.e., not 1:1), set a flag indicating the sprite should cover the area.
            if self.props.scale_x != 1. || self.props.scale_y != 1. {
                flags |= flags::SPRITE_COVER;
            }

            // Calculate source width (sw) based on horizontal flip.
            let sw = if self.props.flip_x {
                sprite_data.width as i32 * -1 // Negative width indicates a horizontal flip.
            } else {
                sprite_data.width as i32
            };

            // Calculate source height (sh) based on vertical flip.
            let sh = if self.props.flip_y {
                sprite_data.height as i32 * -1 // Negative height indicates a vertical flip.
            } else {
                sprite_data.height as i32
            };

            // Apply opacity to the sprite's primary and background colors.
            let color = utils::color::apply_opacity(self.props.color, self.props.opacity);
            let background_color =
                utils::color::apply_opacity(self.props.background_color, self.props.opacity);

            // Determine the frame index for animation:
            // If a frame is explicitly set in props, use it; otherwise, compute it based on animation speed.
            let frame_index = self.props.frame.unwrap_or_else(|| {
                utils::sprite::get_frame_index(&sprite_data, self.props.animation_speed)
            }) % sprite_data.animation_frames.len();

            // If the sprite is set to repeat, mark the repeat flag.
            if self.props.repeat {
                flags |= flags::SPRITE_REPEAT;
            }

            // Calculate the x and y position of the current sprite frame within the spritesheet
            let sx = sprite_data.x + (sprite_data.width * frame_index as u32);
            let sy = sprite_data.y;

            // Finally, draw the sprite using the calculated parameters.
            utils::sprite::draw(
                dx,                       // Adjusted x-coordinate for drawing.
                dy,                       // Adjusted y-coordinate for drawing.
                dw,                       // Drawing width.
                dh,                       // Drawing height.
                sx,                       // Source x-coordinate on the texture.
                sy,                       // Source y-coordinate on the texture.
                sw,                       // Source width (with flip adjustments).
                sh,                       // Source height (with flip adjustments).
                self.props.texture_x,     // Texture x offset.
                self.props.texture_y,     // Texture y offset.
                color,                    // Color with opacity applied.
                background_color,         // Background color with opacity applied.
                self.props.border_radius, // Border radius for rounded corners.
                self.props.origin_x,      // Origin x-coordinate for transformations.
                self.props.origin_y,      // Origin y-coordinate for transformations.
                self.props.rotation,      // Rotation angle.
                flags,                    // Flags that affect drawing behavior.
            );
        }
    }

    // TODO: opacity, origin, rotation, scaling
    pub struct NineSliceSprite<'a> {
        sprite: sprite::Sprite<'a>,
        margins: (u32, u32, u32, u32), // Clockwise: left, top, right, bottom
        target: Bounds,
    }
    impl<'a> NineSliceSprite<'a> {
        /// Creates a new NineSliceSprite from a sprite and specified margins.
        pub fn new(sprite: sprite::Sprite<'a>, margins: (u32, u32, u32, u32)) -> Self {
            Self {
                sprite,
                margins,
                target: Bounds::default(),
            }
        }

        /// Sets the nine-slice’s position.
        pub fn position<X: NumCast, Y: NumCast>(mut self, x: X, y: Y) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.target.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.target.y);
            self.target = self.target.position(x, y);
            self
        }

        /// Sets the nine-slice’s position.
        pub fn set_position<X: NumCast, Y: NumCast>(&mut self, x: X, y: Y) {
            let x: i32 = NumCast::from(x).unwrap_or(self.target.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.target.y);
            self.target = self.target.position(x, y);
        }

        /// Sets the nine-slice’s x position.
        pub fn position_x<X: NumCast>(mut self, x: X) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.target.x);
            self.target = self.target.position(x, self.target.y);
            self
        }

        /// Sets the nine-slice’s x position.
        pub fn set_position_x<X: NumCast>(&mut self, x: X) {
            let x: i32 = NumCast::from(x).unwrap_or(self.target.x);
            self.target = self.target.position(x, self.target.y);
        }

        /// Sets the nine-slice’s y position.
        pub fn position_y<Y: NumCast>(mut self, y: Y) -> Self {
            let y: i32 = NumCast::from(y).unwrap_or(self.target.y);
            self.target = self.target.position(self.target.x, y);
            self
        }

        /// Sets the nine-slice’s y position.
        pub fn set_position_y<Y: NumCast>(mut self, y: Y) {
            let y: i32 = NumCast::from(y).unwrap_or(self.target.y);
            self.target = self.target.position(self.target.x, y);
        }

        /// Sets the nine-slice’s x and y position.
        pub fn position_xy<X: NumCast, Y: NumCast>(mut self, (x, y): (X, Y)) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.target.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.target.y);
            self.target = self.target.position(x, y);
            self
        }

        /// Sets the nine-slice’s x and y position.
        pub fn set_position_xy<X: NumCast, Y: NumCast>(mut self, (x, y): (X, Y)) {
            let x: i32 = NumCast::from(x).unwrap_or(self.target.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.target.y);
            self.target = self.target.position(x, y);
        }

        /// Sets the nine-slice’s size.
        pub fn size<W: NumCast, H: NumCast>(mut self, w: W, h: H) -> Self {
            let w: u32 = NumCast::from(w).unwrap_or(self.target.w);
            let h: u32 = NumCast::from(h).unwrap_or(self.target.h);
            self.target = self.target.size(w, h);
            self
        }

        /// Sets the nine-slice’s size.
        pub fn set_size<W: NumCast, H: NumCast>(mut self, w: W, h: H) {
            let w: u32 = NumCast::from(w).unwrap_or(self.target.w);
            let h: u32 = NumCast::from(h).unwrap_or(self.target.h);
            self.target = self.target.size(w, h);
        }

        /// Sets the nine-slice’s width.
        pub fn set_size_w<W: NumCast>(mut self, w: W) {
            let w: u32 = NumCast::from(w).unwrap_or(self.target.w);
            self.target = self.target.size(w, self.target.h);
        }

        /// Sets the nine-slice’s height.
        pub fn size_h<H: NumCast>(mut self, h: H) -> Self {
            let h: u32 = NumCast::from(h).unwrap_or(self.target.h);
            self.target = self.target.size(self.target.w, h);
            self
        }

        /// Sets the nine-slice’s height.
        pub fn set_size_h<H: NumCast>(mut self, h: H) {
            let h: u32 = NumCast::from(h).unwrap_or(self.target.h);
            self.target = self.target.size(self.target.w, h);
        }

        /// Sets the nine-slice’s width and height.
        pub fn size_wh<W: NumCast, H: NumCast>(mut self, (w, h): (W, H)) -> Self {
            let w: u32 = NumCast::from(w).unwrap_or(self.target.w);
            let h: u32 = NumCast::from(h).unwrap_or(self.target.h);
            self.target = self.target.size(w, h);
            self
        }

        /// Sets the nine-slice’s width and height.
        pub fn set_size_wh<W: NumCast, H: NumCast>(mut self, (w, h): (W, H)) {
            let w: u32 = NumCast::from(w).unwrap_or(self.target.w);
            let h: u32 = NumCast::from(h).unwrap_or(self.target.h);
            self.target = self.target.size(w, h);
        }

        /// Sets the nine-slice’s width.
        pub fn width<W: NumCast>(mut self, w: W) -> Self {
            let w: u32 = NumCast::from(w).unwrap_or(self.target.w);
            self.target = self.target.size(w, self.target.h);
            self
        }

        /// Sets the nine-slice’s width.
        pub fn set_width<W: NumCast>(mut self, w: W) {
            let w: u32 = NumCast::from(w).unwrap_or(self.target.w);
            self.target = self.target.size(w, self.target.h);
        }

        /// Sets the nine-slice’s height.
        pub fn height<H: NumCast>(mut self, h: H) -> Self {
            let h: u32 = NumCast::from(h).unwrap_or(self.target.h);
            self.target = self.target.size(self.target.w, h);
            self
        }

        /// Sets the nine-slice’s height.
        pub fn set_height<H: NumCast>(mut self, h: H) {
            let h: u32 = NumCast::from(h).unwrap_or(self.target.h);
            self.target = self.target.size(self.target.w, h);
        }

        /// Translates the nine-slice’s position by the given delta.
        pub fn offset<DX: NumCast, DY: NumCast>(mut self, dx: DX, dy: DY) -> Self {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.target = self.target.translate(dx, dy);
            self
        }

        /// Translates the nine-slice’s position by the given delta.
        pub fn set_offset<DX: NumCast, DY: NumCast>(mut self, dx: DX, dy: DY) {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.target = self.target.translate(dx, dy);
        }

        /// Translates the nine-slice’s x position by the given delta.
        pub fn offset_x<DX: NumCast>(mut self, dx: DX) -> Self {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            self.target = self.target.translate(dx, 0);
            self
        }

        /// Translates the nine-slice’s x position by the given delta.
        pub fn set_offset_x<DX: NumCast>(mut self, dx: DX) {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            self.target = self.target.translate(dx, 0);
        }

        /// Translates the nine-slice’s position by the given delta.
        pub fn offset_y<DY: NumCast>(mut self, dy: DY) -> Self {
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.target = self.target.translate(0, dy);
            self
        }

        /// Translates the nine-slice’s position by the given delta.
        pub fn set_offset_y<DY: NumCast>(mut self, dy: DY) {
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.target = self.target.translate(0, dy);
        }

        /// Translates the nine-slice’s position by the given delta.
        pub fn offset_xy<DX: NumCast, DY: NumCast>(mut self, (dx, dy): (DX, DY)) -> Self {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.target = self.target.translate(dx, dy);
            self
        }

        /// Translates the nine-slice’s position by the given delta.
        pub fn set_offset_xy<DX: NumCast, DY: NumCast>(mut self, (dx, dy): (DX, DY)) {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.target = self.target.translate(dx, dy);
        }

        pub fn target(mut self, target: Bounds) -> Self {
            self.target = target;
            self
        }

        /// Draws the sprite using nine-slice scaling within the given target bounds.
        pub fn draw(&self) {
            let source_data = match utils::sprite::get_source_data(self.sprite.name) {
                Some(data) => data,
                None => return,
            };

            let (ml, mt, mr, mb) = self.margins;
            let src = Bounds {
                x: source_data.x as i32,
                y: source_data.y as i32,
                w: source_data.width,
                h: source_data.height,
            };

            let target = self.target;

            let src_top_left = Bounds::with_size(ml, mt).position(src.x, src.y);
            let src_top_center = Bounds::with_size(src.w.saturating_sub(ml + mr), mt)
                .position(src.x + ml as i32, src.y);
            let src_top_right =
                Bounds::with_size(mr, mt).position(src.x + src.w as i32 - mr as i32, src.y);

            let src_mid_left = Bounds::with_size(ml, src.h.saturating_sub(mt + mb))
                .position(src.x, src.y + mt as i32);
            let src_center =
                Bounds::with_size(src.w.saturating_sub(ml + mr), src.h.saturating_sub(mt + mb))
                    .position(src.x + ml as i32, src.y + mt as i32);
            let src_mid_right = Bounds::with_size(mr, src.h.saturating_sub(mt + mb))
                .position(src.x + src.w as i32 - mr as i32, src.y + mt as i32);

            let src_bottom_left =
                Bounds::with_size(ml, mb).position(src.x, src.y + src.h as i32 - mb as i32);
            let src_bottom_center = Bounds::with_size(src.w.saturating_sub(ml + mr), mb)
                .position(src.x + ml as i32, src.y + src.h as i32 - mb as i32);
            let src_bottom_right = Bounds::with_size(mr, mb).position(
                src.x + src.w as i32 - mr as i32,
                src.y + src.h as i32 - mb as i32,
            );

            let dst_top_left = Bounds::with_size(ml, mt).position(target.x, target.y);
            let dst_top_center = Bounds::with_size(target.w.saturating_sub(ml + mr), mt)
                .position(target.x + ml as i32, target.y);
            let dst_top_right = Bounds::with_size(mr, mt)
                .position(target.x + target.w as i32 - mr as i32, target.y);

            let dst_mid_left = Bounds::with_size(ml, target.h.saturating_sub(mt + mb))
                .position(target.x, target.y + mt as i32);
            let dst_center = Bounds::with_size(
                target.w.saturating_sub(ml + mr),
                target.h.saturating_sub(mt + mb),
            )
            .position(target.x + ml as i32, target.y + mt as i32);
            let dst_mid_right = Bounds::with_size(mr, target.h.saturating_sub(mt + mb))
                .position(target.x + target.w as i32 - mr as i32, target.y + mt as i32);

            let dst_bottom_left = Bounds::with_size(ml, mb)
                .position(target.x, target.y + target.h as i32 - mb as i32);
            let dst_bottom_center = Bounds::with_size(target.w.saturating_sub(ml + mr), mb)
                .position(target.x + ml as i32, target.y + target.h as i32 - mb as i32);
            let dst_bottom_right = Bounds::with_size(mr, mb).position(
                target.x + target.w as i32 - mr as i32,
                target.y + target.h as i32 - mb as i32,
            );

            let tex_x = self.sprite.props.texture_x;
            let tex_y = self.sprite.props.texture_y;
            let color =
                utils::color::apply_opacity(self.sprite.props.color, self.sprite.props.opacity);
            let bg_color = utils::color::apply_opacity(
                self.sprite.props.background_color,
                self.sprite.props.opacity,
            );
            let border_radius = self.sprite.props.border_radius;
            let origin_x = self.sprite.props.origin_x;
            let origin_y = self.sprite.props.origin_y;
            let rotation = self.sprite.props.rotation;
            let flags = sprite::flags::SPRITE_REPEAT;

            Self::draw_region(
                &dst_top_left,
                &src_top_left,
                tex_x,
                tex_y,
                color,
                bg_color,
                border_radius,
                origin_x,
                origin_y,
                rotation,
                flags,
            );
            Self::draw_region(
                &dst_top_center,
                &src_top_center,
                tex_x,
                tex_y,
                color,
                bg_color,
                border_radius,
                origin_x,
                origin_y,
                rotation,
                flags,
            );
            Self::draw_region(
                &dst_top_right,
                &src_top_right,
                tex_x,
                tex_y,
                color,
                bg_color,
                border_radius,
                origin_x,
                origin_y,
                rotation,
                flags,
            );

            Self::draw_region(
                &dst_mid_left,
                &src_mid_left,
                tex_x,
                tex_y,
                color,
                bg_color,
                border_radius,
                origin_x,
                origin_y,
                rotation,
                flags,
            );
            Self::draw_region(
                &dst_center,
                &src_center,
                tex_x,
                tex_y,
                color,
                bg_color,
                border_radius,
                origin_x,
                origin_y,
                rotation,
                flags,
            );
            Self::draw_region(
                &dst_mid_right,
                &src_mid_right,
                tex_x,
                tex_y,
                color,
                bg_color,
                border_radius,
                origin_x,
                origin_y,
                rotation,
                flags,
            );

            Self::draw_region(
                &dst_bottom_left,
                &src_bottom_left,
                tex_x,
                tex_y,
                color,
                bg_color,
                border_radius,
                origin_x,
                origin_y,
                rotation,
                flags,
            );
            Self::draw_region(
                &dst_bottom_center,
                &src_bottom_center,
                tex_x,
                tex_y,
                color,
                bg_color,
                border_radius,
                origin_x,
                origin_y,
                rotation,
                flags,
            );
            Self::draw_region(
                &dst_bottom_right,
                &src_bottom_right,
                tex_x,
                tex_y,
                color,
                bg_color,
                border_radius,
                origin_x,
                origin_y,
                rotation,
                flags,
            );
        }

        fn draw_region(
            dst: &Bounds,
            src: &Bounds,
            texture_x: i32,
            texture_y: i32,
            color: u32,
            bg_color: u32,
            border_radius: u32,
            origin_x: i32,
            origin_y: i32,
            rotation: i32,
            flags: u32,
        ) {
            utils::sprite::draw(
                dst.x,
                dst.y,
                dst.w,
                dst.h,
                src.x as u32,
                src.y as u32,
                src.w as i32,
                src.h as i32,
                texture_x,
                texture_y,
                color,
                bg_color,
                border_radius,
                origin_x,
                origin_y,
                rotation,
                flags,
            );
        }
    }
}

//------------------------------------------------------------------------------
// QUAD
//------------------------------------------------------------------------------
pub use quad::*;
mod quad {
    /// Holds properties for a rectangle.
    #[derive(Debug, Clone, Copy)]
    pub struct Quad {
        /// X coordinate for the rectangle's position.
        pub x: i32,
        /// Y coordinate for the rectangle's position.
        pub y: i32,
        /// Width of the rectangle.
        pub w: u32,
        /// Height of the rectangle.
        pub h: u32,
        /// Primary fill color (packed ARGB/RGBA value).
        pub color: u32,
        /// Border radius for rounded corners.
        pub border_radius: u32,
        /// Border size (thickness).
        pub border_size: u32,
        /// Border color (packed ARGB/RGBA value).
        pub border_color: u32,
        /// Rotation angle in degrees.
        pub rotation_deg: i32,
        /// Indicates if the rectangle's position is absolute.
        pub absolute: bool,
        /// X coordinate of the origin (pivot) used for transformations.
        pub origin_x: i32,
        /// Y coordinate of the origin (pivot) used for transformations.
        pub origin_y: i32,
        /// Opacity level (0.0 = fully transparent, 1.0 = fully opaque).
        pub opacity: f32,
    }
    impl Default for Quad {
        fn default() -> Self {
            Self {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
                color: 0xffffffff, // Default fill is white.
                border_radius: 0,
                border_size: 0,
                border_color: 0xff000000, // Default border color is opaque black.
                origin_x: 0,
                origin_y: 0,
                rotation_deg: 0,
                opacity: 1.0,
                absolute: false,
            }
        }
    }

    impl Quad {
        /// Creates new rectangle properties with default values.
        pub fn new() -> Self {
            Self::default()
        }

        /// Sets the position of the rectangle.
        pub fn position(mut self, x: i32, y: i32) -> Self {
            self.x = x;
            self.y = y;
            self
        }

        /// Sets the size of the rectangle.
        pub fn size(mut self, w: u32, h: u32) -> Self {
            self.w = w;
            self.h = h;
            self
        }

        /// Translates the rectangle's position by the given delta.
        pub fn offset(mut self, dx: i32, dy: i32) -> Self {
            self.x += dx;
            self.y += dy;
            self
        }

        /// Sets the primary fill color of the rectangle.
        pub fn color(mut self, color: u32) -> Self {
            self.color = color;
            self
        }

        /// Sets the border radius.
        pub fn border_radius(mut self, radius: u32) -> Self {
            self.border_radius = radius;
            self
        }

        /// Sets the border size (thickness).
        pub fn border_size(mut self, size: u32) -> Self {
            self.border_size = size;
            self
        }

        /// Sets the border color.
        pub fn border_color(mut self, color: u32) -> Self {
            self.border_color = color;
            self
        }

        /// Sets the origin point for transformations.
        pub fn origin(mut self, origin_x: i32, origin_y: i32) -> Self {
            self.origin_x = origin_x;
            self.origin_y = origin_y;
            self
        }

        /// Sets the rotation angle (in degrees) for the rectangle.
        pub fn rotation(mut self, angle: i32) -> Self {
            self.rotation_deg = angle;
            self
        }

        /// Sets whether the rectangle uses absolute positioning.
        pub fn absolute(mut self, absolute: bool) -> Self {
            self.absolute = absolute;
            self
        }

        /// Sets the opacity.
        pub fn opacity(mut self, opacity: f32) -> Self {
            self.opacity = opacity;
            self
        }
    }
}

//------------------------------------------------------------------------------
// RECT
//------------------------------------------------------------------------------
pub mod rect {
    use super::*;
    use num_traits::NumCast;

    /// A builder-style rectangle type.
    #[derive(Debug, Clone, Copy)]
    pub struct Rectangle {
        quad: Quad,
    }

    impl Rectangle {
        /// Creates a new rectangle with default properties.
        pub fn new() -> Self {
            Self {
                quad: Quad::default(),
            }
        }

        /// Sets the rectangle's position.
        pub fn position<X: NumCast, Y: NumCast>(mut self, x: X, y: Y) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(x, y);
            self
        }

        /// Sets the rectangle's position.
        pub fn set_position<X: NumCast, Y: NumCast>(&mut self, x: X, y: Y) {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(x, y);
        }

        /// Sets the rectangle’s x position.
        pub fn position_x<X: NumCast>(mut self, x: X) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            self.quad = self.quad.position(x, self.quad.y);
            self
        }

        /// Sets the rectangle’s x position.
        pub fn set_position_x<X: NumCast>(&mut self, x: X) {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            self.quad = self.quad.position(x, self.quad.y);
        }

        /// Sets the rectangle’s y position.
        pub fn position_y<Y: NumCast>(mut self, y: Y) -> Self {
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(self.quad.x, y);
            self
        }

        /// Sets the rectangle’s y position.
        pub fn set_position_y<Y: NumCast>(&mut self, y: Y) {
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(self.quad.x, y);
        }

        /// Sets the rectangle’s x and y position.
        pub fn position_xy<X: NumCast, Y: NumCast>(mut self, (x, y): (X, Y)) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(x, y);
            self
        }

        /// Sets the rectangle’s x and y position.
        pub fn set_position_xy<X: NumCast, Y: NumCast>(&mut self, (x, y): (X, Y)) {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(x, y);
        }

        /// Sets the rectangle's size.
        pub fn size<W: NumCast, H: NumCast>(mut self, w: W, h: H) -> Self {
            let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
            let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
            self.quad = self.quad.size(w, h);
            self
        }

        /// Sets the rectangle's size.
        pub fn set_size<W: NumCast, H: NumCast>(&mut self, w: W, h: H) {
            let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
            let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
            self.quad = self.quad.size(w, h);
        }

        /// Sets the rectangle's width.
        pub fn set_size_w<W: NumCast>(&mut self, w: W) {
            let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
            self.quad = self.quad.size(w, self.quad.h);
        }

        /// Sets the rectangle's height.
        pub fn size_h<H: NumCast>(mut self, h: H) -> Self {
            let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
            self.quad = self.quad.size(self.quad.w, h);
            self
        }

        /// Sets the rectangle's height.
        pub fn set_size_h<H: NumCast>(&mut self, h: H) {
            let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
            self.quad = self.quad.size(self.quad.w, h);
        }

        /// Sets the rectangle's width and height.
        pub fn size_wh<W: NumCast, H: NumCast>(mut self, (w, h): (W, H)) -> Self {
            let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
            let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
            self.quad = self.quad.size(w, h);
            self
        }

        /// Sets the rectangle's width and height.
        pub fn set_size_wh<W: NumCast, H: NumCast>(&mut self, (w, h): (W, H)) {
            let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
            let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
            self.quad = self.quad.size(w, h);
        }

        /// Sets the rectangle's width.
        pub fn width<W: NumCast>(mut self, w: W) -> Self {
            let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
            self.quad = self.quad.size(w, self.quad.h);
            self
        }

        /// Sets the rectangle's width.
        pub fn set_width<W: NumCast>(&mut self, w: W) {
            let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
            self.quad = self.quad.size(w, self.quad.h);
        }

        /// Sets the rectangle's height.
        pub fn height<H: NumCast>(mut self, h: H) -> Self {
            let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
            self.quad = self.quad.size(self.quad.w, h);
            self
        }

        /// Sets the rectangle's height.
        pub fn set_height<H: NumCast>(&mut self, h: H) {
            let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
            self.quad = self.quad.size(self.quad.w, h);
        }

        /// Translates the rectangle’s position by the given delta.
        pub fn offset<DX: NumCast, DY: NumCast>(mut self, dx: DX, dy: DY) -> Self {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.quad = self.quad.offset(dx, dy);
            self
        }

        /// Translates the rectangle's position by the given delta.
        pub fn set_offset<DX: NumCast, DY: NumCast>(&mut self, dx: DX, dy: DY) {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.quad = self.quad.offset(dx, dy);
        }

        /// Translates the rectangle's x position by the given delta.
        pub fn offset_x<DX: NumCast>(mut self, dx: DX) -> Self {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            self.quad = self.quad.offset(dx, 0);
            self
        }

        /// Translates the rectangle's x position by the given delta.
        pub fn set_offset_x<DX: NumCast>(&mut self, dx: DX) {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            self.quad = self.quad.offset(dx, 0);
        }

        /// Translates the rectangle's position by the given delta.
        pub fn offset_y<DY: NumCast>(mut self, dy: DY) -> Self {
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.quad = self.quad.offset(0, dy);
            self
        }

        /// Sets the rectangle's primary fill color.
        pub fn color(mut self, color: u32) -> Self {
            self.quad = self.quad.color(color);
            self
        }

        /// Sets the rectangle's primary fill color.
        pub fn set_color(&mut self, color: u32) {
            self.quad = self.quad.color(color);
        }

        /// Sets the border radius.
        pub fn border_radius<R: NumCast>(mut self, radius: R) -> Self {
            let radius: u32 = NumCast::from(radius).unwrap_or(0);
            self.quad = self.quad.border_radius(radius);
            self
        }

        /// Sets the border radius.
        pub fn set_border_radius<R: NumCast>(&mut self, radius: R) {
            let radius: u32 = NumCast::from(radius).unwrap_or(0);
            self.quad = self.quad.border_radius(radius);
        }

        /// Sets the border size.
        pub fn border_size(mut self, size: u32) -> Self {
            self.quad = self.quad.border_size(size);
            self
        }

        /// Sets the border size.
        pub fn set_border_size(&mut self, bg: u32) {
            self.quad = self.quad.border_size(bg);
        }

        /// Sets the border color.
        pub fn border_color(mut self, color: u32) -> Self {
            self.quad = self.quad.border_color(color);
            self
        }

        /// Sets the border color.
        pub fn set_border_color(&mut self, bg: u32) {
            self.quad = self.quad.border_color(bg);
        }

        /// Sets the origin for transformations.
        pub fn origin<X: NumCast, Y: NumCast>(mut self, origin_x: X, origin_y: Y) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
            self
        }

        /// Sets the origin for transformations.
        pub fn set_origin<X: NumCast, Y: NumCast>(&mut self, origin_x: X, origin_y: Y) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
        }

        /// Sets the x origin for transformations.
        pub fn origin_x<X: NumCast>(mut self, origin_x: X) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            self.quad = self.quad.origin(origin_x, self.quad.origin_y);
            self
        }

        /// Sets the x origin for transformations.
        pub fn set_origin_x<X: NumCast>(&mut self, origin_x: X) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            self.quad = self.quad.origin(origin_x, self.quad.origin_y);
        }

        /// Sets the y origin for transformations.
        pub fn origin_y<Y: NumCast>(mut self, origin_y: Y) -> Self {
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(self.quad.origin_x, origin_y);
            self
        }

        /// Sets the y origin for transformations.
        pub fn set_origin_y<Y: NumCast>(&mut self, origin_y: Y) {
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(self.quad.origin_x, origin_y);
        }

        /// Sets the x and y origin for transformations.
        pub fn origin_xy<X: NumCast, Y: NumCast>(mut self, (origin_x, origin_y): (X, Y)) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
            self
        }

        /// Sets the x and y origin for transformations.
        pub fn set_origin_xy<X: NumCast, Y: NumCast>(&mut self, (origin_x, origin_y): (X, Y)) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
        }

        /// Sets the rotation angle by degrees.
        pub fn rotation_deg<A: NumCast>(mut self, degrees: A) -> Self {
            let degrees: i32 = NumCast::from(degrees).unwrap_or(0);
            self.quad = self.quad.rotation(degrees);
            self
        }

        /// Sets the rotation angle by degrees.
        pub fn set_rotation_deg<A: NumCast>(&mut self, degrees: A) {
            let degrees: i32 = NumCast::from(degrees).unwrap_or(0);
            self.quad = self.quad.rotation(degrees);
        }

        /// Sets the rotation angle by radians.
        pub fn rotation_rad<R: NumCast>(mut self, radians: R) -> Self {
            let radian: f32 = NumCast::from(radians).unwrap_or(0.);
            let angle = (radian * 180. / std::f32::consts::PI).round() as i32;
            self.quad = self.quad.rotation(angle);
            self
        }

        /// Sets the rotation angle by radians.
        pub fn set_rotation_rad<R: NumCast>(&mut self, radians: R) {
            let radian: f32 = NumCast::from(radians).unwrap_or(0.);
            let angle = (radian * 180. / std::f32::consts::PI).round() as i32;
            self.quad = self.quad.rotation(angle);
        }

        /// Enables or disables absolute positioning.
        pub fn absolute(mut self, absolute: bool) -> Self {
            self.quad = self.quad.absolute(absolute);
            self
        }

        /// Enables or disables absolute positioning.
        pub fn set_absolute(&mut self, absolute: bool) {
            self.quad = self.quad.absolute(absolute);
        }

        /// Sets the opacity.
        pub fn opacity(mut self, opacity: f32) -> Self {
            self.quad = self.quad.opacity(opacity);
            self
        }

        /// Sets the opacity.
        pub fn set_opacity(&mut self, opacity: f32) {
            self.quad = self.quad.opacity(opacity);
        }

        /// Draws the rectangle.
        pub fn draw(&self) {
            // Calculate destination coordinates.
            let mut dx = self.quad.x;
            let mut dy = self.quad.y;

            // If absolute positioning is enabled, adjust coordinates relative to the camera.
            if self.quad.absolute {
                let (cx, cy) = crate::canvas::camera::xy(); // Retrieve camera coordinates.
                let (w, h) = crate::canvas::resolution(); // Get canvas dimensions.
                dx += cx as i32 - (w as i32 / 2); // Center the sprite horizontally.
                dy += cy as i32 - (h as i32 / 2); // Center the sprite vertically.
            }

            // Apply opacity to the sprite's primary and background colors.
            let color = utils::color::apply_opacity(self.quad.color, self.quad.opacity);
            let border_color =
                utils::color::apply_opacity(self.quad.border_color, self.quad.opacity);

            // Draw the rectangle using the utility function.
            utils::rect::draw(
                color,                   // Fill color.
                dx,                      // x-coordinate.
                dy,                      // y-coordinate.
                self.quad.w,             // Width.
                self.quad.h,             // Height.
                self.quad.border_radius, // Border radius.
                self.quad.border_size,   // Border thickness.
                border_color,            // Border color.
                self.quad.origin_x,      // X rotation origin
                self.quad.origin_y,      // Y rotation origin
                self.quad.rotation_deg,  // Rotation in degrees.
            );
        }
    }
}

//------------------------------------------------------------------------------
// ELLIPSE
//------------------------------------------------------------------------------
pub mod ellipse {
    use super::*;
    use num_traits::NumCast;

    /// A builder-style rectangle type.
    #[derive(Debug, Clone, Copy)]
    pub struct Ellipse {
        quad: Quad,
    }

    impl Ellipse {
        /// Creates a new rectangle with default properties.
        pub fn new() -> Self {
            Self {
                quad: Quad::default(),
            }
        }

        /// Sets the rectangle's position.
        pub fn position<X: NumCast, Y: NumCast>(mut self, x: X, y: Y) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(x, y);
            self
        }

        /// Sets the rectangle's position.
        pub fn set_position<X: NumCast, Y: NumCast>(&mut self, x: X, y: Y) {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(x, y);
        }

        /// Sets the rectangle’s x position.
        pub fn position_x<X: NumCast>(mut self, x: X) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            self.quad = self.quad.position(x, self.quad.y);
            self
        }

        /// Sets the rectangle’s x position.
        pub fn set_position_x<X: NumCast>(&mut self, x: X) {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            self.quad = self.quad.position(x, self.quad.y);
        }

        /// Sets the rectangle’s y position.
        pub fn position_y<Y: NumCast>(mut self, y: Y) -> Self {
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(self.quad.x, y);
            self
        }

        /// Sets the rectangle’s y position.
        pub fn set_position_y<Y: NumCast>(&mut self, y: Y) {
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(self.quad.x, y);
        }

        /// Sets the rectangle’s x and y position.
        pub fn position_xy<X: NumCast, Y: NumCast>(mut self, (x, y): (X, Y)) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(x, y);
            self
        }

        /// Sets the rectangle’s x and y position.
        pub fn set_position_xy<X: NumCast, Y: NumCast>(&mut self, (x, y): (X, Y)) {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(x, y);
        }

        /// Sets the rectangle's size.
        pub fn size<W: NumCast, H: NumCast>(mut self, w: W, h: H) -> Self {
            let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
            let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
            self.quad = self.quad.size(w, h);
            self.quad = self.quad.border_radius(w.max(h));
            self
        }

        /// Sets the rectangle's size.
        pub fn set_size<W: NumCast, H: NumCast>(&mut self, w: W, h: H) {
            let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
            let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
            self.quad = self.quad.size(w, h);
            self.quad = self.quad.border_radius(w.max(h));
        }

        /// Sets the rectangle's width.
        pub fn set_size_w<W: NumCast>(&mut self, w: W) {
            let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
            self.quad = self.quad.size(w, self.quad.h);
            self.quad = self.quad.border_radius(w.max(self.quad.h));
        }

        /// Sets the rectangle's height.
        pub fn size_h<H: NumCast>(mut self, h: H) -> Self {
            let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
            self.quad = self.quad.size(self.quad.w, h);
            self.quad = self.quad.border_radius(self.quad.w.max(h));
            self
        }

        /// Sets the rectangle's height.
        pub fn set_size_h<H: NumCast>(&mut self, h: H) {
            let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
            self.quad = self.quad.size(self.quad.w, h);
            self.quad = self.quad.border_radius(self.quad.w.max(h));
        }

        /// Sets the rectangle's width and height.
        pub fn size_wh<W: NumCast, H: NumCast>(mut self, (w, h): (W, H)) -> Self {
            let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
            let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
            self.quad = self.quad.size(w, h);
            self.quad = self.quad.border_radius(w.max(h));
            self
        }

        /// Sets the rectangle's width and height.
        pub fn set_size_wh<W: NumCast, H: NumCast>(&mut self, (w, h): (W, H)) {
            let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
            let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
            self.quad = self.quad.size(w, h);
            self.quad = self.quad.border_radius(w.max(h));
        }

        /// Sets the rectangle's width.
        pub fn width<W: NumCast>(mut self, w: W) -> Self {
            let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
            self.quad = self.quad.size(w, self.quad.h);
            self.quad = self.quad.border_radius(w.max(self.quad.h));
            self
        }

        /// Sets the rectangle's width.
        pub fn set_width<W: NumCast>(&mut self, w: W) {
            let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
            self.quad = self.quad.size(w, self.quad.h);
            self.quad = self.quad.border_radius(w.max(self.quad.h));
        }

        /// Sets the rectangle's height.
        pub fn height<H: NumCast>(mut self, h: H) -> Self {
            let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
            self.quad = self.quad.size(self.quad.w, h);
            self.quad = self.quad.border_radius(self.quad.w.max(h));
            self
        }

        /// Sets the rectangle's height.
        pub fn set_height<H: NumCast>(&mut self, h: H) {
            let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
            self.quad = self.quad.size(self.quad.w, h);
            self.quad = self.quad.border_radius(self.quad.w.max(h));
        }

        /// Translates the rectangle’s position by the given delta.
        pub fn offset<DX: NumCast, DY: NumCast>(mut self, dx: DX, dy: DY) -> Self {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.quad = self.quad.offset(dx, dy);
            self
        }

        /// Translates the rectangle's position by the given delta.
        pub fn set_offset<DX: NumCast, DY: NumCast>(&mut self, dx: DX, dy: DY) {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.quad = self.quad.offset(dx, dy);
        }

        /// Translates the rectangle's x position by the given delta.
        pub fn offset_x<DX: NumCast>(mut self, dx: DX) -> Self {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            self.quad = self.quad.offset(dx, 0);
            self
        }

        /// Translates the rectangle's x position by the given delta.
        pub fn set_offset_x<DX: NumCast>(&mut self, dx: DX) {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            self.quad = self.quad.offset(dx, 0);
        }

        /// Translates the rectangle's position by the given delta.
        pub fn offset_y<DY: NumCast>(mut self, dy: DY) -> Self {
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.quad = self.quad.offset(0, dy);
            self
        }

        /// Sets the rectangle's primary fill color.
        pub fn color(mut self, color: u32) -> Self {
            self.quad = self.quad.color(color);
            self
        }

        /// Sets the rectangle's primary fill color.
        pub fn set_color(&mut self, color: u32) {
            self.quad = self.quad.color(color);
        }

        /// Sets the border size.
        pub fn border_size(mut self, size: u32) -> Self {
            self.quad = self.quad.border_size(size);
            self
        }

        /// Sets the border size.
        pub fn set_border_size(&mut self, bg: u32) {
            self.quad = self.quad.border_size(bg);
        }

        /// Sets the border color.
        pub fn border_color(mut self, color: u32) -> Self {
            self.quad = self.quad.border_color(color);
            self
        }

        /// Sets the border color.
        pub fn set_border_color(&mut self, bg: u32) {
            self.quad = self.quad.border_color(bg);
        }

        /// Sets the origin for transformations.
        pub fn origin<X: NumCast, Y: NumCast>(mut self, origin_x: X, origin_y: Y) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
            self
        }

        /// Sets the origin for transformations.
        pub fn set_origin<X: NumCast, Y: NumCast>(&mut self, origin_x: X, origin_y: Y) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
        }

        /// Sets the x origin for transformations.
        pub fn origin_x<X: NumCast>(mut self, origin_x: X) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            self.quad = self.quad.origin(origin_x, self.quad.origin_y);
            self
        }

        /// Sets the x origin for transformations.
        pub fn set_origin_x<X: NumCast>(&mut self, origin_x: X) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            self.quad = self.quad.origin(origin_x, self.quad.origin_y);
        }

        /// Sets the y origin for transformations.
        pub fn origin_y<Y: NumCast>(mut self, origin_y: Y) -> Self {
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(self.quad.origin_x, origin_y);
            self
        }

        /// Sets the y origin for transformations.
        pub fn set_origin_y<Y: NumCast>(&mut self, origin_y: Y) {
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(self.quad.origin_x, origin_y);
        }

        /// Sets the x and y origin for transformations.
        pub fn origin_xy<X: NumCast, Y: NumCast>(mut self, (origin_x, origin_y): (X, Y)) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
            self
        }

        /// Sets the x and y origin for transformations.
        pub fn set_origin_xy<X: NumCast, Y: NumCast>(&mut self, (origin_x, origin_y): (X, Y)) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
        }

        /// Sets the rotation angle by degrees.
        pub fn rotation_deg<A: NumCast>(mut self, degrees: A) -> Self {
            let degrees: i32 = NumCast::from(degrees).unwrap_or(0);
            self.quad = self.quad.rotation(degrees);
            self
        }

        /// Sets the rotation angle by degrees.
        pub fn set_rotation_deg<A: NumCast>(&mut self, degrees: A) {
            let degrees: i32 = NumCast::from(degrees).unwrap_or(0);
            self.quad = self.quad.rotation(degrees);
        }

        /// Sets the rotation angle by radians.
        pub fn rotation_rad<R: NumCast>(mut self, radians: R) -> Self {
            let radian: f32 = NumCast::from(radians).unwrap_or(0.);
            let angle = (radian * 180. / std::f32::consts::PI).round() as i32;
            self.quad = self.quad.rotation(angle);
            self
        }

        /// Sets the rotation angle by radians.
        pub fn set_rotation_rad<R: NumCast>(&mut self, radians: R) {
            let radian: f32 = NumCast::from(radians).unwrap_or(0.);
            let angle = (radian * 180. / std::f32::consts::PI).round() as i32;
            self.quad = self.quad.rotation(angle);
        }

        /// Enables or disables absolute positioning.
        pub fn absolute(mut self, absolute: bool) -> Self {
            self.quad = self.quad.absolute(absolute);
            self
        }

        /// Enables or disables absolute positioning.
        pub fn set_absolute(&mut self, absolute: bool) {
            self.quad = self.quad.absolute(absolute);
        }

        /// Sets the opacity.
        pub fn opacity(mut self, opacity: f32) -> Self {
            self.quad = self.quad.opacity(opacity);
            self
        }

        /// Sets the opacity.
        pub fn set_opacity(&mut self, opacity: f32) {
            self.quad = self.quad.opacity(opacity);
        }

        /// Draws the rectangle.
        pub fn draw(&self) {
            // Calculate destination coordinates.
            let mut dx = self.quad.x;
            let mut dy = self.quad.y;

            // If absolute positioning is enabled, adjust coordinates relative to the camera.
            if self.quad.absolute {
                let (cx, cy) = crate::canvas::camera::xy(); // Retrieve camera coordinates.
                let (w, h) = crate::canvas::resolution(); // Get canvas dimensions.
                dx += cx as i32 - (w as i32 / 2); // Center the sprite horizontally.
                dy += cy as i32 - (h as i32 / 2); // Center the sprite vertically.
            }

            // Calculate border radius.
            let border_radius = self.quad.w.max(self.quad.h);

            // Apply opacity to the sprite's primary and background colors.
            let color = utils::color::apply_opacity(self.quad.color, self.quad.opacity);
            let border_color =
                utils::color::apply_opacity(self.quad.border_color, self.quad.opacity);

            // Draw the rectangle using the utility function.
            utils::rect::draw(
                color,                  // Fill color.
                dx,                     // x-coordinate.
                dy,                     // y-coordinate.
                self.quad.w,            // Width.
                self.quad.h,            // Height.
                border_radius,          // Border radius.
                self.quad.border_size,  // Border thickness.
                border_color,           // Border color.
                self.quad.origin_x,     // X rotation origin
                self.quad.origin_y,     // Y rotation origin
                self.quad.rotation_deg, // Rotation in degrees.
            );
        }
    }
}

//------------------------------------------------------------------------------
// CIRC
//------------------------------------------------------------------------------
pub mod circ {
    use super::*;
    use num_traits::NumCast;

    /// A builder-style circle type.
    #[derive(Debug, Clone, Copy)]
    pub struct Circle {
        quad: Quad,
    }

    impl Circle {
        /// Creates a new circle with default properties.
        pub fn new() -> Self {
            Self {
                quad: Quad::default(),
            }
        }

        /// Sets the circle's position.
        pub fn position<X: NumCast, Y: NumCast>(mut self, x: X, y: Y) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(x, y);
            self
        }

        /// Sets the circle's position.
        pub fn set_position<X: NumCast, Y: NumCast>(&mut self, x: X, y: Y) {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(x, y);
        }

        /// Sets the circle’s x position.
        pub fn position_x<X: NumCast>(mut self, x: X) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            self.quad = self.quad.position(x, self.quad.y);
            self
        }

        /// Sets the circle’s x position.
        pub fn set_position_x<X: NumCast>(&mut self, x: X) {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            self.quad = self.quad.position(x, self.quad.y);
        }

        /// Sets the circle’s y position.
        pub fn position_y<Y: NumCast>(mut self, y: Y) -> Self {
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(self.quad.x, y);
            self
        }

        /// Sets the circle’s y position.
        pub fn set_position_y<Y: NumCast>(&mut self, y: Y) {
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(self.quad.x, y);
        }

        /// Sets the circle’s x and y position.
        pub fn position_xy<X: NumCast, Y: NumCast>(mut self, (x, y): (X, Y)) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(x, y);
            self
        }

        /// Sets the circle’s x and y position.
        pub fn set_position_xy<X: NumCast, Y: NumCast>(&mut self, (x, y): (X, Y)) {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(x, y);
        }

        /// Sets the circle's size.
        pub fn size<D: NumCast + Copy>(mut self, d: D) -> Self {
            let w: u32 = NumCast::from(d).unwrap_or(self.quad.w);
            let h: u32 = NumCast::from(d).unwrap_or(self.quad.h);
            let radius: u32 = NumCast::from(d).unwrap_or(0);
            self.quad = self.quad.size(w, h);
            self.quad = self.quad.border_radius(radius);
            self
        }

        /// Sets the circle's size.
        pub fn set_size<D: NumCast + Copy>(&mut self, d: D) {
            let w: u32 = NumCast::from(d).unwrap_or(self.quad.w);
            let h: u32 = NumCast::from(d).unwrap_or(self.quad.h);
            let radius: u32 = NumCast::from(d).unwrap_or(0);
            self.quad = self.quad.size(w, h);
            self.quad = self.quad.border_radius(radius);
        }

        /// Sets the circle's diameter.
        pub fn diameter<D: NumCast + Copy>(mut self, d: D) -> Self {
            let w: u32 = NumCast::from(d).unwrap_or(self.quad.w);
            let h: u32 = NumCast::from(d).unwrap_or(self.quad.h);
            let radius: u32 = NumCast::from(d).unwrap_or(0);
            self.quad = self.quad.size(w, h);
            self.quad = self.quad.border_radius(radius);
            self
        }

        /// Sets the circle's diameter.
        pub fn set_diameter<D: NumCast + Copy>(&mut self, d: D) {
            let w: u32 = NumCast::from(d).unwrap_or(self.quad.w);
            let h: u32 = NumCast::from(d).unwrap_or(self.quad.h);
            let radius: u32 = NumCast::from(d).unwrap_or(0);
            self.quad = self.quad.size(w, h);
            self.quad = self.quad.border_radius(radius);
        }

        /// Translates the circle’s position by the given delta.
        pub fn offset<DX: NumCast, DY: NumCast>(mut self, dx: DX, dy: DY) -> Self {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.quad = self.quad.offset(dx, dy);
            self
        }

        /// Translates the circle's position by the given delta.
        pub fn set_offset<DX: NumCast, DY: NumCast>(&mut self, dx: DX, dy: DY) {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.quad = self.quad.offset(dx, dy);
        }

        /// Translates the circle's x position by the given delta.
        pub fn offset_x<DX: NumCast>(mut self, dx: DX) -> Self {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            self.quad = self.quad.offset(dx, 0);
            self
        }

        /// Translates the circle's x position by the given delta.
        pub fn set_offset_x<DX: NumCast>(&mut self, dx: DX) {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            self.quad = self.quad.offset(dx, 0);
        }

        /// Translates the circle's position by the given delta.
        pub fn offset_y<DY: NumCast>(mut self, dy: DY) -> Self {
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.quad = self.quad.offset(0, dy);
            self
        }

        /// Sets the circle's primary fill color.
        pub fn color(mut self, color: u32) -> Self {
            self.quad = self.quad.color(color);
            self
        }

        /// Sets the circle's primary fill color.
        pub fn set_color(&mut self, color: u32) {
            self.quad = self.quad.color(color);
        }

        /// Sets the border size.
        pub fn border_size(mut self, size: u32) -> Self {
            self.quad = self.quad.border_size(size);
            self
        }

        /// Sets the border size.
        pub fn set_border_size(&mut self, bg: u32) {
            self.quad = self.quad.border_size(bg);
        }

        /// Sets the border color.
        pub fn border_color(mut self, color: u32) -> Self {
            self.quad = self.quad.border_color(color);
            self
        }

        /// Sets the border color.
        pub fn set_border_color(&mut self, bg: u32) {
            self.quad = self.quad.border_color(bg);
        }

        /// Sets the origin for transformations.
        pub fn origin<X: NumCast, Y: NumCast>(mut self, origin_x: X, origin_y: Y) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
            self
        }

        /// Sets the origin for transformations.
        pub fn set_origin<X: NumCast, Y: NumCast>(&mut self, origin_x: X, origin_y: Y) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
        }

        /// Sets the x origin for transformations.
        pub fn origin_x<X: NumCast>(mut self, origin_x: X) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            self.quad = self.quad.origin(origin_x, self.quad.origin_y);
            self
        }

        /// Sets the x origin for transformations.
        pub fn set_origin_x<X: NumCast>(&mut self, origin_x: X) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            self.quad = self.quad.origin(origin_x, self.quad.origin_y);
        }

        /// Sets the y origin for transformations.
        pub fn origin_y<Y: NumCast>(mut self, origin_y: Y) -> Self {
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(self.quad.origin_x, origin_y);
            self
        }

        /// Sets the y origin for transformations.
        pub fn set_origin_y<Y: NumCast>(&mut self, origin_y: Y) {
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(self.quad.origin_x, origin_y);
        }

        /// Sets the x and y origin for transformations.
        pub fn origin_xy<X: NumCast, Y: NumCast>(mut self, (origin_x, origin_y): (X, Y)) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
            self
        }

        /// Sets the x and y origin for transformations.
        pub fn set_origin_xy<X: NumCast, Y: NumCast>(&mut self, (origin_x, origin_y): (X, Y)) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
        }

        /// Enables or disables absolute positioning.
        pub fn absolute(mut self, absolute: bool) -> Self {
            self.quad = self.quad.absolute(absolute);
            self
        }

        /// Enables or disables absolute positioning.
        pub fn set_absolute(&mut self, absolute: bool) {
            self.quad = self.quad.absolute(absolute);
        }

        /// Sets the opacity.
        pub fn opacity(mut self, opacity: f32) -> Self {
            self.quad = self.quad.opacity(opacity);
            self
        }

        /// Sets the opacity.
        pub fn set_opacity(&mut self, opacity: f32) {
            self.quad = self.quad.opacity(opacity);
        }

        /// Draws the circle.
        pub fn draw(&self) {
            // Calculate destination coordinates.
            let mut dx = self.quad.x;
            let mut dy = self.quad.y;

            // If absolute positioning is enabled, adjust coordinates relative to the camera.
            if self.quad.absolute {
                let (cx, cy) = crate::canvas::camera::xy(); // Retrieve camera coordinates.
                let (w, h) = crate::canvas::resolution(); // Get canvas dimensions.
                dx += cx as i32 - (w as i32 / 2); // Center the sprite horizontally.
                dy += cy as i32 - (h as i32 / 2); // Center the sprite vertically.
            }

            // Apply opacity to the sprite's primary and background colors.
            let color = utils::color::apply_opacity(self.quad.color, self.quad.opacity);
            let border_color =
                utils::color::apply_opacity(self.quad.border_color, self.quad.opacity);

            // Draw the circle using the utility function.
            utils::rect::draw(
                color,                   // Fill color.
                dx,                      // x-coordinate.
                dy,                      // y-coordinate.
                self.quad.w,             // Width.
                self.quad.h,             // Height.
                self.quad.border_radius, // Border radius.
                self.quad.border_size,   // Border thickness.
                border_color,            // Border color.
                self.quad.origin_x,      // X rotation origin
                self.quad.origin_y,      // Y rotation origin
                self.quad.rotation_deg,  // Rotation in degrees.
            );
        }
    }
}

//------------------------------------------------------------------------------
// LINE
//------------------------------------------------------------------------------
pub mod path {
    use super::*;
    use num_traits::NumCast;

    /// A builder-style line type.
    #[derive(Debug, Clone, Copy)]
    pub struct Path {
        start: (i32, i32),
        end: (i32, i32),
        width: u32,
        rounded: bool,
        quad: Quad,
    }

    impl Path {
        /// Creates a new line with default properties.
        pub fn new() -> Self {
            Self {
                start: (0, 0),
                end: (0, 0),
                width: 1,
                rounded: false,
                quad: Quad::default(),
            }
        }

        /// Sets the line's position.
        pub fn start_position<X: NumCast, Y: NumCast>(mut self, x: X, y: Y) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.start.0);
            let y: i32 = NumCast::from(y).unwrap_or(self.start.1);
            self.start = (x, y);
            self
        }

        /// Sets the line's position.
        pub fn set_start_position<X: NumCast, Y: NumCast>(&mut self, x: X, y: Y) {
            let x: i32 = NumCast::from(x).unwrap_or(self.start.0);
            let y: i32 = NumCast::from(y).unwrap_or(self.start.1);
            self.start = (x, y);
        }

        /// Sets the line’s x position.
        pub fn start_position_x<X: NumCast>(mut self, x: X) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.start.0);
            self.start.0 = x;
            self
        }

        /// Sets the line’s x start_position.
        pub fn set_start_position_x<X: NumCast>(&mut self, x: X) {
            let x: i32 = NumCast::from(x).unwrap_or(self.start.0);
            self.start.0 = x;
        }

        /// Sets the line’s y start_position.
        pub fn start_position_y<Y: NumCast>(mut self, y: Y) -> Self {
            let y: i32 = NumCast::from(y).unwrap_or(self.start.1);
            self.start.1 = y;
            self
        }

        /// Sets the line’s y start_position.
        pub fn set_start_position_y<Y: NumCast>(&mut self, y: Y) {
            let y: i32 = NumCast::from(y).unwrap_or(self.start.1);
            self.start.1 = y;
        }

        /// Sets the line’s x and y start_position.
        pub fn start_position_xy<X: NumCast, Y: NumCast>(mut self, (x, y): (X, Y)) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.start.0);
            let y: i32 = NumCast::from(y).unwrap_or(self.start.1);
            self.start = (x, y);
            self
        }

        /// Sets the line’s x and y start_position.
        pub fn set_start_position_xy<X: NumCast, Y: NumCast>(&mut self, (x, y): (X, Y)) {
            let x: i32 = NumCast::from(x).unwrap_or(self.start.0);
            let y: i32 = NumCast::from(y).unwrap_or(self.start.1);
            self.start = (x, y);
        }

        /// Sets the line's position.
        pub fn end_position<X: NumCast, Y: NumCast>(mut self, x: X, y: Y) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.end.0);
            let y: i32 = NumCast::from(y).unwrap_or(self.end.1);
            self.end = (x, y);
            self
        }

        /// Sets the line's position.
        pub fn set_end_position<X: NumCast, Y: NumCast>(&mut self, x: X, y: Y) {
            let x: i32 = NumCast::from(x).unwrap_or(self.end.0);
            let y: i32 = NumCast::from(y).unwrap_or(self.end.1);
            self.end = (x, y);
        }

        /// Sets the line’s x position.
        pub fn end_position_x<X: NumCast>(mut self, x: X) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.end.0);
            self.end.0 = x;
            self
        }

        /// Sets the line’s x end_position.
        pub fn set_end_position_x<X: NumCast>(&mut self, x: X) {
            let x: i32 = NumCast::from(x).unwrap_or(self.end.0);
            self.end.0 = x;
        }

        /// Sets the line’s y end_position.
        pub fn end_position_y<Y: NumCast>(mut self, y: Y) -> Self {
            let y: i32 = NumCast::from(y).unwrap_or(self.end.1);
            self.end.1 = y;
            self
        }

        /// Sets the line’s y end_position.
        pub fn set_end_position_y<Y: NumCast>(&mut self, y: Y) {
            let y: i32 = NumCast::from(y).unwrap_or(self.end.1);
            self.end.1 = y;
        }

        /// Sets the line’s x and y end_position.
        pub fn end_position_xy<X: NumCast, Y: NumCast>(mut self, (x, y): (X, Y)) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.end.0);
            let y: i32 = NumCast::from(y).unwrap_or(self.end.1);
            self.end = (x, y);
            self
        }

        /// Sets the line’s x and y end_position.
        pub fn set_end_position_xy<X: NumCast, Y: NumCast>(&mut self, (x, y): (X, Y)) {
            let x: i32 = NumCast::from(x).unwrap_or(self.end.0);
            let y: i32 = NumCast::from(y).unwrap_or(self.end.1);
            self.end = (x, y);
        }

        /// Sets the line's size.
        pub fn size<W: NumCast>(mut self, w: W) -> Self {
            let w: u32 = NumCast::from(w).unwrap_or(self.width);
            self.width = w;
            self
        }

        /// Sets the line's size.
        pub fn set_size<W: NumCast>(&mut self, w: W) {
            let w: u32 = NumCast::from(w).unwrap_or(self.width);
            self.width = w;
        }

        /// Sets the line's width.
        pub fn width<W: NumCast>(mut self, w: W) -> Self {
            let w: u32 = NumCast::from(w).unwrap_or(self.width);
            self.width = w;
            self
        }

        /// Sets the line's width.
        pub fn set_width<W: NumCast>(&mut self, w: W) {
            let w: u32 = NumCast::from(w).unwrap_or(self.width);
            self.width = w;
        }

        /// Translates the line’s position by the given delta.
        pub fn offset<DX: NumCast, DY: NumCast>(mut self, dx: DX, dy: DY) -> Self {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.quad = self.quad.offset(dx, dy);
            self
        }

        /// Translates the line's position by the given delta.
        pub fn set_offset<DX: NumCast, DY: NumCast>(&mut self, dx: DX, dy: DY) {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.quad = self.quad.offset(dx, dy);
        }

        /// Translates the line's x position by the given delta.
        pub fn offset_x<DX: NumCast>(mut self, dx: DX) -> Self {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            self.quad = self.quad.offset(dx, 0);
            self
        }

        /// Translates the line's x position by the given delta.
        pub fn set_offset_x<DX: NumCast>(&mut self, dx: DX) {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            self.quad = self.quad.offset(dx, 0);
        }

        /// Translates the line's position by the given delta.
        pub fn offset_y<DY: NumCast>(mut self, dy: DY) -> Self {
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.quad = self.quad.offset(0, dy);
            self
        }

        /// Sets the line's primary fill color.
        pub fn color(mut self, color: u32) -> Self {
            self.quad = self.quad.color(color);
            self
        }

        /// Sets the line's primary fill color.
        pub fn set_color(&mut self, color: u32) {
            self.quad = self.quad.color(color);
        }

        /// Sets the border radius.
        pub fn border_radius<R: NumCast>(mut self, radius: R) -> Self {
            let radius: u32 = NumCast::from(radius).unwrap_or(0);
            self.quad = self.quad.border_radius(radius);
            self
        }

        /// Sets the border radius.
        pub fn set_border_radius<R: NumCast>(&mut self, radius: R) {
            let radius: u32 = NumCast::from(radius).unwrap_or(0);
            self.quad = self.quad.border_radius(radius);
        }

        /// Sets the corners of the line to be rounded.
        pub fn rounded(mut self, rounded: bool) -> Self {
            self.rounded = rounded;
            self
        }

        /// Sets the corners of the line to be rounded.
        pub fn set_rounded(&mut self, rounded: bool) {
            self.rounded = rounded;
        }

        /// Sets the origin for transformations.
        pub fn origin<X: NumCast, Y: NumCast>(mut self, origin_x: X, origin_y: Y) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
            self
        }

        /// Sets the origin for transformations.
        pub fn set_origin<X: NumCast, Y: NumCast>(&mut self, origin_x: X, origin_y: Y) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
        }

        /// Sets the x origin for transformations.
        pub fn origin_x<X: NumCast>(mut self, origin_x: X) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            self.quad = self.quad.origin(origin_x, self.quad.origin_y);
            self
        }

        /// Sets the x origin for transformations.
        pub fn set_origin_x<X: NumCast>(&mut self, origin_x: X) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            self.quad = self.quad.origin(origin_x, self.quad.origin_y);
        }

        /// Sets the y origin for transformations.
        pub fn origin_y<Y: NumCast>(mut self, origin_y: Y) -> Self {
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(self.quad.origin_x, origin_y);
            self
        }

        /// Sets the y origin for transformations.
        pub fn set_origin_y<Y: NumCast>(&mut self, origin_y: Y) {
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(self.quad.origin_x, origin_y);
        }

        /// Sets the x and y origin for transformations.
        pub fn origin_xy<X: NumCast, Y: NumCast>(mut self, (origin_x, origin_y): (X, Y)) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
            self
        }

        /// Sets the x and y origin for transformations.
        pub fn set_origin_xy<X: NumCast, Y: NumCast>(&mut self, (origin_x, origin_y): (X, Y)) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
        }

        /// Sets the rotation angle by degrees.
        pub fn rotation_deg<A: NumCast>(mut self, degrees: A) -> Self {
            let degrees: i32 = NumCast::from(degrees).unwrap_or(0);
            self.quad = self.quad.rotation(degrees);
            self
        }

        /// Sets the rotation angle by degrees.
        pub fn set_rotation_deg<A: NumCast>(&mut self, degrees: A) {
            let degrees: i32 = NumCast::from(degrees).unwrap_or(0);
            self.quad = self.quad.rotation(degrees);
        }

        /// Sets the rotation angle by radians.
        pub fn rotation_rad<R: NumCast>(mut self, radians: R) -> Self {
            let radian: f32 = NumCast::from(radians).unwrap_or(0.);
            let angle = (radian * 180. / std::f32::consts::PI).round() as i32;
            self.quad = self.quad.rotation(angle);
            self
        }

        /// Sets the rotation angle by radians.
        pub fn set_rotation_rad<R: NumCast>(&mut self, radians: R) {
            let radian: f32 = NumCast::from(radians).unwrap_or(0.);
            let angle = (radian * 180. / std::f32::consts::PI).round() as i32;
            self.quad = self.quad.rotation(angle);
        }

        /// Enables or disables absolute positioning.
        pub fn absolute(mut self, absolute: bool) -> Self {
            self.quad = self.quad.absolute(absolute);
            self
        }

        /// Enables or disables absolute positioning.
        pub fn set_absolute(&mut self, absolute: bool) {
            self.quad = self.quad.absolute(absolute);
        }

        /// Sets the opacity.
        pub fn opacity(mut self, opacity: f32) -> Self {
            self.quad = self.quad.opacity(opacity);
            self
        }

        /// Sets the opacity.
        pub fn set_opacity(&mut self, opacity: f32) {
            self.quad = self.quad.opacity(opacity);
        }

        /// Draws the line.
        pub fn draw(&self) {
            // Convert width to float.
            let width = self.width as f64;

            // Convert start and end coordinates to floats.
            let start = (self.start.0 as f64, self.start.1 as f64);
            let end = (self.end.0 as f64, self.end.1 as f64);

            // Compute the differences between the start and end points.
            let delta_x = end.0 - start.0;
            let delta_y = end.1 - start.1;

            // Calculate the Euclidean distance between the two points.
            let distance_f = (delta_x * delta_x + delta_y * delta_y).sqrt();
            // If the distance is negligible, there's nothing to draw.
            if distance_f < 1.0 {
                return;
            }
            let distance = distance_f;

            // Calculate the angle in degrees.
            let mut angle =
                (delta_y.atan2(delta_x) * (180.0 / std::f64::consts::PI)).round() as i32;
            angle += self.quad.rotation_deg;

            // Compute the midpoint between start and end as floats.
            let mid_x = (start.0 + end.0) / 2.0;
            let mid_y = (start.1 + end.1) / 2.0;

            // Determine the drawing rectangle's top-left corner.
            // The rectangle's width equals the line length (distance) and its height equals the line thickness.
            let dx_f = mid_x - (distance_f / 2.0);
            let dy_f = mid_y - (width / 2.0);
            let mut dx = dx_f.floor() as i32;
            let mut dy = dy_f.floor() as i32;

            // For a pill-shaped (rounded) line, set the border radius to half the line's thickness.
            let border_radius = if self.rounded { width / 2. } else { 0. }.round() as u32;

            // If absolute positioning is enabled, adjust coordinates relative to the camera.
            if self.quad.absolute {
                let (cx, cy) = crate::canvas::camera::xy(); // Retrieve camera coordinates.
                let (w, h) = crate::canvas::resolution(); // Get canvas dimensions.
                dx += cx as i32 - (w as i32 / 2); // Center the sprite horizontally.
                dy += cy as i32 - (h as i32 / 2); // Center the sprite vertically.
            }

            // Shift the line right by one pixel when there's no x delta.
            if delta_x == 0. {
                dx += 1;
            }

            // Shift the line down by one pixel when there's no y delta.
            if delta_y == 0. {
                dy += 1;
            }

            // Apply the quad's opacity to its color.
            let color = utils::color::apply_opacity(self.quad.color, self.quad.opacity);

            // Draw the line as a rotated rectangle:
            // - The rectangle's width equals the line length.
            // - Its height equals the line's thickness.
            // - It's rotated by the calculated angle.
            utils::rect::draw(
                color,              // Fill color.
                dx,                 // x-coordinate.
                dy,                 // y-coordinate.
                distance as u32,    // Width (line length).
                self.width,         // Height (line thickness).
                border_radius,      // Border radius.
                0,                  // Border thickness.
                0x00000000,         // Border color (transparent).
                self.quad.origin_x, // X rotation origin.
                self.quad.origin_y, // Y rotation origin.
                angle,              // Rotation angle in degrees.
            );
        }
    }
}

//------------------------------------------------------------------------------
// TEXT
//------------------------------------------------------------------------------
pub mod text {
    use super::*;
    use num_traits::NumCast;

    /// A builder-style rectangle type.
    #[derive(Debug, Clone, Copy)]
    pub struct Text<'a> {
        text: &'a str,
        font: &'a str,
        scale: f32,
        quad: Quad,
    }

    impl<'a> Text<'a> {
        /// Creates a new rectangle with default properties.
        pub fn new(text: &'a str) -> Self {
            Self {
                text,
                font: "",
                scale: 1.0,
                quad: Quad::default(),
            }
        }

        pub fn font(mut self, name: &'a str) -> Self {
            self.font = name;
            self
        }

        pub fn scale(mut self, scale: f32) -> Self {
            self.scale = scale;
            self
        }

        /// Sets the rectangle's position.
        pub fn position<X: NumCast, Y: NumCast>(mut self, x: X, y: Y) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(x, y);
            self
        }

        /// Sets the rectangle's position.
        pub fn set_position<X: NumCast, Y: NumCast>(&mut self, x: X, y: Y) {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(x, y);
        }

        /// Sets the rectangle’s x position.
        pub fn position_x<X: NumCast>(mut self, x: X) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            self.quad = self.quad.position(x, self.quad.y);
            self
        }

        /// Sets the rectangle’s x position.
        pub fn set_position_x<X: NumCast>(&mut self, x: X) {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            self.quad = self.quad.position(x, self.quad.y);
        }

        /// Sets the rectangle’s y position.
        pub fn position_y<Y: NumCast>(mut self, y: Y) -> Self {
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(self.quad.x, y);
            self
        }

        /// Sets the rectangle’s y position.
        pub fn set_position_y<Y: NumCast>(&mut self, y: Y) {
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(self.quad.x, y);
        }

        /// Sets the rectangle’s x and y position.
        pub fn position_xy<X: NumCast, Y: NumCast>(mut self, (x, y): (X, Y)) -> Self {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(x, y);
            self
        }

        /// Sets the rectangle’s x and y position.
        pub fn set_position_xy<X: NumCast, Y: NumCast>(&mut self, (x, y): (X, Y)) {
            let x: i32 = NumCast::from(x).unwrap_or(self.quad.x);
            let y: i32 = NumCast::from(y).unwrap_or(self.quad.y);
            self.quad = self.quad.position(x, y);
        }

        /// Translates the rectangle’s position by the given delta.
        pub fn offset<DX: NumCast, DY: NumCast>(mut self, dx: DX, dy: DY) -> Self {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.quad = self.quad.offset(dx, dy);
            self
        }

        /// Translates the rectangle's position by the given delta.
        pub fn set_offset<DX: NumCast, DY: NumCast>(&mut self, dx: DX, dy: DY) {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.quad = self.quad.offset(dx, dy);
        }

        /// Translates the rectangle's x position by the given delta.
        pub fn offset_x<DX: NumCast>(mut self, dx: DX) -> Self {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            self.quad = self.quad.offset(dx, 0);
            self
        }

        /// Translates the rectangle's x position by the given delta.
        pub fn set_offset_x<DX: NumCast>(&mut self, dx: DX) {
            let dx: i32 = NumCast::from(dx).unwrap_or(0);
            self.quad = self.quad.offset(dx, 0);
        }

        /// Translates the rectangle's position by the given delta.
        pub fn offset_y<DY: NumCast>(mut self, dy: DY) -> Self {
            let dy: i32 = NumCast::from(dy).unwrap_or(0);
            self.quad = self.quad.offset(0, dy);
            self
        }

        /// Sets the rectangle's primary fill color.
        pub fn color(mut self, color: u32) -> Self {
            self.quad = self.quad.color(color);
            self
        }

        /// Sets the rectangle's primary fill color.
        pub fn set_color(&mut self, color: u32) {
            self.quad = self.quad.color(color);
        }

        /// Sets the border radius.
        pub fn border_radius<R: NumCast>(mut self, radius: R) -> Self {
            let radius: u32 = NumCast::from(radius).unwrap_or(0);
            self.quad = self.quad.border_radius(radius);
            self
        }

        /// Sets the border radius.
        pub fn set_border_radius<R: NumCast>(&mut self, radius: R) {
            let radius: u32 = NumCast::from(radius).unwrap_or(0);
            self.quad = self.quad.border_radius(radius);
        }

        /// Sets the border size.
        pub fn border_size(mut self, size: u32) -> Self {
            self.quad = self.quad.border_size(size);
            self
        }

        /// Sets the border size.
        pub fn set_border_size(&mut self, bg: u32) {
            self.quad = self.quad.border_size(bg);
        }

        /// Sets the border color.
        pub fn border_color(mut self, color: u32) -> Self {
            self.quad = self.quad.border_color(color);
            self
        }

        /// Sets the border color.
        pub fn set_border_color(&mut self, bg: u32) {
            self.quad = self.quad.border_color(bg);
        }

        /// Sets the origin for transformations.
        pub fn origin<X: NumCast, Y: NumCast>(mut self, origin_x: X, origin_y: Y) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
            self
        }

        /// Sets the origin for transformations.
        pub fn set_origin<X: NumCast, Y: NumCast>(&mut self, origin_x: X, origin_y: Y) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
        }

        /// Sets the x origin for transformations.
        pub fn origin_x<X: NumCast>(mut self, origin_x: X) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            self.quad = self.quad.origin(origin_x, self.quad.origin_y);
            self
        }

        /// Sets the x origin for transformations.
        pub fn set_origin_x<X: NumCast>(&mut self, origin_x: X) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            self.quad = self.quad.origin(origin_x, self.quad.origin_y);
        }

        /// Sets the y origin for transformations.
        pub fn origin_y<Y: NumCast>(mut self, origin_y: Y) -> Self {
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(self.quad.origin_x, origin_y);
            self
        }

        /// Sets the y origin for transformations.
        pub fn set_origin_y<Y: NumCast>(&mut self, origin_y: Y) {
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(self.quad.origin_x, origin_y);
        }

        /// Sets the x and y origin for transformations.
        pub fn origin_xy<X: NumCast, Y: NumCast>(mut self, (origin_x, origin_y): (X, Y)) -> Self {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
            self
        }

        /// Sets the x and y origin for transformations.
        pub fn set_origin_xy<X: NumCast, Y: NumCast>(&mut self, (origin_x, origin_y): (X, Y)) {
            let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.quad.origin_x);
            let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.quad.origin_y);
            self.quad = self.quad.origin(origin_x, origin_y);
        }

        /// Sets the rotation angle by degrees.
        pub fn rotation_deg<A: NumCast>(mut self, degrees: A) -> Self {
            let degrees: i32 = NumCast::from(degrees).unwrap_or(0);
            self.quad = self.quad.rotation(degrees);
            self
        }

        /// Sets the rotation angle by degrees.
        pub fn set_rotation_deg<A: NumCast>(&mut self, degrees: A) {
            let degrees: i32 = NumCast::from(degrees).unwrap_or(0);
            self.quad = self.quad.rotation(degrees);
        }

        /// Sets the rotation angle by radians.
        pub fn rotation_rad<R: NumCast>(mut self, radians: R) -> Self {
            let radian: f32 = NumCast::from(radians).unwrap_or(0.);
            let angle = (radian * 180. / std::f32::consts::PI).round() as i32;
            self.quad = self.quad.rotation(angle);
            self
        }

        /// Sets the rotation angle by radians.
        pub fn set_rotation_rad<R: NumCast>(&mut self, radians: R) {
            let radian: f32 = NumCast::from(radians).unwrap_or(0.);
            let angle = (radian * 180. / std::f32::consts::PI).round() as i32;
            self.quad = self.quad.rotation(angle);
        }

        /// Enables or disables absolute positioning.
        pub fn absolute(mut self, absolute: bool) -> Self {
            self.quad = self.quad.absolute(absolute);
            self
        }

        /// Enables or disables absolute positioning.
        pub fn set_absolute(&mut self, absolute: bool) {
            self.quad = self.quad.absolute(absolute);
        }

        /// Sets the opacity.
        pub fn opacity(mut self, opacity: f32) -> Self {
            self.quad = self.quad.opacity(opacity);
            self
        }

        /// Sets the opacity.
        pub fn set_opacity(&mut self, opacity: f32) {
            self.quad = self.quad.opacity(opacity);
        }

        /// Draws the rectangle.
        pub fn draw(&self) {
            // Calculate destination coordinates.
            let mut dx = self.quad.x;
            let mut dy = self.quad.y;

            // If absolute positioning is enabled, adjust coordinates relative to the camera.
            if self.quad.absolute {
                let (cx, cy) = crate::canvas::camera::xy(); // Retrieve camera coordinates.
                let (w, h) = crate::canvas::resolution(); // Get canvas dimensions.
                dx += cx as i32 - (w as i32 / 2); // Center the sprite horizontally.
                dy += cy as i32 - (h as i32 / 2); // Center the sprite vertically.
            }

            // Apply opacity to the sprite's primary and background colors.
            let color = utils::color::apply_opacity(self.quad.color, self.quad.opacity);

            // Convert degrees to radians
            let rotation = self.quad.rotation_deg as f32 * std::f32::consts::PI / 180.0;

            // Draw the rectangle using the utility function.
            utils::text::draw(
                self.font,  // Font name.
                self.text,  // Text to draw
                dx,         // x-coordinate.
                dy,         // y-coordinate.
                color,      // Fill color.
                self.scale, // Font scale.
                rotation,   // Rotation in degrees.
            );
        }
    }
}

//------------------------------------------------------------------------------
// UTILS
//------------------------------------------------------------------------------
pub mod utils {
    use super::*;

    pub mod sprite {
        use crate::ffi;
        use crate::sys::tick;
        use borsh::{BorshDeserialize, BorshSerialize};
        use std::{collections::BTreeMap, ops::Div};

        static mut TURBO_SPRITE_DATA_NONCE: u64 = 0;
        static mut TURBO_SPRITE_DATA: BTreeMap<String, SpriteSourceData> = BTreeMap::new();

        #[derive(Debug, Clone, Default, PartialEq, BorshDeserialize, BorshSerialize)]
        pub struct SpriteSourceData {
            pub x: u32,
            pub y: u32,
            pub width: u32,
            pub height: u32,
            pub animation_loop_count: u32,
            pub animation_direction: SpriteAnimationDirection,
            pub animation_frames: Vec<SpriteAnimationFrame>,
        }

        #[derive(Debug, Clone, Copy, Default, PartialEq, BorshDeserialize, BorshSerialize)]
        pub enum SpriteAnimationDirection {
            #[default]
            Forward,
            Reverse,
            PingPong,
            PingPongReverse,
        }

        #[derive(Debug, Clone, Copy, Default, PartialEq, BorshDeserialize, BorshSerialize)]
        pub struct SpriteAnimationFrame {
            pub duration: f32,
        }

        pub fn get_source_data_nonce() -> u64 {
            unsafe { TURBO_SPRITE_DATA_NONCE }
        }

        pub fn get_source_data(name: &str) -> Option<SpriteSourceData> {
            #[allow(static_mut_refs)]
            unsafe {
                // Check latest sprite data nonce
                let nonce = ffi::canvas::get_sprite_data_nonce_v1();

                // If nonce has been updated, refresh data
                if TURBO_SPRITE_DATA_NONCE < nonce {
                    // Get latest sprite data
                    let mut data = vec![0; 1024 * 1024]; // up to 100kb of sprite data
                    let data_ptr = data.as_mut_ptr();
                    let mut len = data.len() as u32;
                    let len_ptr = &mut len;
                    ffi::canvas::get_sprite_data_v1(data_ptr, len_ptr);

                    // Deserialize sprite data
                    match <BTreeMap<String, SpriteSourceData>>::deserialize(&mut &data[..]) {
                        // Update statics
                        Ok(data) => {
                            TURBO_SPRITE_DATA_NONCE = nonce;
                            TURBO_SPRITE_DATA = data;
                        }
                        // Log the error
                        Err(err) => {
                            crate::println!("Sprite data deserialization failed: {err:?}");
                        }
                    }
                }

                // Return the sprite data
                return TURBO_SPRITE_DATA.get(name).cloned();
            }
        }

        pub fn get_frame_index(sprite_data: &SpriteSourceData, speed: f32) -> usize {
            let elapsed_time = (tick() as f32 / 60.0) * 1000.0;
            let total_duration = sprite_data
                .animation_frames
                .iter()
                .map(|f| f.duration)
                .sum::<f32>()
                .div(speed);
            let animation_time = elapsed_time % total_duration;
            let mut accumulated_time = 0.0;
            let mut index = 0;
            for (i, frame) in sprite_data.animation_frames.iter().enumerate() {
                accumulated_time += frame.duration.div(speed);
                if animation_time < accumulated_time {
                    index = i;
                    break;
                }
            }
            index
        }

        pub fn draw(
            dx: i32,
            dy: i32,
            dw: u32,
            dh: u32,
            sx: u32,
            sy: u32,
            sw: i32,
            sh: i32,
            texture_x: i32,
            texture_y: i32,
            color: u32,
            background_color: u32,
            border_radius: u32,
            origin_x: i32,
            origin_y: i32,
            rotatation_deg: i32,
            flags: u32,
        ) {
            let dest_xy = ((dx as u64) << 32) | (dy as u64 & 0xffffffff);
            let dest_wh = ((dw as u64) << 32) | (dh as u32 as u64);
            let sprite_xy = ((sx as u64) << 32) | (sy as u64);
            let sprite_xy_offset = ((texture_x as u64) << 32) | (texture_y as u32 as u64);
            let sprite_wh = ((sw as u64) << 32) | (sh as u32 as u64);
            let origin_xy = ((origin_x as u64) << 32) | (origin_y as u64 & 0xffffffff);
            let fill_ab = (background_color as u64) << 32 | (color as u64 & 0xffffffff);
            ffi::canvas::draw_quad2(
                dest_xy,
                dest_wh,
                sprite_xy,
                sprite_wh,
                sprite_xy_offset,
                fill_ab,
                border_radius,
                0,
                0,
                origin_xy,
                rotatation_deg,
                flags,
            )
        }
    }

    pub mod rect {
        use crate::ffi;

        pub fn draw(
            color: u32,
            dx: i32,
            dy: i32,
            dw: u32,
            dh: u32,
            border_radius: u32,
            border_size: u32,
            border_color: u32,
            origin_x: i32,
            origin_y: i32,
            rotation_deg: i32,
        ) {
            let dest_xy = ((dx as u64) << 32) | (dy as u32 as u64);
            let dest_wh = ((dw as u64) << 32) | (dh as u32 as u64);
            let origin_xy = ((origin_x as u64) << 32) | (origin_y as u64 & 0xffffffff);
            let fill_ab = (color as u64) << 32;
            ffi::canvas::draw_quad_v1(
                dest_xy,
                dest_wh,
                0,
                0,
                fill_ab,
                border_radius,
                border_size,
                border_color,
                origin_xy,
                rotation_deg,
            )
        }
    }

    pub mod text {
        use crate::ffi;

        pub fn draw(
            font_name: &str,
            text: &str,
            x: i32,
            y: i32,
            color: u32,
            scale: f32,
            rotation: f32,
        ) {
            let font_name_ptr = font_name.as_ptr();
            let font_name_len = font_name.len() as u32;
            let text_ptr = text.as_ptr();
            let text_len = text.len() as u32;
            ffi::canvas::text2(
                x,
                y,
                color,
                scale,
                rotation,
                font_name_ptr,
                font_name_len,
                text_ptr,
                text_len,
            )
        }
    }

    pub mod color {
        #[allow(arithmetic_overflow)]
        pub fn apply_opacity(color: u32, opacity: f32) -> u32 {
            if opacity == 1.0 {
                return color;
            }
            // Apply gamma correction
            let gamma = 2.2;
            let linear_opacity = opacity.powf(1.0 / gamma);

            // Calculate the alpha value
            let alpha = (255.0 * linear_opacity) as u32;

            // Combine the alpha with the color
            alpha << 32 | (color & 0xffffff00)
        }
    }

    pub mod hash {
        use std::collections::BTreeMap;

        static mut TURBO_FNV1A_REVERSE_LOOKUP: BTreeMap<u64, Vec<u8>> = BTreeMap::new();
        pub fn fnv1a(data: &[u8]) -> u64 {
            const FNV_OFFSET: u64 = 0xcbf29ce484222325;
            const FNV_PRIME: u64 = 0x100000001b3;
            let mut hash = FNV_OFFSET;
            for &byte in data {
                hash ^= byte as u64;
                hash = hash.wrapping_mul(FNV_PRIME);
            }
            #[allow(static_mut_refs)]
            unsafe {
                if !TURBO_FNV1A_REVERSE_LOOKUP.contains_key(&hash) {
                    TURBO_FNV1A_REVERSE_LOOKUP.insert(hash, data.to_vec());
                }
            }
            hash
        }
        pub fn lookup_fnv1a<'a>(hash: u64) -> Option<&'a [u8]> {
            #[allow(static_mut_refs)]
            unsafe {
                TURBO_FNV1A_REVERSE_LOOKUP.get(&hash).map(|v| &**v)
            }
        }
    }
}

//------------------------------------------------------------------------------
// MACROS
//------------------------------------------------------------------------------
pub use macros::*;
mod macros {

    //--------------------------------------------------------------------------
    // sprite!
    //--------------------------------------------------------------------------

    #[doc(inline)]
    pub use crate::__sprite__ as sprite;

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __sprite__ {
        (animation_key = $anim:expr) => {{ $crate::canvas::animation($anim).sprite().draw() }};
        (animation_key = $anim:expr, $( $key:ident = $val:expr ),* $(,)*) => {{
            // 1. Make a sprite with the given name
            let mut sprite = $crate::canvas::animation($anim).sprite();
            // 2. For each key-value pair, call the corresponding method on the sprite.
            $(sprite = __sprite__!(@set sprite, $key, $val);)*
            // 3. Draw it!
            sprite.draw();
        }};
        (animation = $anim:expr) => {{ $anim.sprite().draw() }};
        (animation = $anim:expr, $( $key:ident = $val:expr ),* $(,)*) => {{
            // 1. Make a sprite with the given name
            let mut sprite = $anim.sprite();
            // 2. For each key-value pair, call the corresponding method on the sprite.
            $(sprite = __sprite__!(@set sprite, $key, $val);)*
            // 3. Draw it!
            sprite.draw();
        }};
        ($name:expr) => {{
            let name = $name;
            $crate::canvas::__sprite__!(name)
        }};
        ($name:expr, $( $key:ident = $val:expr ),* $(,)*) => {{
            // 1. Make a sprite with the given name
            let name = $name;
            let mut sprite = $crate::canvas::sprite(name);
            // 2. For each key-value pair, call the corresponding method on the sprite.
            $(sprite = __sprite__!(@set sprite, $key, $val);)*
            // 3. Draw it!
            sprite.draw();
        }};
        (@set $sprite:ident, x, $val:expr) => { $sprite.position_x($val) };
        (@set $sprite:ident, y, $val:expr) => { $sprite.position_y($val) };
        (@set $sprite:ident, xy, $val:expr) => { $sprite.position_xy($val) };
        (@set $sprite:ident, position, $val:expr) => { $sprite.position_xy($val) };
        (@set $sprite:ident, tx, $val:expr) => { $sprite.tex_position_x($val) };
        (@set $sprite:ident, ty, $val:expr) => { $sprite.tex_position_y($val) };
        (@set $sprite:ident, w, $val:expr) => { $sprite.width($val) };
        (@set $sprite:ident, h, $val:expr) => { $sprite.height($val) };
        (@set $sprite:ident, wh, $val:expr) => { $sprite.size_wh($val) };
        (@set $sprite:ident, size, $val:expr) => { $sprite.size_wh($val) };
        (@set $sprite:ident, origin, $val:expr) => { $sprite.origin_xy($val) };
        (@set $sprite:ident, rotation, $val:expr) => { $sprite.rotation_deg($val) };
        (@set $sprite:ident, $key:ident, $val:expr) => { $sprite.$key($val) };
    }

    //--------------------------------------------------------------------------
    // nine_slice!
    //--------------------------------------------------------------------------

    #[doc(inline)]
    pub use crate::__nine_slice__ as nine_slice;

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __nine_slice__ {
        ($name:expr, margins = $margins:expr, $( $key:ident = $val:expr ),* $(,)*) => {{
            // 1. Make a nine-slice with the given name
            let mut nine_slice = $crate::canvas::nine_slice($name, $margins);
            // 2. For each key-value pair, call the corresponding method on the nine_slice.
            $(nine_slice = __nine_slice__!(@set nine_slice, $key, $val);)*
            // 3. Draw it!
            nine_slice.draw();
        }};
        (@set $nine_slice:ident, x, $val:expr) => { $nine_slice.position_x($val) };
        (@set $nine_slice:ident, y, $val:expr) => { $nine_slice.position_y($val) };
        (@set $nine_slice:ident, xy, $val:expr) => { $nine_slice.position_xy($val) };
        (@set $nine_slice:ident, position, $val:expr) => { $nine_slice.position_xy($val) };
        (@set $nine_slice:ident, tx, $val:expr) => { $nine_slice.tex_position_x($val) };
        (@set $nine_slice:ident, ty, $val:expr) => { $nine_slice.tex_position_y($val) };
        (@set $nine_slice:ident, w, $val:expr) => { $nine_slice.width($val) };
        (@set $nine_slice:ident, h, $val:expr) => { $nine_slice.height($val) };
        (@set $nine_slice:ident, wh, $val:expr) => { $nine_slice.size_wh($val) };
        (@set $nine_slice:ident, size, $val:expr) => { $nine_slice.size_wh($val) };
        (@set $nine_slice:ident, origin, $val:expr) => { $nine_slice.origin_xy($val) };
        (@set $nine_slice:ident, rotation, $val:expr) => { $nine_slice.rotation_deg($val) };
        (@set $nine_slice:ident, $key:ident, $val:expr) => { $nine_slice.$key($val) };
    }

    //--------------------------------------------------------------------------
    // rect!
    //--------------------------------------------------------------------------

    #[doc(inline)]
    pub use crate::__rect__ as rect;

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __rect__ {
        ($( $key:ident = $val:expr ),* $(,)*) => {{
            // 1. Make a rect
            let mut rect = $crate::canvas::rect::Rectangle::new();
            // 2. For each key-value pair, call the corresponding method on the rect.
            $(rect = __rect__!(@set rect, $key, $val);)*
            // 3. Draw it!
            rect.draw();
        }};
        (@set $rect:ident, x, $val:expr) => { $rect.position_x($val) };
        (@set $rect:ident, y, $val:expr) => { $rect.position_y($val) };
        (@set $rect:ident, xy, $val:expr) => { $rect.position_xy($val) };
        (@set $rect:ident, position, $val:expr) => { $rect.position_xy($val) };
        (@set $rect:ident, w, $val:expr) => { $rect.width($val) };
        (@set $rect:ident, h, $val:expr) => { $rect.height($val) };
        (@set $rect:ident, wh, $val:expr) => { $rect.size_wh($val) };
        (@set $rect:ident, size, $val:expr) => { $rect.size_wh($val) };
        (@set $rect:ident, origin, $val:expr) => { $rect.origin_xy($val) };
        (@set $rect:ident, rotation, $val:expr) => { $rect.rotation_deg($val) };
        (@set $rect:ident, $key:ident, $val:expr) => { $rect.$key($val) };
    }

    //--------------------------------------------------------------------------
    // ellipse!
    //--------------------------------------------------------------------------

    #[doc(inline)]
    pub use crate::__ellipse__ as ellipse;

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __ellipse__ {
        ($( $key:ident = $val:expr ),* $(,)*) => {{
            // 1. Make a rect
            let mut ellipse = $crate::canvas::ellipse::Ellipse::new();
            // 2. For each key-value pair, call the corresponding method on the ellipse.
            $(ellipse = __ellipse__!(@set ellipse, $key, $val);)*
            // 3. Draw it!
            ellipse.draw();
        }};
        (@set $ellipse:ident, x, $val:expr) => { $ellipse.position_x($val) };
        (@set $ellipse:ident, y, $val:expr) => { $ellipse.position_y($val) };
        (@set $ellipse:ident, xy, $val:expr) => { $ellipse.position_xy($val) };
        (@set $ellipse:ident, position, $val:expr) => { $ellipse.position_xy($val) };
        (@set $ellipse:ident, w, $val:expr) => { $ellipse.width($val) };
        (@set $ellipse:ident, h, $val:expr) => { $ellipse.height($val) };
        (@set $ellipse:ident, wh, $val:expr) => { $ellipse.size_wh($val) };
        (@set $ellipse:ident, size, $val:expr) => { $ellipse.size_wh($val) };
        (@set $ellipse:ident, origin, $val:expr) => { $ellipse.origin_xy($val) };
        (@set $ellipse:ident, rotation, $val:expr) => { $ellipse.rotation_deg($val) };
        (@set $ellipse:ident, $key:ident, $val:expr) => { $ellipse.$key($val) };
    }

    //--------------------------------------------------------------------------
    // circ!
    //--------------------------------------------------------------------------

    #[doc(inline)]
    pub use crate::__circ__ as circ;

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __circ__ {
        ($( $key:ident = $val:expr ),* $(,)*) => {{
            // 1. Make a circ
            let mut circ = $crate::canvas::circ::Circle::new();
            // 2. For each key-value pair, call the corresponding method on the circ.
            $(circ = __circ__!(@set circ, $key, $val);)*
            // 3. Draw it!
            circ.draw();
        }};
        (@set $circ:ident, x, $val:expr) => { $circ.position_x($val) };
        (@set $circ:ident, y, $val:expr) => { $circ.position_y($val) };
        (@set $circ:ident, xy, $val:expr) => { $circ.position_xy($val) };
        (@set $circ:ident, position, $val:expr) => { $circ.position_xy($val) };
        (@set $circ:ident, d, $val:expr) => { $circ.diameter($val) };
        (@set $circ:ident, origin, $val:expr) => { $circ.origin_xy($val) };
        (@set $circ:ident, $key:ident, $val:expr) => { $circ.$key($val) };
    }

    //--------------------------------------------------------------------------
    // line!
    //--------------------------------------------------------------------------

    #[doc(inline)]
    pub use crate::__line__ as line;

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __line__ {
        ($( $key:ident = $val:expr ),* $(,)*) => {{
            // 1. Make a line
            let mut line = $crate::canvas::path::Path::new();
            // 2. For each key-value pair, call the corresponding method on the line.
            $(line = __line__!(@set line, $key, $val);)*
            // 3. Draw it!
            line.draw();
        }};
        (@set $line:ident, start_x, $val:expr) => { $line.start_position_x($val) };
        (@set $line:ident, start_y, $val:expr) => { $line.start_position_y($val) };
        (@set $line:ident, start, $val:expr) => { $line.start_position_xy($val) };
        (@set $line:ident, end_x, $val:expr) => { $line.end_position_x($val) };
        (@set $line:ident, end_y, $val:expr) => { $line.end_position_y($val) };
        (@set $line:ident, end, $val:expr) => { $line.end_position_xy($val) };
        (@set $line:ident, w, $val:expr) => { $line };
        (@set $line:ident, origin, $val:expr) => { $line.origin_xy($val) };
        (@set $rect:ident, rotation, $val:expr) => { $rect.rotation_deg($val) };
        (@set $line:ident, $key:ident, $val:expr) => { $line.$key($val) };
    }

    //--------------------------------------------------------------------------
    // text!
    //--------------------------------------------------------------------------

    #[doc(inline)]
    pub use crate::__text__ as text;

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __text__ {
        // Interpolation + key-value pairs.
        ($string:expr, $( $arg:expr ),* ; $( $key:ident = $val:expr ),* $(,)*) => {{
            // 1. Make a text
            let string = format!($string, $($arg),*);
            let mut text = $crate::canvas::text::Text::new(&string);
            // 2. For each key-value pair, call the corresponding method on the text.
            $(text = __text__!(@set text, $key, $val);)*
            // 3. Draw it!
            text.draw();
        }};
        // No interpolation + key-value pairs.
        ($string:expr, $( $key:ident = $val:expr ),* $(,)*) => {{
            // 1. Make a text
            let mut text = $crate::canvas::text::Text::new($string);
            // 2. For each key-value pair, call the corresponding method on the text.
            $(text = __text__!(@set text, $key, $val);)*
            // 3. Draw it!
            text.draw();
        }};
        // Interpolation + no key-value pairs.
        ($string:expr, $( $arg:expr ),*$(,)*) => {{
            // 1. Make a text
            let string = format!($string, $($arg),*);
            let mut text = $crate::canvas::text::Text::new(&string);
            // 2. Draw it!
            text.draw();
        }};
        // No interpolation + no key-value pairs.
        ($string:expr) => {{
            // 1. Make a text
            let mut text = $crate::canvas::text::Text::new($string);
            // 2. Draw it!
            text.draw();
        }};
        (@set $text:ident, x, $val:expr) => { $text.position_x($val) };
        (@set $text:ident, y, $val:expr) => { $text.position_y($val) };
        (@set $text:ident, xy, $val:expr) => { $text.position_xy($val) };
        (@set $text:ident, position, $val:expr) => { $text.position_xy($val) };
        (@set $text:ident, rotation, $val:expr) => { $text.rotation_deg($val) };
        (@set $text:ident, $key:ident, $val:expr) => { $text.$key($val) };
    }
}
