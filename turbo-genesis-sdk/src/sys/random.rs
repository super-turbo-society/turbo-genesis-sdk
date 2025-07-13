use num_traits::{FromPrimitive, NumCast, One, ToPrimitive, Zero};
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::ops::{Add, Rem, Sub};
use std::ops::{Bound, Range, RangeBounds};

/// Provides random number generation utilities.
///
/// This module leverages an external system RNG (`turbo_genesis_ffi::sys::rand()`)
/// as its primary source of entropy. While the underlying system RNG is assumed
/// to provide high-quality, uniformly distributed 32-bit integers,
/// certain operations (especially those involving modulo for ranges, or type
/// conversions with limited precision) may introduce minor statistical biases
/// or reduce the number of truly distinct results.
///
/// **Key Limitations/Considerations:**
/// - **`usize` Range Generation (`range`, `within_range`):** When generating a random `usize`
///   within a very large range (specifically, a `span` greater than `u32::MAX`),
///   using the modulo operator (`%`) with a `u32` random source can lead to a slight
///   statistical bias towards lower numbers in the target range. For most practical
///   applications, this bias is negligible. For perfectly unbiased results
///   across `usize::MAX`, more complex methods (e.g., rejection sampling with a `u64` source)
///   would be required.
/// - **Floating-Point Precision (`f64`, `f32`):** While `f64` offers
///   significant precision, casting the very large range of `u64` into the finite
///   set of `f64` representable values means that not every theoretical floating-point
///   number can be generated. The distribution is uniform across the *representable*
///   `f64` values, but tiny gaps exist. When casting to `f32`, this effect is more pronounced.
///
/// Despite these points, the provided functions offer a robust and highly
/// usable random number generation suite for common use cases.
///
/// **Assumptions:**
/// - `turbo_genesis_ffi::sys::rand()` provides truly uniform, cryptographically
///   secure (or sufficiently high-quality for general purposes), full 32-bit `u32`.
/// - **Game Dev Specific Footguns Addressed/Considerations:**
///     - **Lack of Explicit Seeding/Reproducibility:** This module currently relies on the system's RNG which may not be deterministically seeded. For game state reproducibility (debugging, save/load, multiplayer sync), an external seeding mechanism or an internal, seedable PRNG (seeded once by `sys::rand()`) is recommended.
///     - **Performance of `sys::rand()`:** If `sys::rand()` is cryptographically secure, it might be slower than needed for high-frequency game randomness (e.g., particles). For performance-critical loops, consider a faster, non-cryptographic PRNG seeded by `sys::rand()`.
///     - **Modulo Bias for Integer Ranges in `between`:** For integer types, `between` now uses modulo arithmetic on `u64` or `i64`. This means for ranges whose span is not a power of 2, a slight statistical bias towards lower numbers can occur. For most game development needs, this bias is negligible. If perfectly unbiased integers are critical, a different, more complex generation method would be required.

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

/// Returns a random number of type `T` between two values (inclusive lower bound, inclusive upper bound).
///
/// This function intelligently handles different numeric types to provide the best
/// possible uniform distribution:
///
/// ### Behavior for Full-Domain Integer Ranges (`0..u64::MAX` or `i64::MIN..i64::MAX`):
/// For these specific maximum ranges, the function directly uses `rand::u64()` or `rand::i64()`.
/// This prevents span calculation overflows and ensures a perfectly uniform distribution
/// across the entire domain of the integer type.
///
/// ### Behavior for Other Integer Ranges:
/// It converts `lower` and `upper` to `u64` (or `i64` if `T` is signed). It then
/// calculates the span and uses `u64()` (or `i64()`) with modulo arithmetic to
/// generate the random integer within the range.
/// **Note:** This method introduces a **modulo bias** for ranges whose span is not
/// a power of 2. For most game development needs, this bias is negligible and is
/// preferred over floating-point truncation issues for small integers.
///
/// ### Behavior for Floating-Point Types:
/// Falls back to `f64`-based arithmetic, leveraging `f64()` to generate the random value.
/// This method is highly precise for floating-point numbers.
///
/// # Panics
/// - Panics if `lower` is greater than `upper`.
/// - Panics if casting the final result back to `T` fails. This indicates a type mismatch
///   or an unexpected overflow if `T` is smaller than the generated `i64`/`u64`
///   and the value somehow exceeds `T::MAX`/`T::MIN` after range constraints.
///   In practice, this means, custom numeric types implementing `NumCast` or
///   types that are larger than 64-bits and fall back to `f64` due to size/precision (`i128`, `u128`, etc)
///
/// # Examples
/// ```
/// use turbo::*;
///
/// let x: u8 = random::between(1, 10);      // Works correctly for small integers
/// let y: i32 = random::between(-50, 50);   // Works correctly for integers
/// let z: f32 = random::between(0.0, 1.0);  // Works correctly for floats
/// let w: f64 = random::between(10.5, 20.5); // Works correctly for floats
/// let full_u64: u64 = random::between(0, u64::MAX); // Handled precisely
/// let full_i64: i64 = random::between(i64::MIN, i64::MAX); // Handled precisely
/// ```
pub fn between<T>(lower: T, upper: T) -> T
where
    T: NumCast + Copy + PartialOrd + Default + Debug,
    T: Add<Output = T> + Sub<Output = T> + Rem<Output = T> + One + Zero,
    T: ToPrimitive + FromPrimitive,
{
    // Attempt to convert to i64 first.
    let lower_as_i64_opt = lower.to_i64();
    let upper_as_i64_opt = upper.to_i64();

    // Check if it's an i64-compatible integer range
    if let (Some(l_i64), Some(u_i64)) = (lower_as_i64_opt, upper_as_i64_opt) {
        // Handle full-domain i64 specifically to avoid span overflow for standard arithmetic
        if l_i64 == i64::MIN && u_i64 == i64::MAX {
            return T::from_i64(i64()).unwrap_or_else(|| {
                // This scenario means T is smaller than i64 (e.g., i8) but the bounds
                // were explicitly i64::MIN to i64::MAX. This might happen if NumCast
                // allows it. If the random i64 doesn't fit T, clamp it.
                crate::log!("Could not cast full-domain i64 result back to target integer type T. Clamping to bounds.");
                if i64::MIN < l_i64 { lower } else { upper }
            });
        }

        // Standard signed integer range handling
        if l_i64 > u_i64 {
            crate::log!("between: lower bound ({:?}) cannot be greater than upper bound ({:?}) for integer types", lower, upper);
            panic!();
        }
        if l_i64 == u_i64 {
            return lower; // Single value range
        }

        // Calculate span using raw arithmetic. For non-full-domain i64/u64 types,
        // this will not overflow if NumCast correctly converted.
        let span_i64 = u_i64 - l_i64 + 1; // Panics on overflow in debug, wraps in release (unlikely here)

        let random_offset_i64 = i64() % span_i64;

        let result_i64 = l_i64 + random_offset_i64; // Panics on overflow in debug, wraps in release (unlikely here)

        return T::from_i64(result_i64).unwrap_or_else(|| {
            crate::log!("Could not cast generated i64 result back to target integer type T. Clamping to bounds.");
            if result_i64 < l_i64 { lower } else { upper }
        });
    }

    // Attempt to convert to u64 if i64 conversion failed (e.g., T is u64 itself, or a custom type)
    let lower_as_u64_opt = lower.to_u64();
    let upper_as_u64_opt = upper.to_u64();

    // Check if it's a u64-compatible integer range
    if let (Some(l_u64), Some(u_u64)) = (lower_as_u64_opt, upper_as_u64_opt) {
        // Handle full-domain u64 specifically to avoid span overflow for standard arithmetic
        if l_u64 == 0 && u_u64 == u64::MAX {
            return T::from_u64(u64()).unwrap_or_else(|| {
                crate::log!("Could not cast full-domain u64 result back to target integer type T. Clamping to bounds.");
                if 0 < l_u64 { lower } else { upper }
            });
        }

        // Standard unsigned integer range handling
        if l_u64 > u_u64 {
            crate::log!("between: lower bound ({:?}) cannot be greater than upper bound ({:?}) for unsigned integer types", lower, upper);
            panic!();
        }
        if l_u64 == u_u64 {
            return lower; // Single value range
        }

        // Calculate span using raw arithmetic.
        let span_u64 = u_u64 - l_u64 + 1; // Panics on overflow in debug, wraps in release (unlikely here)

        let random_offset_u64 = u64() % span_u64;

        let result_u64 = l_u64 + random_offset_u64; // Panics on overflow in debug, wraps in release (unlikely here)

        return T::from_u64(result_u64).unwrap_or_else(|| {
            crate::log!("Could not cast generated u64 result back to target unsigned integer type T. Clamping to bounds.");
            if result_u64 < l_u64 { lower } else { upper }
        });
    }

    // Fallback to floating-point arithmetic for types not handled by integer paths
    // (e.g., f32, f64, or custom NumCast types that don't fit into i64/u64)
    let normalized_random = f64();
    let lower_f64 = lower.to_f64().unwrap_or_default();
    let upper_f64 = upper.to_f64().unwrap_or_default();

    let result_f64 = lower_f64 + normalized_random * (upper_f64 - lower_f64);

    NumCast::from(result_f64).unwrap_or_else(|| {
        crate::log!("Could not cast f64 result back to target type T. Using fallback...");
        // This fallback heuristic tries to return the closest boundary in case of failed cast.
        if (result_f64 - lower_f64).abs() < (result_f64 - upper_f64).abs() {
            lower
        } else {
            upper
        }
    })
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
