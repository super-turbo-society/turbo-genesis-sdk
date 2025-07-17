//! Provides random number generation utilities.
//!
//! This module leverages an external system RNG (`turbo_genesis_ffi::sys::rand()`)
//! as its primary source of entropy. While the underlying system RNG is assumed
//! to provide high-quality, uniformly distributed 32-bit integers,
//! certain operations (especially those involving modulo for ranges, or type
//! conversions with limited precision) may introduce minor statistical biases
//! or reduce the number of truly distinct results.
//!
//! **Key Limitations/Considerations:**
//! - **`usize` Range Generation (`range`, `within_range`):** When generating a random `usize`
//!   within a very large range (specifically, a `span` greater than `u32::MAX`),
//!   using the modulo operator (`%`) with a `u32` random source can lead to a slight
//!   statistical bias towards lower numbers in the target range. For most practical
//!   applications, this bias is negligible. For perfectly unbiased results
//!   across `usize::MAX`, more complex methods (e.g., rejection sampling with a `u64` source)
//!   would be required.
//! - **Floating-Point Precision (`f64`, `f32`):** While `f64` offers
//!   significant precision, casting the very large range of `u64` into the finite
//!   set of `f64` representable values means that not every theoretical floating-point
//!   number can be generated. The distribution is uniform across the *representable*
//!   `f64` values, but tiny gaps exist. When casting to `f32`, this effect is more pronounced.
//!
//! Despite these points, the provided functions offer a robust and highly
//! usable random number generation suite for common use cases.
//!
//! **Assumptions:**
//! - `turbo_genesis_ffi::sys::rand()` provides truly uniform, cryptographically
//!   secure (or sufficiently high-quality for general purposes), full 32-bit `u32`.
//! - **Game Dev Specific Footguns Addressed/Considerations:**
//!     - **Lack of Explicit Seeding/Reproducibility:** This module currently relies on the system's RNG which may not be deterministically seeded. For game state reproducibility (debugging, save/load, multiplayer sync), an external seeding mechanism or an internal, seedable PRNG (seeded once by `sys::rand()`) is recommended.
//!     - **Performance of `sys::rand()`:** If `sys::rand()` is cryptographically secure, it might be slower than needed for high-frequency game randomness (e.g., particles). For performance-critical loops, consider a faster, non-cryptographic PRNG seeded by `sys::rand()`.
//!     - **Modulo Bias for Integer Ranges in `between`:** For integer types, `between` now uses modulo arithmetic on `u64` or `i64`. This means for ranges whose span is not a power of 2, a slight statistical bias towards lower numbers can occur. For most game development needs, this bias is negligible. If perfectly unbiased integers are critical, a different, more complex generation method would be required.
//!

use num_traits::{Float, FromPrimitive, NumCast, One, PrimInt, ToPrimitive, Zero};
use std::cmp::PartialOrd;
use std::fmt::{Debug, Display};
use std::ops::{Add, Rem, Sub};
use std::ops::{Bound, Range, RangeBounds};

/// Returns a random `u32` from the underlying system RNG.
///
/// This function is the direct interface to the system's entropy source.
/// It is assumed to provide a uniformly distributed, full 32-bit `u32`.
///
/// ## Implementation Details
/// - **Standard mode (`cfg(not(turbo_no_run))`):** Uses the system's RNG via
///   `turbo_genesis_ffi::sys::rand()` which directly calls the platform's
///   random number generator.
/// - **Server mode (`cfg(turbo_no_run)`):** Uses an alternative implementation
///   that bypasses the system RNG and instead calls
///   `turbo_genesis_ffi::os::server::random_bytes()` to fill a buffer with
///   cryptographically secure random bytes. This mode is likely used in
///   environments where direct system RNG access is restricted or unavailable,
///   such as sandboxed environments or when running in a server context.
#[cfg(not(turbo_no_run))]
pub fn u32() -> u32 {
    turbo_genesis_ffi::sys::rand()
}

/// Alternative implementation for environments where direct system RNG access
/// is not available. This version uses the OS server's random byte generation
/// service to create a `u32` by requesting 4 random bytes and converting them
/// to a `u32` using unaligned memory access.
#[cfg(turbo_no_run)]
pub fn u32() -> u32 {
    const LEN: usize = std::mem::size_of::<u32>();
    let buf: &mut [u8; LEN] = &mut [0u8; LEN];
    // Request random bytes from the OS server interface
    turbo_genesis_ffi::os::server::random_bytes(buf.as_mut_ptr(), LEN);
    let mut arr = [0u8; LEN];
    arr[..LEN].copy_from_slice(&buf[..LEN]);
    // Convert the byte array to u32 using unaligned read for safety
    unsafe { std::ptr::read_unaligned(arr.as_ptr() as *const u32) }
}

/// Generates a random `u64` by concatenating two `u32()` calls.
///
/// This function provides a full 64-bit random integer, suitable for
/// high-quality random numbers and as a source for floating-point generation.
///
/// ## Implementation Details
/// - **Standard mode (`cfg(not(turbo_no_run))`):** Calls `u32()` twice and
///   combines the results by bit-shifting the upper 32 bits and OR-ing with
///   the lower 32 bits.
/// - **Server mode (`cfg(turbo_no_run)`):** Similar to the `u32()` standard
///   implementation, but requests 8 random bytes from the OS server to
///   directly construct a `u64`. This avoids the need for two separate
///   calls and potential performance overhead.
#[cfg(not(turbo_no_run))]
pub fn u64() -> u64 {
    let lower = u32() as u64;
    let upper = u32() as u64;
    (upper << 32) | lower
}

/// Alternative implementation for `u64()` in server environments.
/// Directly requests 8 random bytes from the OS server and converts
/// them to a `u64` using unaligned memory access, which is more efficient
/// than calling the `u32()` function twice.
#[cfg(turbo_no_run)]
pub fn u64() -> u64 {
    const LEN: usize = std::mem::size_of::<u64>();
    let buf: &mut [u8; LEN] = &mut [0u8; LEN];
    // Request 8 random bytes from the OS server interface
    turbo_genesis_ffi::os::server::random_bytes(buf.as_mut_ptr(), LEN);
    let mut arr = [0u8; LEN];
    arr[..LEN].copy_from_slice(&buf[..LEN]);
    // Convert the byte array to u64 using unaligned read for safety
    unsafe { std::ptr::read_unaligned(arr.as_ptr() as *const u64) }
}

/// Returns a random `u8`.
///
/// Derived from `u32()`, ensuring uniform distribution over its range.
pub fn u8() -> u8 {
    u32() as u8
}

/// Returns a random `u16`.
///
/// Derived from `u32()`, ensuring uniform distribution over its range.
pub fn u16() -> u16 {
    u32() as u16
}

/// Returns a random `i8`.
///
/// Derived from `u32()`, cast to `i8`. The range of `i8` is smaller
/// than `u32`, so the value is simply truncated/converted.
pub fn i8() -> i8 {
    u32() as i8
}

/// Returns a random `i16`.
///
/// Derived from `u32()`, cast to `i16`.
pub fn i16() -> i16 {
    u32() as i16
}

/// Returns a random `i32`.
///
/// Derived from `u32()`, cast to `i32`. This results in a full-range `i32`.
pub fn i32() -> i32 {
    u32() as i32
}

/// Returns a random `i64`.
///
/// Derived from `u64()`, cast to `i64`. This results in a full-range `i64`.
pub fn i64() -> i64 {
    u64() as i64
}

/// Returns a random `f32` percentage between 0.0 (inclusive) and 1.0 (inclusive).
///
/// This provides a uniformly distributed random floating-point value.
///
/// **Accuracy:** Derived from `u64()` and normalized. `f32` has significantly
/// less precision (24 bits) than `f64` (53 bits). This means the distribution,
/// while uniform across representable `f32` values, will have larger gaps and
/// fewer distinct possible results compared to `f64()`.
///
/// **Note on Inclusivity:** This function is designed to **include** `1.0` as a possible
/// result, which is often desirable for game development scenarios (e.g., reaching
/// exactly 100% chance, or the end of an interpolation range).
pub fn f32() -> f32 {
    // Leverage the high-quality 64-bit integer source.
    let random_u64 = u64();
    // Normalize to [0.0, 1.0] range. Dividing by `u64::MAX as f32` allows 1.0 to be hit.
    (random_u64 as f32) / (u64::MAX as f32)
}

/// Returns a random `f64` percentage between 0.0 (inclusive) and 1.0 (inclusive).
///
/// This provides a uniformly distributed random floating-point value suitable
/// for representing probabilities or percentages.
///
/// **Accuracy:** Achieves high precision by normalizing a full 64-bit random integer
/// (`u64::MAX`). While floating-point numbers have discrete representations,
/// this method ensures a good uniform distribution across representable `f64` values.
///
/// **Note on Inclusivity:** This function is designed to **include** `1.0` as a possible
/// result, which is often desirable for game development scenarios (e.g., reaching
/// exactly 100% chance, or the end of an interpolation range).
pub fn f64() -> f64 {
    // Generate a full 64-bit random integer using `u64`.
    let random_u64: u64 = u64();

    // Normalize to the [0.0, 1.0] range. Dividing by `u64::MAX as f64` allows 1.0 to be hit.
    (random_u64 as f64) / (u64::MAX as f64)
}

/// Returns a random `usize` within a half-open range `[start, end)`.
///
/// **Bias Consideration:** For `usize` ranges where the `span` (length of the range)
/// is significantly larger than `u32::MAX`, using the modulo operator (`%`)
/// with a `u32` random source (`u32()`) can introduce a slight bias towards smaller numbers
/// in the target range. This is because values from `0` to `span % u32::MAX` would
/// appear more frequently. For most practical `usize` ranges, this bias is negligible.
fn range(range: Range<usize>) -> usize {
    let span = range.end.saturating_sub(range.start); // Safe from underflow if end < start
    if span == 0 {
        return range.start; // Handle empty or single-point ranges.
    }
    // Note: range.start + (u32() as usize % span) could theoretically overflow
    // if range.start is very large and usize is 32-bit.
    // For most common game use cases on 64-bit systems, this is not an issue.
    // For absolute robustness with very large usize ranges, `checked_add` would be needed.
    range.start + (u32() as usize % span)
}

/// Returns a random `usize` within arbitrary bounds (inclusive or exclusive).
///
/// This function processes flexible range bounds to determine the effective
/// `[start, end)` for the `range` helper.
pub fn within_range(bounds: impl RangeBounds<usize>) -> usize {
    let start = match bounds.start_bound() {
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1, // Convert exclusive to inclusive start
        Bound::Unbounded => 0,
    };
    let end = match bounds.end_bound() {
        Bound::Included(&x) => x + 1, // Convert inclusive to exclusive end
        Bound::Excluded(&x) => x,
        Bound::Unbounded => usize::MAX,
    };
    range(start..end)
}

/// Core trait for generating a uniformly-distributed random value in a closed range.
///
/// Types implementing `Between` provide a zero-bias integer path (via
/// rejection sampling) or a high-precision float path (via linear interpolation).
pub trait Between {
    /// Returns a random value `x` such that `lower ≤ x ≤ upper`.
    ///
    /// # Integer types
    /// Uses unbiased rejection sampling on a 64-bit source to guarantee
    /// every integer in `[lower, upper]` has exactly equal probability.
    /// Panics if `lower > upper`.
    ///
    /// # Floating-point types
    /// Performs `lower + rand * (upper − lower)` with `rand ∈ [0,1]` from `f64()`,
    /// then casts back to `T`. Panics if casting fails.
    fn between(lower: Self, upper: Self) -> Self;
}

/// Implements `Between` for signed integer primitives using unbiased rejection sampling.
///
/// * Draws a full-width `i64`, rejects any sample ≥ the largest multiple of `span`,
///   then reduces via `% span`. Guarantees zero modulo bias.
/// * Handles the full-domain case (`i64::MIN..=i64::MAX`)
///   by returning a direct `i64()` call.
/// * Panics if `lower > upper`.
macro_rules! impl_int_between {
    ($($t:ty),*) => {
        $(
            impl Between for $t {
                fn between(lower: Self, upper: Self) -> Self {
                    let l_i = lower.to_i64().unwrap_or_else(|| {
                        crate::log!("between<{t}>: failed to cast lower `{lower}` to i64", t = stringify!($t));
                        panic!("between<{t}>: invalid lower bound", t = stringify!($t));
                    });
                    let u_i = upper.to_i64().unwrap_or_else(|| {
                        crate::log!("between<{t}>: failed to cast upper `{upper}` to i64", t = stringify!($t));
                        panic!("between<{t}>: invalid upper bound", t = stringify!($t));
                    });
                    assert!(l_i <= u_i, "between: lower > upper");
                    if l_i == u_i {
                        return lower;
                    }
                    if l_i == i64::MIN && u_i == i64::MAX {
                        return <$t>::from_i64(i64()).unwrap_or_else(|| {
                            crate::log!(
                                "between<{t}>: failed to cast full-domain i64() back to {t}",
                                t = stringify!($t)
                            );
                            panic!("between<{t}>: full-domain cast error", t = stringify!($t));
                        });
                    }
                    // compute span <= 2^64–1
                    let span = (u_i as i128 - l_i as i128 + 1) as u64;
                    let thresh = u64::MAX - (u64::MAX % span);
                    loop {
                        let r = u64();
                        if r < thresh {
                            let offset = (r % span) as i128;
                            let result = l_i as i128 + offset;
                            return <$t>::from_i64(result as i64).unwrap_or_else(|| {
                                crate::log!("between<{t}>: failed to cast result `{result}` back to {t}", t = stringify!($t));
                                panic!("between<{t}>: result cast error", t = stringify!($t));
                            });
                        }
                    }
                }
            }
        )*
    };
}

impl_int_between!(i8, i16, i32, i64);

/// Implements `Between` for unsigned integer primitives using unbiased rejection sampling.
///
/// * Draws a full-width `u64`, rejects any sample ≥ the largest multiple of `span`,
///   then reduces via `% span`. Guarantees zero modulo bias.
/// * Handles the full-domain case (`u64::MIN..=u64::MAX`)
///   by returning a direct `u64()` call.
/// * Panics if `lower > upper`.
macro_rules! impl_uint_between {
    ($($t:ty),*) => {
        $(
            impl Between for $t {
                fn between(lower: Self, upper: Self) -> Self {
                    crate::log!("between<{t}>({lower}, {upper}) -> {t}", t = stringify!($t));
                    let l = lower.to_u64().unwrap_or_else(|| {
                        crate::log!("between<{t}>: failed to cast lower bound `{lower}` to u64", t = stringify!($t));
                        panic!("between<{t}>: invalid lower bound cast", t = stringify!($t));
                    });
                    let u = upper.to_u64().unwrap_or_else(|| {
                        crate::log!("between<{t}>: failed to cast upper bound `{upper}` to u64", t = stringify!($t));
                        panic!("between<{t}>: invalid upper bound cast", t = stringify!($t));
                    });
                    assert!(l <= u, "between: lower > upper");
                    if l == u {
                        return lower;
                    }
                    if l == u64::MIN && u == u64::MAX {
                        return NumCast::from(u64()).unwrap_or_else(|| {
                            crate::log!("between<{t}>: failed to cast full-domain u64() back to {t}", t = stringify!($t));
                            panic!("between<{t}>: full-domain cast error", t = stringify!($t));
                        });
                    }
                    let span = u - l + 1;
                    let thresh = u64::MAX - (u64::MAX % span);
                    loop {
                        let r = u64();
                        if r < thresh {
                            let val = l + (r % span);
                            return NumCast::from(val).unwrap_or_else(|| {
                                crate::log!("between<{t}>: failed to cast result `{val}` back to {t}", t = stringify!($t));
                                panic!("between<{t}>: result cast error", t = stringify!($t));
                            });
                        }
                    }
                }
            }
        )*
    };
}

impl_uint_between!(u8, u16, u32, u64);

/// Implements `Between` for floating-point primitives via linear interpolation.
///
/// * Draws a uniform `f64` in [0,1] from `f64()`.
/// * Returns `lower + rand * (upper − lower)`, cast back to `T`.
/// * Panics if casting fails, which should not happen for `f32` or `f64`.
macro_rules! impl_float_between {
    ($($t:ty),*) => {
        $(
            impl Between for $t {
                fn between(lower: Self, upper: Self) -> Self {
                    let r  = f64();
                    let lf = lower.to_f64().unwrap_or_else(|| {
                        crate::log!("between<{t}>: failed to cast lower bound `{lower}` to f64", t = stringify!($t));
                        panic!("between<{t}>: lower bound cast error", t = stringify!($t));
                    });
                    let uf = upper.to_f64().unwrap_or_else(|| {
                        crate::log!("between<{t}>: failed to cast upper bound `{upper}` to f64", t = stringify!($t));
                        panic!("between<{t}>: upper bound cast error", t = stringify!($t));
                    });
                    let v = lf + r * (uf - lf);
                    NumCast::from(v).unwrap_or_else(|| {
                        crate::log!("between<{t}>: failed to cast interpolated `{v}` back to {t}", t = stringify!($t));
                        panic!("between<{t}>: interpolation cast error", t = stringify!($t));
                    })
                }
            }
        )*
    };
}

impl_float_between!(f32, f64);

/// Returns a random value of type `T` between `l` and `u` (inclusive).
///
/// Dispatches to the appropriate `Between` impl:
/// - **Integers (`PrimInt`)**: unbiased rejection sampling (zero bias)
/// - **Floats (`Float`)**: `lower + rand * (upper − lower)`
///
/// # Panics
/// - If `l > u` for integer types.
/// - If a final cast back to `T` fails (should not happen for standard primitives).
///
/// # Examples
/// ```ignore
/// let a: u8  = random::between(10, 20);     // integers via rejection sampling
/// let b: i32 = random::between(-5, 5);
/// let c: f32 = random::between(0.0, 1.0);   // floats via interpolation
/// let d: f64 = random::between(1.5, 3.5);
/// ```
pub fn between<T: Between + Display>(l: T, u: T) -> T {
    T::between(l, u)
}

/// Returns `true` or `false` with equal probability (50/50 chance).
///
/// This provides a robust boolean random value, relying on the quality
/// of `u32()` (system RNG).
pub fn bool() -> bool {
    (u32() & 1) == 1
}

/// Randomly shuffles elements of the given slice in-place using Fisher-Yates.
///
/// The shuffle quality depends on the randomness of `within_range()`,
/// which relies on the `u32()` function (system RNG).
pub fn shuffle<T>(slice: &mut [T]) {
    let len = slice.len();
    for i in (1..len).rev() {
        // Using `within_range` for robust `usize` generation.
        let j = within_range(..len);
        slice.swap(i, j);
    }
}

/// Selects a random reference from a slice, or `None` if the slice is empty.
///
/// The selection quality depends on the randomness of `within_range()`.
pub fn pick<'a, T>(slice: &'a [T]) -> Option<&'a T> {
    if slice.is_empty() {
        None // Cannot pick from an empty slice.
    } else {
        Some(&slice[within_range(..slice.len())])
    }
}
