use std::mem::size_of;
use std::ops::{Bound, Range, RangeBounds};

/// Returns a random `u32` from the system RNG.
pub fn rand() -> u32 {
    turbo_genesis_ffi::sys::rand()
}

/// Generates a random instance of any `Copy + Default` type by filling its bytes from `rand()`.
pub fn generate<T: Copy + Default>() -> T {
    let mut out = T::default();
    let out_bytes =
        unsafe { core::slice::from_raw_parts_mut(&mut out as *mut T as *mut u8, size_of::<T>()) };

    for chunk in out_bytes.chunks_mut(4) {
        let r = rand().to_le_bytes();
        for (i, b) in chunk.iter_mut().enumerate() {
            *b = r[i];
        }
    }

    out
}

/// Returns a random `usize` within a half-open range `[start, end)`.
fn range(range: Range<usize>) -> usize {
    let span = range.end.saturating_sub(range.start);
    if span == 0 {
        return range.start;
    }
    range.start + (rand() as usize % span)
}

/// Returns a random `usize` within arbitrary bounds (inclusive or exclusive).
pub fn between(bounds: impl RangeBounds<usize>) -> usize {
    let start = match bounds.start_bound() {
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
        Bound::Unbounded => 0,
    };
    let end = match bounds.end_bound() {
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
        Bound::Unbounded => usize::MAX,
    };
    range(start..end)
}

/// Returns `true` or `false` with equal probability (50/50 chance).
pub fn bool() -> bool {
    (rand() & 1) == 1
}

/// Randomly shuffles elements of the given slice in-place using Fisher-Yates.
pub fn shuffle<T>(slice: &mut [T]) {
    let len = slice.len();
    for i in (1..len).rev() {
        let j = between(..len);
        slice.swap(i, j);
    }
}

/// Selects a random reference from a slice, or `None` if the slice is empty.
pub fn pick<'a, T>(slice: &'a [T]) -> Option<&'a T> {
    if slice.is_empty() {
        None
    } else {
        Some(&slice[between(..slice.len())])
    }
}
