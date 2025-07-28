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
    /// Use fixed positioning (ignores camera)
    pub fixed: bool,
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
            fixed: false,
        }
    }
}

impl Quad {
    /// Creates new rectangle properties with default values.
    #[allow(unused)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables are disables fixed positioning
    pub fn fixed(mut self, fixed: bool) -> Self {
        self.fixed = fixed;
        self
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
