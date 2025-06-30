use super::{sprite, utils};
use sprite::Sprite;
use std::collections::BTreeMap;
use utils::sprite::{SpriteAnimationDirection, SpriteAnimationFrame};

/// Retrieves (or creates if not present) a sprite animation associated with the given key.
/// This ensures that an animation exists in the global animation map.
pub fn get(key: &str) -> &mut SpriteAnimation {
    SpriteAnimation::get_or_insert(key)
}

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
            started_at: turbo_genesis_ffi::sys::tick() as usize,
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
            self.paused_at = Some(turbo_genesis_ffi::sys::tick() as usize);
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
        self.started_at = turbo_genesis_ffi::sys::tick() as usize;
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
        let now = turbo_genesis_ffi::sys::tick() as usize;

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
            let t = turbo_genesis_ffi::sys::tick() as usize; // Get the current tick count.
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
    /// After calling this method, the following animation data will be modified:
    /// - sprite name
    /// - repeats (loop count)
    /// - animation direction
    pub fn use_sprite(&mut self, name: &str) {
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
