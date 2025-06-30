use super::{flags, sprite, utils};
use crate::bounds::Bounds;
use num_traits::NumCast;
use sprite::Sprite;

// TODO: opacity, origin, rotation, scaling
pub struct NineSliceSprite<'a> {
    sprite: Sprite<'a>,
    margins: (u32, u32, u32, u32), // Clockwise: left, top, right, bottom
    target: Bounds,
}
impl<'a> NineSliceSprite<'a> {
    /// Creates a new NineSliceSprite from a sprite and specified margins.
    pub fn new(sprite: Sprite<'a>, margins: (u32, u32, u32, u32)) -> Self {
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
        let src_top_center =
            Bounds::with_size(src.w.saturating_sub(ml + mr), mt).position(src.x + ml as i32, src.y);
        let src_top_right =
            Bounds::with_size(mr, mt).position(src.x + src.w as i32 - mr as i32, src.y);

        let src_mid_left =
            Bounds::with_size(ml, src.h.saturating_sub(mt + mb)).position(src.x, src.y + mt as i32);
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
        let dst_top_right =
            Bounds::with_size(mr, mt).position(target.x + target.w as i32 - mr as i32, target.y);

        let dst_mid_left = Bounds::with_size(ml, target.h.saturating_sub(mt + mb))
            .position(target.x, target.y + mt as i32);
        let dst_center = Bounds::with_size(
            target.w.saturating_sub(ml + mr),
            target.h.saturating_sub(mt + mb),
        )
        .position(target.x + ml as i32, target.y + mt as i32);
        let dst_mid_right = Bounds::with_size(mr, target.h.saturating_sub(mt + mb))
            .position(target.x + target.w as i32 - mr as i32, target.y + mt as i32);

        let dst_bottom_left =
            Bounds::with_size(ml, mb).position(target.x, target.y + target.h as i32 - mb as i32);
        let dst_bottom_center = Bounds::with_size(target.w.saturating_sub(ml + mr), mb)
            .position(target.x + ml as i32, target.y + target.h as i32 - mb as i32);
        let dst_bottom_right = Bounds::with_size(mr, mb).position(
            target.x + target.w as i32 - mr as i32,
            target.y + target.h as i32 - mb as i32,
        );

        let tex_x = self.sprite.props.texture_x;
        let tex_y = self.sprite.props.texture_y;
        let color = utils::color::apply_opacity(self.sprite.props.color, self.sprite.props.opacity);
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
