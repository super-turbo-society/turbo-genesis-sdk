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

    /// Enables are disables fixed positioning
    pub fn fixed(mut self, fixed: bool) -> Self {
        self.quad = self.quad.fixed(fixed);
        self
    }

    /// Enables are disables fixed positioning
    pub fn set_fixed(&mut self, fixed: bool) {
        self.quad.fixed = fixed;
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

        // Set the fixed positioning flag
        let flags = if self.quad.fixed {
            flags::POSITION_FIXED
        } else {
            0
        };

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
            flags,
        );
    }
}
