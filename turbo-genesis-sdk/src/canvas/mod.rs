use crate::bounds::Bounds;
use num_traits::NumCast;

/// Creates and returns a new sprite from the given file stem.
/// Internally, this calls `sprite::Sprite::new(name)`.
pub fn sprite(file_stem: &str) -> sprite::Sprite {
    sprite::Sprite::new(file_stem)
}

/// Creates and returns a new nine-slice sprite from the given file stem and margins.
/// Internally, this calls `sprite::Sprite::new(file_stem)` and wraps it in a `NineSliceSprite`.
pub fn nine_slice(file_stem: &str, margins: (u32, u32, u32, u32)) -> nine_slice::NineSliceSprite {
    nine_slice::NineSliceSprite::new(sprite::Sprite::new(file_stem), margins)
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

/// Returns the current resolution as a tuple (width, height).
/// The resolution is fetched from the system as a single integer:
/// - The lower 16 bits represent the width.
/// - The upper bits (shifted right 16) represent the height.
pub fn resolution() -> (u32, u32) {
    let res = turbo_genesis_ffi::sys::resolution();
    let w = res & 0xffff; // Extract the lower 16 bits for width.
    let h = res >> 16; // Extract the upper bits for height.
    (w, h)
}

/// Clears the canvas using the specified color.
/// The `color` is a packed big-endian RGBA value (e.g., `0x000000ff` is black).
pub fn clear(color: u32) {
    turbo_genesis_ffi::canvas::clear(color)
}

pub(crate) mod quad;

pub mod animation;

pub mod camera;

pub mod circ;

pub mod ellipse;

pub mod flags;

pub use macros::*;
mod macros;

pub mod nine_slice;

pub mod path;

pub mod rect;

pub mod shaders;

pub mod sprite;

pub mod text;

pub mod text_box;

pub mod utils;
