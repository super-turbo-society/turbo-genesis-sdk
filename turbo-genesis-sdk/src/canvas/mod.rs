//! Graphics Utilities Module
//!
//! Contains convenience constructors and helpers for common graphical primitives
//! and canvas operations in the Turbo Genesis environment. All functions wrap
//! lower-level types (e.g., sprites, shapes, text) and perform necessary type
//! conversions or FFI calls under the hood.

use num_traits::NumCast;

// Modules and re-exports
pub mod animation;
pub mod circ;
pub mod ellipse;
pub mod flags;
pub(crate) mod quad;
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

/// Create a new sprite from the given file stem.
///
/// # Parameters
/// - `file_stem`: Base name of the image asset (without extension).
///
/// # Returns
/// A fully initialized [`sprite::Sprite`].
pub fn sprite(file_stem: &str) -> sprite::Sprite {
    sprite::Sprite::new(file_stem)
}

/// Create a nine-slice sprite, given asset name and margins.
///
/// Nine-slice sprites are useful for resizable UI panels. This wraps a
/// base sprite in a [`nine_slice::NineSliceSprite`].
///
/// # Parameters
/// - `file_stem`: Base name of the image asset.
/// - `margins`: Tuple `(left, top, right, bottom)` slice sizes in pixels.
///
/// # Returns
/// A [`nine_slice::NineSliceSprite`] configured with the given margins.
pub fn nine_slice(file_stem: &str, margins: (u32, u32, u32, u32)) -> nine_slice::NineSliceSprite {
    nine_slice::NineSliceSprite::new(sprite::Sprite::new(file_stem), margins)
}

/// Create a rectangle with specified width and height.
///
/// Uses [`NumCast`] to convert generic numeric types into `u32`. Defaults
/// to `0` on conversion failure.
///
/// # Type Parameters
/// - `W`: Width type implementing `NumCast`.
/// - `H`: Height type implementing `NumCast`.
///
/// # Returns
/// A [`rect::Rectangle`] sized accordingly.
pub fn rect<W: NumCast, H: NumCast>(w: W, h: H) -> rect::Rectangle {
    let width = NumCast::from(w).unwrap_or(0);
    let height = NumCast::from(h).unwrap_or(0);
    rect::Rectangle::new().size(width, height)
}

/// Create an ellipse with specified width and height.
///
/// Uses [`NumCast`] conversion; defaults to `0` on failure.
///
/// # Type Parameters
/// - `W`: Width type implementing `NumCast`.
/// - `H`: Height type implementing `NumCast`.
///
/// # Returns
/// An [`ellipse::Ellipse`] sized accordingly.
pub fn ellipse<W: NumCast, H: NumCast>(w: W, h: H) -> ellipse::Ellipse {
    let width = NumCast::from(w).unwrap_or(0);
    let height = NumCast::from(h).unwrap_or(0);
    ellipse::Ellipse::new().size(width, height)
}

/// Create a circle with the given diameter.
///
/// Uses [`NumCast`] conversion; defaults to `0` on failure.
///
/// # Type Parameters
/// - `D`: Diameter type implementing `NumCast + Copy`.
///
/// # Returns
/// A [`circ::Circle`] with the specified diameter.
pub fn circ<D: NumCast + Copy>(d: D) -> circ::Circle {
    let diameter = NumCast::from(d).unwrap_or(0);
    circ::Circle::new().size(diameter)
}

/// Create a line segment from start to end coordinates.
///
/// Coordinates are converted via [`NumCast`] and normalized to `u32`.
///
/// # Type Parameters
/// - `X0, Y0, X1, Y1`: Numeric types for start and end coordinates.
///
/// # Returns
/// A [`path::Path`] connecting the two points.
pub fn path<X0: NumCast + Copy, Y0: NumCast + Copy, X1: NumCast + Copy, Y1: NumCast + Copy>(
    start_x: X0,
    start_y: Y0,
    end_x: X1,
    end_y: Y1,
) -> path::Path {
    let sx = NumCast::from(start_x).unwrap_or(0);
    let sy = NumCast::from(start_y).unwrap_or(0);
    let ex = NumCast::from(end_x).unwrap_or(0);
    let ey = NumCast::from(end_y).unwrap_or(0);
    path::Path::new()
        .start_position(sx, sy)
        .end_position(ex, ey)
}

/// Create a new text object displaying the given string.
///
/// # Parameters
/// - `string`: The text content.
///
/// # Returns
/// A [`text::Text`] ready for rendering.
pub fn text(string: &str) -> text::Text {
    text::Text::new(string)
}

/// Query the current display resolution as `(width, height)`.
///
/// Internally calls FFI `resolution()`, where:
/// - Lower 16 bits encode width.
/// - Upper bits encode height.
///
/// # Returns
/// Tuple `(width, height)` in pixels.
pub fn resolution() -> (u32, u32) {
    let res = turbo_genesis_ffi::sys::resolution();
    let width = res & 0xffff;
    let height = res >> 16;
    (width, height)
}

/// Clear the entire canvas with a solid color.
///
/// # Parameters
/// - `color`: Packed big-endian RGBA (`0xRRGGBBAA`).
///
/// # Example
/// ```ignore
/// clear(0x000000ff); // Opaque black
/// ```
pub fn clear(color: u32) {
    turbo_genesis_ffi::canvas::clear(color)
}
