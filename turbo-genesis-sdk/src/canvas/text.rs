use super::{flags, quad, utils};
use num_traits::NumCast;
use quad::Quad;

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

    /// Enables are disables fixed positioning
    pub fn fixed(mut self, fixed: bool) -> Self {
        self.quad = self.quad.fixed(fixed);
        self
    }

    /// Enables are disables fixed positioning
    pub fn set_fixed(&mut self, fixed: bool) {
        self.quad.fixed = fixed;
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
            let (cx, cy) = crate::camera::xy(); // Retrieve camera coordinates.
            let (w, h) = crate::canvas::resolution(); // Get canvas dimensions.
            dx += cx as i32 - (w as i32 / 2); // Center the sprite horizontally.
            dy += cy as i32 - (h as i32 / 2); // Center the sprite vertically.
        }

        // Set the fixed positioning flag
        let flags = if self.quad.fixed {
            flags::POSITION_FIXED
        } else {
            0
        };

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
            flags,
        );
    }
}
