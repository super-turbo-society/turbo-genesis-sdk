use crate::{bounds::Bounds, sys};
use borsh::{BorshDeserialize, BorshSerialize};
use std::ops::Add;

/// Standard easing function types used to modify interpolation curves.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum Easing {
    #[default]
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInBack,
}

#[allow(unused)]
impl Easing {
    /// All easing variants (for menus, debug UIs, etc).
    pub const ALL: [Self; 23] = [
        Self::Linear,
        Self::EaseInQuad,
        Self::EaseOutQuad,
        Self::EaseInOutQuad,
        Self::EaseInCubic,
        Self::EaseOutCubic,
        Self::EaseInOutCubic,
        Self::EaseInQuart,
        Self::EaseOutQuart,
        Self::EaseInOutQuart,
        Self::EaseInQuint,
        Self::EaseOutQuint,
        Self::EaseInOutQuint,
        Self::EaseInSine,
        Self::EaseOutSine,
        Self::EaseInOutSine,
        Self::EaseInExpo,
        Self::EaseOutExpo,
        Self::EaseInOutExpo,
        Self::EaseInCirc,
        Self::EaseOutCirc,
        Self::EaseInOutCirc,
        Self::EaseInBack,
    ];

    /// Apply the easing function to a normalized `t` in [0.0, 1.0]
    pub fn apply(&self, t: f64) -> f64 {
        match *self {
            Easing::Linear => t,
            Easing::EaseInQuad => t * t,
            Easing::EaseOutQuad => t * (2.0 - t),
            Easing::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Easing::EaseInCubic => t * t * t,
            Easing::EaseOutCubic => {
                let t = t - 1.0;
                t * t * t + 1.0
            }
            Easing::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let t = t - 1.0;
                    1.0 + 4.0 * t * t * t
                }
            }
            Easing::EaseInQuart => t * t * t * t,
            Easing::EaseOutQuart => {
                let t = t - 1.0;
                1.0 - t * t * t * t
            }
            Easing::EaseInOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    let t = t - 1.0;
                    1.0 - 8.0 * t * t * t * t
                }
            }
            Easing::EaseInQuint => t * t * t * t * t,
            Easing::EaseOutQuint => {
                let t = t - 1.0;
                t * t * t * t * t + 1.0
            }
            Easing::EaseInOutQuint => {
                if t < 0.5 {
                    16.0 * t * t * t * t * t
                } else {
                    let t = t - 1.0;
                    1.0 + 16.0 * t * t * t * t * t
                }
            }
            Easing::EaseInSine => 1.0 - (t * std::f64::consts::FRAC_PI_2).cos(),
            Easing::EaseOutSine => (t * std::f64::consts::FRAC_PI_2).sin(),
            Easing::EaseInOutSine => 0.5 * (1.0 - (std::f64::consts::PI * t).cos()),
            Easing::EaseInExpo => {
                if t == 0.0 {
                    0.0
                } else {
                    2f64.powf(10.0 * (t - 1.0))
                }
            }
            Easing::EaseOutExpo => {
                if t == 1.0 {
                    1.0
                } else {
                    1.0 - 2f64.powf(-10.0 * t)
                }
            }
            Easing::EaseInOutExpo => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    0.5 * 2f64.powf(10.0 * (2.0 * t - 1.0))
                } else {
                    0.5 * (2.0 - 2f64.powf(-10.0 * (2.0 * t - 1.0)))
                }
            }
            Easing::EaseInCirc => 1.0 - (1.0 - t * t).sqrt(),
            Easing::EaseOutCirc => (1.0 - (t - 1.0).powi(2)).sqrt(),
            Easing::EaseInOutCirc => {
                if t < 0.5 {
                    0.5 * (1.0 - (1.0 - 4.0 * t * t).sqrt())
                } else {
                    0.5 * ((-((2.0 * t - 2.0).powi(2) - 1.0)).sqrt() + 1.0)
                }
            }
            Easing::EaseInBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.;
                c3 * t * t * t - c1 * t * t
            }
        }
    }
}

/// A generic time-based interpolator from `start` to `end`.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Tween<T> {
    pub start: T,
    pub end: T,
    pub duration: usize,
    pub elapsed: usize,
    pub easing: Easing,
    pub start_tick: Option<usize>,
}

#[allow(unused)]
impl<T> Tween<T>
where
    T: Copy + Default + PartialEq + Interpolate<T> + Add<Output = T>,
{
    /// Creates a new tween with zero duration.
    pub fn new(start: T) -> Self {
        Self {
            start,
            end: start,
            duration: 0,
            elapsed: 0,
            easing: Easing::default(),
            start_tick: None,
        }
    }

    /// Sets duration and returns modified tween.
    pub fn duration(&mut self, duration: usize) -> Self {
        self.duration = duration;
        *self
    }

    /// Sets easing and returns modified tween.
    pub fn ease(&mut self, easing: Easing) -> Self {
        self.easing = easing;
        *self
    }

    /// Mutably set duration.
    pub fn set_duration(&mut self, duration: usize) {
        self.duration = duration;
    }

    /// Mutably set easing function.
    pub fn set_ease(&mut self, easing: Easing) {
        self.easing = easing;
    }

    /// Starts a new tween toward `new_end`.
    pub fn set(&mut self, new_end: T) -> Self {
        if new_end == self.end {
            return *self;
        }
        self.start = self.get();
        self.end = new_end;
        self.elapsed = 0;
        self.start_tick = Some(turbo_genesis_ffi::sys::tick() as usize);
        *self
    }

    /// Adds a delta to the end value and resets.
    pub fn add(&mut self, delta: T) {
        self.start = self.get();
        self.end = self.end + delta;
        self.elapsed = 0;
        self.start_tick = Some(turbo_genesis_ffi::sys::tick() as usize);
    }

    /// Returns the current interpolated value.
    pub fn get(&mut self) -> T {
        if self.duration == 0 || self.elapsed >= self.duration {
            return self.end;
        }
        if self.start_tick.is_none() {
            self.start_tick = Some(turbo_genesis_ffi::sys::tick() as usize);
        }
        self.elapsed = turbo_genesis_ffi::sys::tick() as usize - self.start_tick.unwrap_or(0);
        let t = self.elapsed as f64 / self.duration.max(1) as f64;
        let eased_t = self.easing.apply(t);
        T::interpolate(eased_t, self.start, self.end)
    }

    /// Returns true if tween is complete.
    pub fn done(&mut self) -> bool {
        let _ = self.get();
        self.duration == 0 || self.elapsed >= self.duration
    }

    /// Returns ticks since tween completed.
    pub fn elapsed_since_done(&mut self) -> Option<usize> {
        let _ = self.get();
        let end_tick = self.start_tick.map_or(0, |t| t + self.duration);
        let t = turbo_genesis_ffi::sys::tick() as usize;
        if t >= end_tick {
            Some(t - end_tick)
        } else {
            None
        }
    }
}

/// Trait for interpolatable types.
pub trait Interpolate<T> {
    fn interpolate(t: f64, start: T, end: T) -> T;
}

/// Macro for implementing `Interpolate` for primitive types.
macro_rules! impl_interpolate_for {
    ($($t:ty),*) => {
        $(
            impl Interpolate<$t> for $t {
                fn interpolate(t: f64, start: $t, end: $t) -> $t {
                    let diff = end as f64 - start as f64;
                    (start as f64 + diff * t) as $t
                }
            }
        )*
    };
}

impl_interpolate_for!(f32, f64, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

/// Implements interpolation for `Bounds` by interpolating each field.
impl Interpolate<Bounds> for Bounds {
    fn interpolate(t: f64, start: Bounds, end: Bounds) -> Bounds {
        Bounds {
            x: i32::interpolate(t, start.x, end.x),
            y: i32::interpolate(t, start.y, end.y),
            w: u32::interpolate(t, start.w, end.w),
            h: u32::interpolate(t, start.h, end.h),
        }
    }
}
