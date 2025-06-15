use super::*;
use crate::bounds::*;
use num_traits::NumCast;

#[derive(Debug, Clone, Copy)]
pub struct SpriteProps {
    /// X coordinate for the sprite's position.
    x: i32,
    /// Y coordinate for the sprite's position.
    y: i32,
    /// Width of the sprite.
    w: u32,
    /// Height of the sprite.
    h: u32,
    /// X offset of the sprite texture.
    texture_x: i32,
    /// Y offset of the sprite texture.
    texture_y: i32,
    /// Primary color overlay (typically a packed ARGB/RGBA value).
    color: u32,
    /// Background color of the sprite.
    background_color: u32,
    /// Border radius for rounded corners.
    border_radius: u32,
    /// X coordinate of the origin (pivot) used for transformations.
    origin_x: i32,
    /// Y coordinate of the origin (pivot) used for transformations.
    origin_y: i32,
    /// Rotation angle of the sprite (e.g., in degrees).
    rotation: i32,
    /// Horizontal scale factor.
    scale_x: f32,
    /// Vertical scale factor.
    scale_y: f32,
    /// Flip the sprite horizontally.
    flip_x: bool,
    /// Flip the sprite vertically.
    flip_y: bool,
    /// Whether the sprite texture should be repeated.
    repeat: bool,
    /// Whether the sprite texture should cover the destination rect.
    cover: bool,
    /// Indicates if the sprite's position is absolute.
    absolute: bool,
    /// Use fixed positioning (ignores camera)
    fixed: bool,
    /// Opacity level (0.0 = fully transparent, 1.0 = fully opaque).
    opacity: f32,
    /// Speed factor for sprite animations.
    animation_speed: f32,
    /// Current animation frame index, if applicable.
    frame: Option<usize>,
}
impl Default for SpriteProps {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
            texture_x: 0,
            texture_y: 0,
            color: 0xffffffff,            // Default color overlay is white
            background_color: 0x00000000, // Default background is transparent
            border_radius: 0,
            origin_x: 0,
            origin_y: 0,
            rotation: 0,
            scale_x: 1.0, // Default scale is 1.0
            scale_y: 1.0, // Default scale is 1.0
            flip_x: false,
            flip_y: false,
            repeat: false,
            cover: true,
            absolute: false,
            fixed: false,
            opacity: 1.0,
            animation_speed: 1.0, // Default animation speed is 1.0
            frame: None,
        }
    }
}
impl SpriteProps {
    /// Creates new sprite properties with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables are disables fixed positioning
    pub fn fixed(mut self, fixed: bool) -> Self {
        self.fixed = fixed;
        self
    }

    /// Sets the position of the sprite.
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Sets the size of the sprite.
    pub fn size(mut self, w: u32, h: u32) -> Self {
        self.w = w;
        self.h = h;
        self
    }

    /// Translates the sprite’s position by the given delta.
    pub fn offset(mut self, dx: i32, dy: i32) -> Self {
        self.x += dx;
        self.y += dy;
        self
    }

    /// Sets the inner texture position for the sprite.
    pub fn tex_position(mut self, texture_x: i32, texture_y: i32) -> Self {
        self.texture_x = texture_x;
        self.texture_y = texture_y;
        self
    }

    /// Sets the primary color of the sprite.
    pub fn color(mut self, color: u32) -> Self {
        self.color = color;
        self
    }

    /// Sets the background color of the sprite.
    pub fn background_color(mut self, background_color: u32) -> Self {
        self.background_color = background_color;
        self
    }

    /// Sets the border radius for the sprite.
    pub fn border_radius(mut self, radius: u32) -> Self {
        self.border_radius = radius;
        self
    }

    /// Sets the origin point for transformations.
    pub fn origin(mut self, origin_x: i32, origin_y: i32) -> Self {
        self.origin_x = origin_x;
        self.origin_y = origin_y;
        self
    }

    /// Sets the rotation (in degrees) for the sprite.
    pub fn rotation(mut self, angle: i32) -> Self {
        self.rotation = angle;
        self
    }

    /// Sets the scale factors for the sprite.
    pub fn scale(mut self, scale_x: f32, scale_y: f32) -> Self {
        self.scale_x = scale_x;
        self.scale_y = scale_y;
        self
    }

    /// Sets the flip flags for the sprite.
    pub fn flip(mut self, flip_x: bool, flip_y: bool) -> Self {
        self.flip_x = flip_x;
        self.flip_y = flip_y;
        self
    }

    /// Enables or disables texture repeating.
    pub fn repeat(mut self, repeat: bool) -> Self {
        self.repeat = repeat;
        self
    }

    /// Sets the sprite to use absolute positioning.
    pub fn absolute(mut self, absolute: bool) -> Self {
        self.absolute = absolute;
        self
    }

    /// Sets the opacity of the sprite (0.0 to 1.0).
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    /// Sets the playback speed of the sprite animation.
    pub fn animation_speed(mut self, speed: f32) -> Self {
        self.animation_speed = speed;
        self
    }

    /// Sets a fixed frame for the sprite animation.
    pub fn frame(mut self, frame: usize) -> Self {
        self.frame = Some(frame);
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct Sprite<'a> {
    name: &'a str,
    props: SpriteProps,
}
impl<'a> Sprite<'a> {
    /// Creates a new sprite with the given name and default properties.
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            props: SpriteProps::default(),
        }
    }

    /// Enables or disables fixed positioning
    pub fn fixed(mut self, fixed: bool) -> Self {
        self.props.fixed = fixed;
        self
    }

    /// Enables or disables fixed positioning
    pub fn set_fixed(&mut self, fixed: bool) {
        self.props.fixed = fixed;
    }

    /// Sets whether the sprite texture should cover the destination rect.
    pub fn cover(mut self, cover: bool) -> Self {
        self.props.cover = cover;
        self
    }

    /// Sets whether the sprite texture should cover the destination rect.
    pub fn set_cover(&mut self, cover: bool) {
        self.props.cover = cover;
    }

    /// Sets the sprite’s position.
    pub fn position<X: NumCast, Y: NumCast>(mut self, x: X, y: Y) -> Self {
        let x: i32 = NumCast::from(x).unwrap_or(self.props.x);
        let y: i32 = NumCast::from(y).unwrap_or(self.props.y);
        self.props = self.props.position(x, y);
        self
    }

    /// Sets the sprite’s position.
    pub fn set_position<X: NumCast, Y: NumCast>(&mut self, x: X, y: Y) {
        let x: i32 = NumCast::from(x).unwrap_or(self.props.x);
        let y: i32 = NumCast::from(y).unwrap_or(self.props.y);
        self.props = self.props.position(x, y);
    }

    /// Sets the sprite’s x position.
    pub fn position_x<X: NumCast>(mut self, x: X) -> Self {
        let x: i32 = NumCast::from(x).unwrap_or(self.props.x);
        self.props = self.props.position(x, self.props.y);
        self
    }

    /// Sets the sprite’s x position.
    pub fn set_position_x<X: NumCast>(&mut self, x: X) {
        let x: i32 = NumCast::from(x).unwrap_or(self.props.x);
        self.props = self.props.position(x, self.props.y);
    }

    /// Sets the sprite’s y position.
    pub fn position_y<Y: NumCast>(mut self, y: Y) -> Self {
        let y: i32 = NumCast::from(y).unwrap_or(self.props.y);
        self.props = self.props.position(self.props.x, y);
        self
    }

    /// Sets the sprite’s y position.
    pub fn set_position_y<Y: NumCast>(&mut self, y: Y) {
        let y: i32 = NumCast::from(y).unwrap_or(self.props.y);
        self.props = self.props.position(self.props.x, y);
    }

    /// Sets the sprite’s x and y position.
    pub fn position_xy<X: NumCast, Y: NumCast>(mut self, (x, y): (X, Y)) -> Self {
        let x: i32 = NumCast::from(x).unwrap_or(self.props.x);
        let y: i32 = NumCast::from(y).unwrap_or(self.props.y);
        self.props = self.props.position(x, y);
        self
    }

    /// Sets the sprite’s x and y position.
    pub fn set_position_xy<X: NumCast, Y: NumCast>(&mut self, (x, y): (X, Y)) {
        let x: i32 = NumCast::from(x).unwrap_or(self.props.x);
        let y: i32 = NumCast::from(y).unwrap_or(self.props.y);
        self.props = self.props.position(x, y);
    }

    /// Sets the sprite’s size.
    pub fn size<W: NumCast, H: NumCast>(mut self, w: W, h: H) -> Self {
        let w: u32 = NumCast::from(w).unwrap_or(self.props.w);
        let h: u32 = NumCast::from(h).unwrap_or(self.props.h);
        self.props = self.props.size(w, h);
        self
    }

    /// Sets the sprite’s size.
    pub fn set_size<W: NumCast, H: NumCast>(&mut self, w: W, h: H) {
        let w: u32 = NumCast::from(w).unwrap_or(self.props.w);
        let h: u32 = NumCast::from(h).unwrap_or(self.props.h);
        self.props = self.props.size(w, h);
    }

    /// Sets the sprite’s width.
    pub fn set_size_w<W: NumCast>(&mut self, w: W) {
        let w: u32 = NumCast::from(w).unwrap_or(self.props.w);
        self.props = self.props.size(w, self.props.h);
    }

    /// Sets the sprite’s height.
    pub fn size_h<H: NumCast>(mut self, h: H) -> Self {
        let h: u32 = NumCast::from(h).unwrap_or(self.props.h);
        self.props = self.props.size(self.props.w, h);
        self
    }

    /// Sets the sprite’s height.
    pub fn set_size_h<H: NumCast>(&mut self, h: H) {
        let h: u32 = NumCast::from(h).unwrap_or(self.props.h);
        self.props = self.props.size(self.props.w, h);
    }

    /// Sets the sprite’s width and height.
    pub fn size_wh<W: NumCast, H: NumCast>(mut self, (w, h): (W, H)) -> Self {
        let w: u32 = NumCast::from(w).unwrap_or(self.props.w);
        let h: u32 = NumCast::from(h).unwrap_or(self.props.h);
        self.props = self.props.size(w, h);
        self
    }

    /// Sets the sprite’s width and height.
    pub fn set_size_wh<W: NumCast, H: NumCast>(&mut self, (w, h): (W, H)) {
        let w: u32 = NumCast::from(w).unwrap_or(self.props.w);
        let h: u32 = NumCast::from(h).unwrap_or(self.props.h);
        self.props = self.props.size(w, h);
    }

    /// Sets the sprite’s width.
    pub fn width<W: NumCast>(mut self, w: W) -> Self {
        let w: u32 = NumCast::from(w).unwrap_or(self.props.w);
        self.props = self.props.size(w, self.props.h);
        self
    }

    /// Sets the sprite’s width.
    pub fn set_width<W: NumCast>(&mut self, w: W) {
        let w: u32 = NumCast::from(w).unwrap_or(self.props.w);
        self.props = self.props.size(w, self.props.h);
    }

    /// Sets the sprite’s height.
    pub fn height<H: NumCast>(mut self, h: H) -> Self {
        let h: u32 = NumCast::from(h).unwrap_or(self.props.h);
        self.props = self.props.size(self.props.w, h);
        self
    }

    /// Sets the sprite’s height.
    pub fn set_height<H: NumCast>(&mut self, h: H) {
        let h: u32 = NumCast::from(h).unwrap_or(self.props.h);
        self.props = self.props.size(self.props.w, h);
    }

    /// Translates the sprite’s position by the given delta.
    pub fn offset<DX: NumCast, DY: NumCast>(mut self, dx: DX, dy: DY) -> Self {
        let dx: i32 = NumCast::from(dx).unwrap_or(0);
        let dy: i32 = NumCast::from(dy).unwrap_or(0);
        self.props = self.props.offset(dx, dy);
        self
    }

    /// Translates the sprite’s position by the given delta.
    pub fn set_offset<DX: NumCast, DY: NumCast>(&mut self, dx: DX, dy: DY) {
        let dx: i32 = NumCast::from(dx).unwrap_or(0);
        let dy: i32 = NumCast::from(dy).unwrap_or(0);
        self.props = self.props.offset(dx, dy);
    }

    /// Translates the sprite’s x position by the given delta.
    pub fn offset_x<DX: NumCast>(mut self, dx: DX) -> Self {
        let dx: i32 = NumCast::from(dx).unwrap_or(0);
        self.props = self.props.offset(dx, 0);
        self
    }

    /// Translates the sprite’s x position by the given delta.
    pub fn set_offset_x<DX: NumCast>(&mut self, dx: DX) {
        let dx: i32 = NumCast::from(dx).unwrap_or(0);
        self.props = self.props.offset(dx, 0);
    }

    /// Translates the sprite’s position by the given delta.
    pub fn offset_y<DY: NumCast>(mut self, dy: DY) -> Self {
        let dy: i32 = NumCast::from(dy).unwrap_or(0);
        self.props = self.props.offset(0, dy);
        self
    }

    /// Translates the sprite’s position by the given delta.
    pub fn set_offset_y<DY: NumCast>(&mut self, dy: DY) {
        let dy: i32 = NumCast::from(dy).unwrap_or(0);
        self.props = self.props.offset(0, dy);
    }

    /// Translates the sprite’s position by the given delta.
    pub fn offset_xy<DX: NumCast, DY: NumCast>(mut self, (dx, dy): (DX, DY)) -> Self {
        let dx: i32 = NumCast::from(dx).unwrap_or(0);
        let dy: i32 = NumCast::from(dy).unwrap_or(0);
        self.props = self.props.offset(dx, dy);
        self
    }

    /// Translates the sprite’s position by the given delta.
    pub fn set_offset_xy<DX: NumCast, DY: NumCast>(&mut self, (dx, dy): (DX, DY)) {
        let dx: i32 = NumCast::from(dx).unwrap_or(0);
        let dy: i32 = NumCast::from(dy).unwrap_or(0);
        self.props = self.props.offset(dx, dy);
    }

    /// Sets the inner texture position for the sprite.
    pub fn tex_position<TX: NumCast, TY: NumCast>(
        mut self,
        texture_x: TX,
        texture_y: TY,
    ) -> Self {
        let texture_x: i32 = NumCast::from(texture_x).unwrap_or(self.props.texture_x);
        let texture_y: i32 = NumCast::from(texture_y).unwrap_or(self.props.texture_y);
        self.props = self.props.tex_position(texture_x, texture_y);
        self
    }

    /// Sets the inner texture position for the sprite.
    pub fn set_tex_position<TX: NumCast, TY: NumCast>(&mut self, texture_x: TX, texture_y: TY) {
        let texture_x: i32 = NumCast::from(texture_x).unwrap_or(self.props.texture_x);
        let texture_y: i32 = NumCast::from(texture_y).unwrap_or(self.props.texture_y);
        self.props = self.props.tex_position(texture_x, texture_y);
    }

    /// Sets the inner texture x position for the sprite.
    pub fn tex_position_x<TX: NumCast>(mut self, texture_x: TX) -> Self {
        let texture_x: i32 = NumCast::from(texture_x).unwrap_or(self.props.texture_x);
        self.props = self.props.tex_position(texture_x, self.props.texture_y);
        self
    }

    /// Sets the inner texture x position for the sprite.
    pub fn set_tex_position_x<TX: NumCast>(&mut self, texture_x: TX) {
        let texture_x: i32 = NumCast::from(texture_x).unwrap_or(self.props.texture_x);
        self.props = self.props.tex_position(texture_x, self.props.texture_y);
    }

    /// Sets the inner texture y position for the sprite.
    pub fn tex_position_y<TY: NumCast>(mut self, texture_y: TY) -> Self {
        let texture_y: i32 = NumCast::from(texture_y).unwrap_or(self.props.texture_y);
        self.props = self.props.tex_position(self.props.texture_x, texture_y);
        self
    }

    /// Sets the inner texture y position for the sprite.
    pub fn set_tex_position_y<TY: NumCast>(&mut self, texture_y: TY) {
        let texture_y: i32 = NumCast::from(texture_y).unwrap_or(self.props.texture_y);
        self.props = self.props.tex_position(self.props.texture_x, texture_y);
    }

    /// Sets the inner texture x and y position for the sprite.
    pub fn tex_position_xy<TX: NumCast, TY: NumCast>(
        mut self,
        (texture_x, texture_y): (TX, TY),
    ) -> Self {
        let texture_x: i32 = NumCast::from(texture_x).unwrap_or(self.props.texture_x);
        let texture_y: i32 = NumCast::from(texture_y).unwrap_or(self.props.texture_y);
        self.props = self.props.tex_position(texture_x, texture_y);
        self
    }

    /// Sets the inner texture x and y position for the sprite.
    pub fn set_tex_position_xy<TX: NumCast, TY: NumCast>(
        &mut self,
        (texture_x, texture_y): (TX, TY),
    ) {
        let texture_x: i32 = NumCast::from(texture_x).unwrap_or(self.props.texture_x);
        let texture_y: i32 = NumCast::from(texture_y).unwrap_or(self.props.texture_y);
        self.props = self.props.tex_position(texture_x, texture_y);
    }

    /// Sets the color to blend with the sprite's texture.
    pub fn color(mut self, color: u32) -> Self {
        self.props = self.props.color(color);
        self
    }

    /// Sets the color to blend with the sprite's texture.
    pub fn set_color(&mut self, color: u32) {
        self.props = self.props.color(color);
    }

    /// Sets the background color.
    pub fn background_color(mut self, bg: u32) -> Self {
        self.props = self.props.background_color(bg);
        self
    }

    /// Sets the background color.
    pub fn set_background_color(&mut self, bg: u32) {
        self.props = self.props.background_color(bg);
    }

    /// Sets the border radius.
    pub fn border_radius<R: NumCast>(mut self, radius: R) -> Self {
        let radius: u32 = NumCast::from(radius).unwrap_or(0);
        self.props = self.props.border_radius(radius);
        self
    }

    /// Sets the border radius.
    pub fn set_border_radius<R: NumCast>(&mut self, radius: R) {
        let radius: u32 = NumCast::from(radius).unwrap_or(0);
        self.props = self.props.border_radius(radius);
    }

    /// Sets the origin for transformations.
    pub fn origin<X: NumCast, Y: NumCast>(mut self, origin_x: X, origin_y: Y) -> Self {
        let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.props.origin_x);
        let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.props.origin_y);
        self.props = self.props.origin(origin_x, origin_y);
        self
    }

    /// Sets the origin for transformations.
    pub fn set_origin<X: NumCast, Y: NumCast>(&mut self, origin_x: X, origin_y: Y) {
        let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.props.origin_x);
        let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.props.origin_y);
        self.props = self.props.origin(origin_x, origin_y);
    }

    /// Sets the x origin for transformations.
    pub fn origin_x<X: NumCast>(mut self, origin_x: X) -> Self {
        let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.props.origin_x);
        self.props = self.props.origin(origin_x, self.props.origin_y);
        self
    }

    /// Sets the x origin for transformations.
    pub fn set_origin_x<X: NumCast>(&mut self, origin_x: X) {
        let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.props.origin_x);
        self.props = self.props.origin(origin_x, self.props.origin_y);
    }

    /// Sets the y origin for transformations.
    pub fn origin_y<Y: NumCast>(mut self, origin_y: Y) -> Self {
        let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.props.origin_y);
        self.props = self.props.origin(self.props.origin_x, origin_y);
        self
    }

    /// Sets the y origin for transformations.
    pub fn set_origin_y<Y: NumCast>(&mut self, origin_y: Y) {
        let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.props.origin_y);
        self.props = self.props.origin(self.props.origin_x, origin_y);
    }

    /// Sets the x and y origin for transformations.
    pub fn origin_xy<X: NumCast, Y: NumCast>(mut self, (origin_x, origin_y): (X, Y)) -> Self {
        let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.props.origin_x);
        let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.props.origin_y);
        self.props = self.props.origin(origin_x, origin_y);
        self
    }

    /// Sets the x and y origin for transformations.
    pub fn set_origin_xy<X: NumCast, Y: NumCast>(&mut self, (origin_x, origin_y): (X, Y)) {
        let origin_x: i32 = NumCast::from(origin_x).unwrap_or(self.props.origin_x);
        let origin_y: i32 = NumCast::from(origin_y).unwrap_or(self.props.origin_y);
        self.props = self.props.origin(origin_x, origin_y);
    }

    /// Sets the rotation angle by degrees.
    pub fn rotation_deg<A: NumCast>(mut self, degrees: A) -> Self {
        let degrees: i32 = NumCast::from(degrees).unwrap_or(0);
        self.props = self.props.rotation(degrees);
        self
    }

    /// Sets the rotation angle by degrees.
    pub fn set_rotation_deg<A: NumCast>(&mut self, degrees: A) {
        let degrees: i32 = NumCast::from(degrees).unwrap_or(0);
        self.props = self.props.rotation(degrees);
    }

    /// Sets the rotation angle by radians.
    pub fn rotation_rad<R: NumCast>(mut self, radians: R) -> Self {
        let radian: f32 = NumCast::from(radians).unwrap_or(0.);
        let angle = (radian * 180. / std::f32::consts::PI).round() as i32;
        self.props = self.props.rotation(angle);
        self
    }

    /// Sets the rotation angle by radians.
    pub fn set_rotation_rad<R: NumCast>(&mut self, radians: R) {
        let radian: f32 = NumCast::from(radians).unwrap_or(0.);
        let angle = (radian * 180. / std::f32::consts::PI).round() as i32;
        self.props = self.props.rotation(angle);
    }

    /// Sets the scale factor.
    pub fn scale<S: NumCast>(mut self, scale: S) -> Self {
        let scale: f32 = NumCast::from(scale).unwrap_or(1.0);
        self.props = self.props.scale(scale, scale);
        self
    }

    /// Sets the scale factor.
    pub fn set_scale<S: NumCast>(&mut self, scale: S) {
        let scale: f32 = NumCast::from(scale).unwrap_or(1.0);
        self.props = self.props.scale(scale, scale);
    }

    /// Sets the x scale factor.
    pub fn scale_x<SX: NumCast>(mut self, scale_x: SX) -> Self {
        let scale_x: f32 = NumCast::from(scale_x).unwrap_or(1.0);
        self.props = self.props.scale(scale_x, self.props.scale_y);
        self
    }

    /// Sets the x scale factor.
    pub fn set_scale_x<SX: NumCast>(&mut self, scale_x: SX) {
        let scale_x: f32 = NumCast::from(scale_x).unwrap_or(1.0);
        self.props = self.props.scale(scale_x, self.props.scale_y);
    }

    /// Sets the y scale factors.
    pub fn scale_y<SY: NumCast>(mut self, scale_y: SY) -> Self {
        let scale_y: f32 = NumCast::from(scale_y).unwrap_or(1.0);
        self.props = self.props.scale(self.props.scale_x, scale_y);
        self
    }

    /// Sets the y scale factors.
    pub fn set_scale_y<SY: NumCast>(&mut self, scale_y: SY) {
        let scale_y: f32 = NumCast::from(scale_y).unwrap_or(1.0);
        self.props = self.props.scale(self.props.scale_x, scale_y);
    }

    /// Sets the x and y scale factors.
    pub fn scale_xy<SX: NumCast, SY: NumCast>(mut self, (scale_x, scale_y): (SX, SY)) -> Self {
        let scale_x: f32 = NumCast::from(scale_x).unwrap_or(1.0);
        let scale_y: f32 = NumCast::from(scale_y).unwrap_or(1.0);
        self.props = self.props.scale(scale_x, scale_y);
        self
    }

    /// Sets the x and y scale factors.
    pub fn set_scale_xy<SX: NumCast, SY: NumCast>(&mut self, (scale_x, scale_y): (SX, SY)) {
        let scale_x: f32 = NumCast::from(scale_x).unwrap_or(1.0);
        let scale_y: f32 = NumCast::from(scale_y).unwrap_or(1.0);
        self.props = self.props.scale(scale_x, scale_y);
    }

    /// Flips the sprite horizontally and/or vertically.
    pub fn flip(mut self, flip_x: bool, flip_y: bool) -> Self {
        self.props = self.props.flip(flip_x, flip_y);
        self
    }

    /// Flips the sprite horizontally and/or vertically.
    pub fn set_flip(&mut self, flip_x: bool, flip_y: bool) {
        self.props = self.props.flip(flip_x, flip_y);
    }

    /// Flips the sprite horizontally.
    pub fn flip_x(mut self, flip_x: bool) -> Self {
        self.props = self.props.flip(flip_x, self.props.flip_y);
        self
    }

    /// Flips the sprite horizontally.
    pub fn set_flip_x(&mut self, flip_x: bool) {
        self.props = self.props.flip(flip_x, self.props.flip_y);
    }

    /// Flips the sprite vertically.
    pub fn flip_y(mut self, flip_y: bool) -> Self {
        self.props = self.props.flip(self.props.flip_x, flip_y);
        self
    }

    /// Flips the sprite vertically.
    pub fn set_flip_y(&mut self, flip_y: bool) {
        self.props = self.props.flip(self.props.flip_x, flip_y);
    }

    /// Enables or disables texture repeating.
    pub fn repeat(mut self, repeat: bool) -> Self {
        self.props = self.props.repeat(repeat);
        self
    }

    /// Enables or disables texture repeating.
    pub fn set_repeat(&mut self, repeat: bool) {
        self.props = self.props.repeat(repeat);
    }

    /// Enables or disables absolute positioning.
    pub fn absolute(mut self, absolute: bool) -> Self {
        self.props = self.props.absolute(absolute);
        self
    }

    /// Enables or disables absolute positioning.
    pub fn set_absolute(mut self, absolute: bool) {
        self.props = self.props.absolute(absolute);
    }

    /// Sets the opacity.
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.props = self.props.opacity(opacity);
        self
    }

    /// Sets the opacity.
    pub fn set_opacity(&mut self, opacity: f32) {
        self.props = self.props.opacity(opacity);
    }

    /// Sets the animation speed.
    pub fn animation_speed(mut self, speed: f32) -> Self {
        self.props = self.props.animation_speed(speed);
        self
    }

    /// Sets the animation speed.
    pub fn set_animation_speed(&mut self, speed: f32) {
        self.props = self.props.animation_speed(speed);
    }

    /// Sets a fixed frame.
    pub fn frame(mut self, frame: usize) -> Self {
        self.props = self.props.frame(frame);
        self
    }

    /// Sets a fixed frame.
    pub fn set_frame(&mut self, frame: usize) {
        self.props = self.props.frame(frame);
    }

    /// Uses an animation key to set the sprite's animation frame
    pub fn animation_key(mut self, animation_key: &str) -> Self {
        let sprite = crate::canvas::animation::get(animation_key).sprite();
        self.props.frame = sprite.props.frame;
        self
    }

    // Draws the sprite
    pub fn draw(&self) {
        // Attempt to retrieve sprite source data using the sprite's name.
        // If not found, exit early.
        let Some(sprite_data) = utils::sprite::get_source_data(&self.name) else {
            return;
        };

        // Initialize flags used to modify drawing behavior.
        let mut flags: u32 = 0;

        // Set the fixed positioning flag
        if self.props.fixed {
            flags |= flags::POSITION_FIXED;
        }

        // Set the cover flag and unsets the repeat flag
        if self.props.cover {
            flags &= !flags::SPRITE_REPEAT;
            flags |= flags::SPRITE_COVER;
        }

        // Set the repeat flag and unsets the cover flag
        if self.props.repeat {
            flags &= !flags::SPRITE_COVER;
            flags |= flags::SPRITE_REPEAT;
        }

        // Set initial destination coordinates from sprite properties.
        let mut dx = self.props.x;
        let mut dy = self.props.y;

        // If absolute positioning is enabled, adjust coordinates relative to the camera.
        if !self.props.fixed && self.props.absolute {
            let (cx, cy) = crate::canvas::camera::xy(); // Retrieve camera coordinates.
            let (w, h) = crate::canvas::resolution(); // Get canvas dimensions.
            dx += cx as i32 - (w as i32 / 2); // Center the sprite horizontally.
            dy += cy as i32 - (h as i32 / 2); // Center the sprite vertically.
        }

        // Determine the destination width (dw) and height (dh) by either using provided dimensions
        // or falling back to the sprite data dimensions, then applying scaling factors.
        let mut dw = if self.props.w == 0 {
            sprite_data.width
        } else {
            self.props.w
        };
        let mut dh = if self.props.h == 0 {
            sprite_data.height
        } else {
            self.props.h
        };

        // Calculate source width (sw) based on horizontal flip.
        let mut sw = if self.props.flip_x {
            sprite_data.width as i32 * -1 // Negative width indicates a horizontal flip.
        } else {
            sprite_data.width as i32
        };

        // Calculate source height (sh) based on vertical flip.
        let mut sh = if self.props.flip_y {
            sprite_data.height as i32 * -1 // Negative height indicates a vertical flip.
        } else {
            sprite_data.height as i32
        };

        // Apply scale to destination width and height in cover mode
        if flags & flags::SPRITE_COVER != 0 {
            dw = (dw as f32 * self.props.scale_x) as u32;
            dh = (dh as f32 * self.props.scale_y) as u32;
        }

        // Apply opacity to the sprite's primary and background colors.
        let color = utils::color::apply_opacity(self.props.color, self.props.opacity);
        let background_color =
            utils::color::apply_opacity(self.props.background_color, self.props.opacity);

        // Determine the frame index for animation:
        // If a frame is explicitly set in props, use it; otherwise, compute it based on animation speed.
        let frame_index = self.props.frame.unwrap_or_else(|| {
            utils::sprite::get_frame_index(&sprite_data, self.props.animation_speed)
        }) % sprite_data.animation_frames.len();

        // Calculate the x and y position of the current sprite frame within the spritesheet
        let sx = sprite_data.x + (sprite_data.width * frame_index as u32);
        let sy = sprite_data.y;

        // Finally, draw the sprite using the calculated parameters.
        utils::sprite::draw(
            dx,                       // Adjusted x-coordinate for drawing.
            dy,                       // Adjusted y-coordinate for drawing.
            dw,                       // Drawing width.
            dh,                       // Drawing height.
            sx,                       // Source x-coordinate on the texture.
            sy,                       // Source y-coordinate on the texture.
            sw,                       // Source width (with flip adjustments).
            sh,                       // Source height (with flip adjustments).
            self.props.texture_x,     // Texture x offset.
            self.props.texture_y,     // Texture y offset.
            color,                    // Color with opacity applied.
            background_color,         // Background color with opacity applied.
            self.props.border_radius, // Border radius for rounded corners.
            if !self.props.repeat {
                0
            } else {
                (self.props.scale_x * 10000.) as u32
            },
            u32::from_be((self.props.scale_y * 10000.) as u32),
            self.props.origin_x, // Origin x-coordinate for transformations.
            self.props.origin_y, // Origin y-coordinate for transformations.
            self.props.rotation, // Rotation angle.
            flags,               // Flags that affect drawing behavior.
        );
    }
}

// TODO: opacity, origin, rotation, scaling
pub struct NineSliceSprite<'a> {
    sprite: sprite::Sprite<'a>,
    margins: (u32, u32, u32, u32), // Clockwise: left, top, right, bottom
    target: Bounds,
}
impl<'a> NineSliceSprite<'a> {
    /// Creates a new NineSliceSprite from a sprite and specified margins.
    pub fn new(sprite: sprite::Sprite<'a>, margins: (u32, u32, u32, u32)) -> Self {
        Self {
            sprite,
            margins,
            target: Bounds::default(),
        }
    }

    /// Enables are disables fixed positioning
    pub fn fixed(mut self, fixed: bool) -> Self {
        self.sprite.props.fixed = fixed;
        self
    }

    /// Enables are disables fixed positioning
    pub fn set_fixed(&mut self, fixed: bool) {
        self.sprite.props.fixed = fixed;
    }

    /// Sets the nine-slice’s position.
    pub fn position<X: NumCast, Y: NumCast>(mut self, x: X, y: Y) -> Self {
        let x: i32 = NumCast::from(x).unwrap_or(self.target.x);
        let y: i32 = NumCast::from(y).unwrap_or(self.target.y);
        self.target = self.target.position(x, y);
        self
    }

    /// Sets the nine-slice’s position.
    pub fn set_position<X: NumCast, Y: NumCast>(&mut self, x: X, y: Y) {
        let x: i32 = NumCast::from(x).unwrap_or(self.target.x);
        let y: i32 = NumCast::from(y).unwrap_or(self.target.y);
        self.target = self.target.position(x, y);
    }

    /// Sets the nine-slice’s x position.
    pub fn position_x<X: NumCast>(mut self, x: X) -> Self {
        let x: i32 = NumCast::from(x).unwrap_or(self.target.x);
        self.target = self.target.position(x, self.target.y);
        self
    }

    /// Sets the nine-slice’s x position.
    pub fn set_position_x<X: NumCast>(&mut self, x: X) {
        let x: i32 = NumCast::from(x).unwrap_or(self.target.x);
        self.target = self.target.position(x, self.target.y);
    }

    /// Sets the nine-slice’s y position.
    pub fn position_y<Y: NumCast>(mut self, y: Y) -> Self {
        let y: i32 = NumCast::from(y).unwrap_or(self.target.y);
        self.target = self.target.position(self.target.x, y);
        self
    }

    /// Sets the nine-slice’s y position.
    pub fn set_position_y<Y: NumCast>(mut self, y: Y) {
        let y: i32 = NumCast::from(y).unwrap_or(self.target.y);
        self.target = self.target.position(self.target.x, y);
    }

    /// Sets the nine-slice’s x and y position.
    pub fn position_xy<X: NumCast, Y: NumCast>(mut self, (x, y): (X, Y)) -> Self {
        let x: i32 = NumCast::from(x).unwrap_or(self.target.x);
        let y: i32 = NumCast::from(y).unwrap_or(self.target.y);
        self.target = self.target.position(x, y);
        self
    }

    /// Sets the nine-slice’s x and y position.
    pub fn set_position_xy<X: NumCast, Y: NumCast>(mut self, (x, y): (X, Y)) {
        let x: i32 = NumCast::from(x).unwrap_or(self.target.x);
        let y: i32 = NumCast::from(y).unwrap_or(self.target.y);
        self.target = self.target.position(x, y);
    }

    /// Sets the nine-slice’s size.
    pub fn size<W: NumCast, H: NumCast>(mut self, w: W, h: H) -> Self {
        let w: u32 = NumCast::from(w).unwrap_or(self.target.w);
        let h: u32 = NumCast::from(h).unwrap_or(self.target.h);
        self.target = self.target.size(w, h);
        self
    }

    /// Sets the nine-slice’s size.
    pub fn set_size<W: NumCast, H: NumCast>(mut self, w: W, h: H) {
        let w: u32 = NumCast::from(w).unwrap_or(self.target.w);
        let h: u32 = NumCast::from(h).unwrap_or(self.target.h);
        self.target = self.target.size(w, h);
    }

    /// Sets the nine-slice’s width.
    pub fn set_size_w<W: NumCast>(mut self, w: W) {
        let w: u32 = NumCast::from(w).unwrap_or(self.target.w);
        self.target = self.target.size(w, self.target.h);
    }

    /// Sets the nine-slice’s height.
    pub fn size_h<H: NumCast>(mut self, h: H) -> Self {
        let h: u32 = NumCast::from(h).unwrap_or(self.target.h);
        self.target = self.target.size(self.target.w, h);
        self
    }

    /// Sets the nine-slice’s height.
    pub fn set_size_h<H: NumCast>(mut self, h: H) {
        let h: u32 = NumCast::from(h).unwrap_or(self.target.h);
        self.target = self.target.size(self.target.w, h);
    }

    /// Sets the nine-slice’s width and height.
    pub fn size_wh<W: NumCast, H: NumCast>(mut self, (w, h): (W, H)) -> Self {
        let w: u32 = NumCast::from(w).unwrap_or(self.target.w);
        let h: u32 = NumCast::from(h).unwrap_or(self.target.h);
        self.target = self.target.size(w, h);
        self
    }

    /// Sets the nine-slice’s width and height.
    pub fn set_size_wh<W: NumCast, H: NumCast>(mut self, (w, h): (W, H)) {
        let w: u32 = NumCast::from(w).unwrap_or(self.target.w);
        let h: u32 = NumCast::from(h).unwrap_or(self.target.h);
        self.target = self.target.size(w, h);
    }

    /// Sets the nine-slice’s width.
    pub fn width<W: NumCast>(mut self, w: W) -> Self {
        let w: u32 = NumCast::from(w).unwrap_or(self.target.w);
        self.target = self.target.size(w, self.target.h);
        self
    }

    /// Sets the nine-slice’s width.
    pub fn set_width<W: NumCast>(mut self, w: W) {
        let w: u32 = NumCast::from(w).unwrap_or(self.target.w);
        self.target = self.target.size(w, self.target.h);
    }

    /// Sets the nine-slice’s height.
    pub fn height<H: NumCast>(mut self, h: H) -> Self {
        let h: u32 = NumCast::from(h).unwrap_or(self.target.h);
        self.target = self.target.size(self.target.w, h);
        self
    }

    /// Sets the nine-slice’s height.
    pub fn set_height<H: NumCast>(mut self, h: H) {
        let h: u32 = NumCast::from(h).unwrap_or(self.target.h);
        self.target = self.target.size(self.target.w, h);
    }

    /// Translates the nine-slice’s position by the given delta.
    pub fn offset<DX: NumCast, DY: NumCast>(mut self, dx: DX, dy: DY) -> Self {
        let dx: i32 = NumCast::from(dx).unwrap_or(0);
        let dy: i32 = NumCast::from(dy).unwrap_or(0);
        self.target = self.target.translate(dx, dy);
        self
    }

    /// Translates the nine-slice’s position by the given delta.
    pub fn set_offset<DX: NumCast, DY: NumCast>(mut self, dx: DX, dy: DY) {
        let dx: i32 = NumCast::from(dx).unwrap_or(0);
        let dy: i32 = NumCast::from(dy).unwrap_or(0);
        self.target = self.target.translate(dx, dy);
    }

    /// Translates the nine-slice’s x position by the given delta.
    pub fn offset_x<DX: NumCast>(mut self, dx: DX) -> Self {
        let dx: i32 = NumCast::from(dx).unwrap_or(0);
        self.target = self.target.translate(dx, 0);
        self
    }

    /// Translates the nine-slice’s x position by the given delta.
    pub fn set_offset_x<DX: NumCast>(mut self, dx: DX) {
        let dx: i32 = NumCast::from(dx).unwrap_or(0);
        self.target = self.target.translate(dx, 0);
    }

    /// Translates the nine-slice’s position by the given delta.
    pub fn offset_y<DY: NumCast>(mut self, dy: DY) -> Self {
        let dy: i32 = NumCast::from(dy).unwrap_or(0);
        self.target = self.target.translate(0, dy);
        self
    }

    /// Translates the nine-slice’s position by the given delta.
    pub fn set_offset_y<DY: NumCast>(mut self, dy: DY) {
        let dy: i32 = NumCast::from(dy).unwrap_or(0);
        self.target = self.target.translate(0, dy);
    }

    /// Translates the nine-slice’s position by the given delta.
    pub fn offset_xy<DX: NumCast, DY: NumCast>(mut self, (dx, dy): (DX, DY)) -> Self {
        let dx: i32 = NumCast::from(dx).unwrap_or(0);
        let dy: i32 = NumCast::from(dy).unwrap_or(0);
        self.target = self.target.translate(dx, dy);
        self
    }

    /// Translates the nine-slice’s position by the given delta.
    pub fn set_offset_xy<DX: NumCast, DY: NumCast>(mut self, (dx, dy): (DX, DY)) {
        let dx: i32 = NumCast::from(dx).unwrap_or(0);
        let dy: i32 = NumCast::from(dy).unwrap_or(0);
        self.target = self.target.translate(dx, dy);
    }

    pub fn target(mut self, target: Bounds) -> Self {
        self.target = target;
        self
    }

    /// Draws the sprite using nine-slice scaling within the given target bounds.
    pub fn draw(&self) {
        let source_data = match utils::sprite::get_source_data(self.sprite.name) {
            Some(data) => data,
            None => return,
        };

        let (ml, mt, mr, mb) = self.margins;
        let src = Bounds {
            x: source_data.x as i32,
            y: source_data.y as i32,
            w: source_data.width,
            h: source_data.height,
        };

        let target = self.target;

        let src_top_left = Bounds::with_size(ml, mt).position(src.x, src.y);
        let src_top_center = Bounds::with_size(src.w.saturating_sub(ml + mr), mt)
            .position(src.x + ml as i32, src.y);
        let src_top_right =
            Bounds::with_size(mr, mt).position(src.x + src.w as i32 - mr as i32, src.y);

        let src_mid_left = Bounds::with_size(ml, src.h.saturating_sub(mt + mb))
            .position(src.x, src.y + mt as i32);
        let src_center =
            Bounds::with_size(src.w.saturating_sub(ml + mr), src.h.saturating_sub(mt + mb))
                .position(src.x + ml as i32, src.y + mt as i32);
        let src_mid_right = Bounds::with_size(mr, src.h.saturating_sub(mt + mb))
            .position(src.x + src.w as i32 - mr as i32, src.y + mt as i32);

        let src_bottom_left =
            Bounds::with_size(ml, mb).position(src.x, src.y + src.h as i32 - mb as i32);
        let src_bottom_center = Bounds::with_size(src.w.saturating_sub(ml + mr), mb)
            .position(src.x + ml as i32, src.y + src.h as i32 - mb as i32);
        let src_bottom_right = Bounds::with_size(mr, mb).position(
            src.x + src.w as i32 - mr as i32,
            src.y + src.h as i32 - mb as i32,
        );

        let dst_top_left = Bounds::with_size(ml, mt).position(target.x, target.y);
        let dst_top_center = Bounds::with_size(target.w.saturating_sub(ml + mr), mt)
            .position(target.x + ml as i32, target.y);
        let dst_top_right = Bounds::with_size(mr, mt)
            .position(target.x + target.w as i32 - mr as i32, target.y);

        let dst_mid_left = Bounds::with_size(ml, target.h.saturating_sub(mt + mb))
            .position(target.x, target.y + mt as i32);
        let dst_center = Bounds::with_size(
            target.w.saturating_sub(ml + mr),
            target.h.saturating_sub(mt + mb),
        )
        .position(target.x + ml as i32, target.y + mt as i32);
        let dst_mid_right = Bounds::with_size(mr, target.h.saturating_sub(mt + mb))
            .position(target.x + target.w as i32 - mr as i32, target.y + mt as i32);

        let dst_bottom_left = Bounds::with_size(ml, mb)
            .position(target.x, target.y + target.h as i32 - mb as i32);
        let dst_bottom_center = Bounds::with_size(target.w.saturating_sub(ml + mr), mb)
            .position(target.x + ml as i32, target.y + target.h as i32 - mb as i32);
        let dst_bottom_right = Bounds::with_size(mr, mb).position(
            target.x + target.w as i32 - mr as i32,
            target.y + target.h as i32 - mb as i32,
        );

        let tex_x = self.sprite.props.texture_x;
        let tex_y = self.sprite.props.texture_y;
        let color =
            utils::color::apply_opacity(self.sprite.props.color, self.sprite.props.opacity);
        let bg_color = utils::color::apply_opacity(
            self.sprite.props.background_color,
            self.sprite.props.opacity,
        );
        let border_radius = self.sprite.props.border_radius;
        let origin_x = self.sprite.props.origin_x;
        let origin_y = self.sprite.props.origin_y;
        let rotation = self.sprite.props.rotation;
        // let flags = flags::SPRITE_REPEAT;
        let mut flags = flags::SPRITE_REPEAT;
        if self.sprite.props.fixed {
            // flags = flags::POSITION_FIXED;
            flags |= flags::POSITION_FIXED;
        }

        Self::draw_region(
            &dst_top_left,
            &src_top_left,
            tex_x,
            tex_y,
            color,
            bg_color,
            border_radius,
            origin_x,
            origin_y,
            rotation,
            flags,
        );
        Self::draw_region(
            &dst_top_center,
            &src_top_center,
            tex_x,
            tex_y,
            color,
            bg_color,
            border_radius,
            origin_x,
            origin_y,
            rotation,
            flags,
        );
        Self::draw_region(
            &dst_top_right,
            &src_top_right,
            tex_x,
            tex_y,
            color,
            bg_color,
            border_radius,
            origin_x,
            origin_y,
            rotation,
            flags,
        );

        Self::draw_region(
            &dst_mid_left,
            &src_mid_left,
            tex_x,
            tex_y,
            color,
            bg_color,
            border_radius,
            origin_x,
            origin_y,
            rotation,
            flags,
        );
        Self::draw_region(
            &dst_center,
            &src_center,
            tex_x,
            tex_y,
            color,
            bg_color,
            border_radius,
            origin_x,
            origin_y,
            rotation,
            flags,
        );
        Self::draw_region(
            &dst_mid_right,
            &src_mid_right,
            tex_x,
            tex_y,
            color,
            bg_color,
            border_radius,
            origin_x,
            origin_y,
            rotation,
            flags,
        );

        Self::draw_region(
            &dst_bottom_left,
            &src_bottom_left,
            tex_x,
            tex_y,
            color,
            bg_color,
            border_radius,
            origin_x,
            origin_y,
            rotation,
            flags,
        );
        Self::draw_region(
            &dst_bottom_center,
            &src_bottom_center,
            tex_x,
            tex_y,
            color,
            bg_color,
            border_radius,
            origin_x,
            origin_y,
            rotation,
            flags,
        );
        Self::draw_region(
            &dst_bottom_right,
            &src_bottom_right,
            tex_x,
            tex_y,
            color,
            bg_color,
            border_radius,
            origin_x,
            origin_y,
            rotation,
            flags,
        );
    }

    fn draw_region(
        dst: &Bounds,
        src: &Bounds,
        texture_x: i32,
        texture_y: i32,
        color: u32,
        bg_color: u32,
        border_radius: u32,
        origin_x: i32,
        origin_y: i32,
        rotation: i32,
        flags: u32,
    ) {
        utils::sprite::draw(
            dst.x,
            dst.y,
            dst.w,
            dst.h,
            src.x as u32,
            src.y as u32,
            src.w as i32,
            src.h as i32,
            texture_x,
            texture_y,
            color,
            bg_color,
            border_radius,
            0,
            0,
            origin_x,
            origin_y,
            rotation,
            flags,
        );
    }
}
