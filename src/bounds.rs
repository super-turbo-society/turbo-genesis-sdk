use crate::input::pointer;
use borsh::{BorshDeserialize, BorshSerialize};
use num_traits::NumCast;
use std::ops::Add;

//------------------------------------------------------------------------------
// Bounds
//------------------------------------------------------------------------------

/// Returns the current viewport bounds.
/// This is typically used to get the canvas or screen boundaries.
pub fn viewport() -> Bounds {
    let (w, h) = crate::canvas::resolution();
    let (x, y, z) = crate::canvas::camera::xyz();
    let w = (w as f32 * (1. / z)) as u32;
    let h = (h as f32 * (1. / z)) as u32;
    let x = (x as f32 - (w as f32 / 2.)) as i32;
    let y = (y as f32 - (h as f32 / 2.)) as i32;
    Bounds::new(x, y, w, h)
}

/// Returns the fixed canvas bounds without any camera position or zoom adjustments.
/// This is typically useful for GUI elements that are fixed in size and position relative to the game's canvas.
pub fn canvas() -> Bounds {
    let (w, h) = crate::canvas::resolution();
    Bounds::new(0, 0, w, h)
}

/// `Bounds` represents a rectangular region in 2D space.
/// It is the core primitive for low-res immediate-mode graphics,
/// providing essential methods for positioning, sizing, and geometric operations.
#[derive(Debug, Clone, Copy, Default, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Bounds {
    // Top-left corner coordinates.
    pub(crate) x: i32,
    pub(crate) y: i32,
    // Width and height of the rectangle.
    pub(crate) w: u32,
    pub(crate) h: u32,
}

impl Bounds {
    /// Creates a new `Bounds` with the specified position and size,
    pub fn new<X: NumCast, Y: NumCast, W: NumCast, H: NumCast>(x: X, y: Y, w: W, h: H) -> Self {
        let x = NumCast::from(x).unwrap_or(0);
        let y = NumCast::from(y).unwrap_or(0);
        let w = NumCast::from(w).unwrap_or(0);
        let h = NumCast::from(h).unwrap_or(0);
        Self { x, y, w, h }
    }

    /// Creates a new `Bounds` with the specified width and height,
    /// defaulting to the origin (0,0).
    pub fn with_size<W: NumCast, H: NumCast>(w: W, h: H) -> Self {
        let w = NumCast::from(w).unwrap_or(0);
        let h = NumCast::from(h).unwrap_or(0);
        Self { x: 0, y: 0, w, h }
    }

    /// Returns the current position (x, y) of the top-left corner.
    pub const fn xy(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Returns the current size (width, height).
    pub const fn wh(&self) -> (u32, u32) {
        (self.w, self.h)
    }

    /// Returns the current width.
    pub const fn w(&self) -> u32 {
        self.w
    }

    /// Returns the current height.
    pub const fn h(&self) -> u32 {
        self.h
    }

    /// Returns the x-coordinate of the left edge.
    pub const fn x(&self) -> i32 {
        self.x
    }

    /// Returns the y-coordinate of the top edge.
    pub const fn y(&self) -> i32 {
        self.y
    }

    /// Returns the y-coordinate of the top edge.
    pub const fn top(&self) -> i32 {
        self.y
    }

    /// Returns the y-coordinate of the bottom edge.
    ///
    /// Computed as `top + height - 1` to account for 0-based indexing.
    pub const fn bottom(&self) -> i32 {
        self.y + self.h.saturating_sub(1) as i32
    }

    /// Returns the x-coordinate of the left edge.
    pub const fn left(&self) -> i32 {
        self.x
    }

    /// Returns the x-coordinate of the right edge.
    ///
    /// Computed as `left + width - 1` to account for 0-based indexing.
    pub const fn right(&self) -> i32 {
        self.x + self.w.saturating_sub(1) as i32
    }

    /// Returns the coordinates of the top-left corner.
    pub const fn top_left(&self) -> (i32, i32) {
        (self.left(), self.top())
    }

    /// Returns the coordinates of the top-right corner.
    pub const fn top_right(&self) -> (i32, i32) {
        (self.right(), self.top())
    }

    /// Returns the coordinates of the bottom-left corner.
    pub const fn bottom_left(&self) -> (i32, i32) {
        (self.left(), self.bottom())
    }

    /// Returns the coordinates of the bottom-right corner.
    pub const fn bottom_right(&self) -> (i32, i32) {
        (self.right(), self.bottom())
    }

    /// Sets the position (top-left corner) of the bounds.
    ///
    /// # Parameters
    /// - `x`: The new x-coordinate.
    /// - `y`: The new y-coordinate.
    pub fn position<X: NumCast, Y: NumCast>(mut self, x: X, y: Y) -> Self {
        self.x = NumCast::from(x).unwrap_or(0);
        self.y = NumCast::from(y).unwrap_or(0);
        self
    }

    /// Sets the position (top-left corner) of the bounds.
    ///
    /// # Parameters
    /// - `x`: The new x-coordinate.
    /// - `y`: The new y-coordinate.
    pub fn position_xy<X: NumCast, Y: NumCast>(mut self, pos: (X, Y)) -> Self {
        self.x = NumCast::from(pos.0).unwrap_or(0);
        self.y = NumCast::from(pos.1).unwrap_or(0);
        self
    }

    /// Sets the size of the bounds.
    ///
    /// # Parameters
    /// - `w`: The new width.
    /// - `h`: The new height.
    pub fn size<W: NumCast, H: NumCast>(mut self, w: W, h: H) -> Self {
        self.w = NumCast::from(w).unwrap_or(0);
        self.h = NumCast::from(h).unwrap_or(0);
        self
    }

    /// Sets the width.
    ///
    /// # Parameters
    /// - `w`: The new width.
    pub fn width<T: NumCast>(mut self, w: T) -> Self {
        self.w = NumCast::from(w).unwrap_or(0);
        self
    }

    /// Sets the height.
    ///
    /// # Parameters
    /// - `w`: The new height.
    pub fn height<T: NumCast>(mut self, h: T) -> Self {
        self.h = NumCast::from(h).unwrap_or(0);
        self
    }

    /// Adjusts the width by the specified delta.
    ///
    /// The delta can be positive (increasing the width) or negative (decreasing the width).
    /// If the adjustment would result in a negative width, it is clamped to 0.
    ///
    /// # Parameters
    /// - `delta`: The amount to adjust the width by.
    ///
    /// # Example
    /// ```
    /// let b = Bounds::new(100, 50);
    /// let wider = b.adjust_width(20);   // width becomes 120
    /// let narrower = b.adjust_width(-30); // width becomes 70
    /// ```
    pub fn adjust_width<T: NumCast>(mut self, delta: T) -> Self {
        let delta: i32 = NumCast::from(delta).unwrap_or(0);
        let new_width = self.w as i32 + delta;
        self.w = new_width.max(0) as u32;
        self
    }

    /// Adjusts the height by the specified delta.
    ///
    /// The delta can be positive (increasing the height) or negative (decreasing the height).
    /// If the adjustment would result in a negative height, it is clamped to 0.
    ///
    /// # Parameters
    /// - `delta`: The amount to adjust the height by.
    ///
    /// # Example
    /// ```
    /// let b = Bounds::new(100, 50);
    /// let taller = b.adjust_height(10);    // height becomes 60
    /// let shorter = b.adjust_height(-20);   // height becomes 30
    /// ```
    pub fn adjust_height<T: NumCast>(mut self, delta: T) -> Self {
        let delta: i32 = NumCast::from(delta).unwrap_or(0);
        let new_height = self.h as i32 + delta;
        self.h = new_height.max(0) as u32;
        self
    }

    /// Moves the bounds by the specified amounts.
    ///
    /// # Parameters
    /// - `dx`: Change in x-coordinate.
    /// - `dy`: Change in y-coordinate.
    pub fn translate<T: NumCast, U: NumCast>(mut self, dx: T, dy: U) -> Self {
        self.x += NumCast::from(dx).unwrap_or(0);
        self.y += NumCast::from(dy).unwrap_or(0);
        self
    }

    /// Moves the y position of the bounds by the specified amount.
    ///
    /// # Parameters
    /// - `dy`: Change in y-coordinate.
    pub fn translate_y<T: NumCast>(mut self, dy: T) -> Self {
        self.y += NumCast::from(dy).unwrap_or(0);
        self
    }

    /// Moves the x position of the bounds by the specified amount.
    ///
    /// # Parameters
    /// - `dx`: Change in y-coordinate.
    pub fn translate_x<T: NumCast>(mut self, dx: T) -> Self {
        self.x += NumCast::from(dx).unwrap_or(0);
        self
    }

    /// Translates the bounds by a fraction of its own dimensions.
    ///
    /// This method shifts the current bounds by `rel_dx` times its width and
    /// `rel_dy` times its height. It's useful for moving an element relative
    /// to its size in immediate-mode layouts.
    ///
    /// # Parameters
    /// - `rel_dx`: Horizontal translation as a fraction of the bounds' width.
    /// - `rel_dy`: Vertical translation as a fraction of the bounds' height.
    ///
    /// # Returns
    /// A new `Bounds` instance with the updated position.
    pub fn translate_by_fraction(&self, rel_dx: f32, rel_dy: f32) -> Self {
        let dx = (self.w as f32 * rel_dx).round() as i32;
        let dy = (self.h as f32 * rel_dy).round() as i32;
        Self {
            x: self.x + dx,
            y: self.y + dy,
            w: self.w,
            h: self.h,
        }
    }

    /// Checks if the current bounds fully contains the provided `other` bounds.
    ///
    /// # Parameters
    /// - `other`: The other bounds to check for full containment.
    ///
    /// # Returns
    /// - `true` if the entire area of `other` lies within the current bounds; otherwise, `false`.
    pub fn contains(&self, other: &Self) -> bool {
        self.left() <= other.left()
            && self.right() >= other.right()
            && self.top() <= other.top()
            && self.bottom() >= other.bottom()
    }

    /// Determines whether these bounds intersects with another.
    ///
    /// # Parameters
    /// - `other`: The other bounds to test.
    ///
    /// # Returns
    /// - `true` if there is any overlap; otherwise, `false`.
    pub const fn intersects(&self, other: &Self) -> bool {
        self.left() <= other.right()
            && self.right() >= other.left()
            && self.top() <= other.bottom()
            && self.bottom() >= other.top()
    }

    /// Checks if a given point is inside this bounds.
    ///
    /// # Parameters
    /// - `x`: The x-coordinate of the point.
    /// - `y`: The y-coordinate of the point.
    ///
    /// # Returns
    /// - `true` if the point lies within the bounds; otherwise, `false`.
    pub fn intersects_position<T: NumCast, U: NumCast>(&self, x: T, y: U) -> bool {
        let x: i32 = NumCast::from(x).unwrap_or(0);
        let y: i32 = NumCast::from(y).unwrap_or(0);
        x >= self.left() && x <= self.right() && y >= self.top() && y <= self.bottom()
    }

    /// Checks if a given point is inside this bounds.
    ///
    /// # Parameters
    /// - `x`: The x-coordinate of the point.
    /// - `y`: The y-coordinate of the point.
    ///
    /// # Returns
    /// - `true` if the point lies within the bounds; otherwise, `false`.
    pub fn intersects_xy<T: NumCast, U: NumCast>(&self, (x, y): (T, U)) -> bool {
        let x: i32 = NumCast::from(x).unwrap_or(0);
        let y: i32 = NumCast::from(y).unwrap_or(0);
        x >= self.left() && x <= self.right() && y >= self.top() && y <= self.bottom()
    }

    /// Calculates the intersection of these bounds with another.
    ///
    /// # Parameters
    /// - `other`: The other bounds to intersect with.
    ///
    /// # Returns
    /// - `Some(Bounds)` representing the overlapping area if it exists,
    ///   or `None` if there is no overlap.
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if !self.intersects(other) {
            return None;
        }
        let x1 = self.left().max(other.left());
        let y1 = self.top().max(other.top());
        let x2 = self.right().min(other.right());
        let y2 = self.bottom().min(other.bottom());
        // Calculate inclusive width and height.
        let width = (x2 - x1 + 1) as u32;
        let height = (y2 - y1 + 1) as u32;
        Some(Self {
            x: x1,
            y: y1,
            w: width,
            h: height,
        })
    }

    /// Computes the union of these bounds with another.
    ///
    /// The union is the smallest rectangle that completely contains both bounds.
    ///
    /// # Parameters
    /// - `other`: The other bounds to unite with.
    pub fn union(&self, other: &Self) -> Self {
        let x1 = self.left().min(other.left());
        let y1 = self.top().min(other.top());
        let x2 = self.right().max(other.right());
        let y2 = self.bottom().max(other.bottom());
        // Calculate inclusive width and height.
        let width = (x2 - x1 + 1) as u32;
        let height = (y2 - y1 + 1) as u32;
        Self {
            x: x1,
            y: y1,
            w: width,
            h: height,
        }
    }

    /// Returns the center point (x, y) of the bounds.
    pub const fn center(&self) -> (i32, i32) {
        (
            self.left() + (self.w as i32) / 2,
            self.top() + (self.h as i32) / 2,
        )
    }

    /// Returns the center x position of the bounds.
    pub const fn center_x(&self) -> i32 {
        self.left() + (self.w as i32) / 2
    }

    /// Returns the center y position of the bounds.
    pub const fn center_y(&self) -> i32 {
        self.top() + (self.h as i32) / 2
    }

    /// Scales the bounds uniformly by a factor.
    ///
    /// # Parameters
    /// - `factor`: The multiplier applied to both width and height.
    pub fn scale<T: NumCast>(mut self, factor: T) -> Self {
        let factor: f32 = NumCast::from(factor).unwrap_or(1.0);
        self.w = ((self.w as f32) * factor) as u32;
        self.h = ((self.h as f32) * factor) as u32;
        self
    }

    /// Returns a new `Bounds` that is a sub-rectangle defined by relative offsets
    /// and dimensions. All parameters are fractions (0.0–1.0) of the parent bounds.
    ///
    /// # Parameters
    /// - `rel_x`: Fractional horizontal offset from the parent's left edge.
    ///
    /// # Example
    /// ```
    /// let parent = Bounds::new(100, 100);
    /// let child = parent.translate_x_by_fraction(0.1);
    /// ```
    pub fn translate_x_by_fraction(&self, rel_x: f32) -> Self {
        let new_x = self.x + ((self.w as f32) * rel_x).round() as i32;
        Self {
            x: new_x,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }

    /// Returns a new `Bounds` that is a sub-rectangle defined by relative offsets
    /// and dimensions. All parameters are fractions (0.0–1.0) of the parent bounds.
    ///
    /// # Parameters
    /// - `rel_y`: Fractional vertical offset from the parent's top edge.
    ///
    /// # Example
    /// ```
    /// let parent = Bounds::new(100, 100);
    /// let child = parent.translate_y_by_fraction(0.1);
    /// ```
    pub fn translate_y_by_fraction(&self, rel_y: f32) -> Self {
        let new_y = self.y + ((self.h as f32) * rel_y).round() as i32;
        Self {
            x: self.x,
            y: new_y,
            w: self.w,
            h: self.h,
        }
    }

    /// Returns a new `Bounds` with its width set to a fraction of the parent's width.
    /// The x and y position, as well as the height, remain unchanged.
    ///
    /// # Parameters
    /// - `rel_w`: Fraction of the parent's width.
    ///
    /// # Example
    /// ```
    /// let parent = Bounds::new(200, 100);
    /// let child = parent.adjust_width_by_fraction(0.5);
    /// // child.w == 100
    /// ```
    pub fn adjust_width_by_fraction(&self, rel_w: f32) -> Self {
        let new_w = ((self.w as f32) * rel_w).round() as u32;
        Self {
            x: self.x,
            y: self.y,
            w: new_w,
            h: self.h,
        }
    }

    /// Returns a new `Bounds` with its height set to a fraction of the parent's height.
    /// The x and y position, as well as the width, remain unchanged.
    ///
    /// # Parameters
    /// - `rel_h`: Fraction of the parent's height.
    ///
    /// # Example
    /// ```
    /// let parent = Bounds::new(100, 200);
    /// let child = parent.adjust_height_by_fraction(0.5);
    /// // child.h == 100
    /// ```
    pub fn adjust_height_by_fraction(&self, rel_h: f32) -> Self {
        let new_h = ((self.h as f32) * rel_h).round() as u32;
        Self {
            x: self.x,
            y: self.y,
            w: self.w,
            h: new_h,
        }
    }

    /// Splits the current bounds horizontally into two sub-bounds using a pixel value.
    /// The left bound will have a width equal to `left_width` (clamped to the bounds’ width),
    /// and the right bound receives the remaining width.
    ///
    /// # Parameters
    /// - `left_width`: The width in pixels for the left sub-bound.
    ///
    /// # Returns
    /// A tuple `(left_bounds, right_bounds)`.
    pub fn split_horizontal_at<T: NumCast>(&self, left_width: T) -> (Self, Self) {
        // Convert the provided left width to an i32 and clamp it within [0, self.w].
        let left_width: i32 = NumCast::from(left_width).unwrap_or(0);
        let left_width = left_width.clamp(0, self.w as i32) as u32;
        let right_w = self.w.saturating_sub(left_width);
        let left = Self {
            x: self.x,
            y: self.y,
            w: left_width,
            h: self.h,
        };
        let right = Self {
            x: self.x + left_width as i32,
            y: self.y,
            w: right_w,
            h: self.h,
        };
        (left, right)
    }

    /// Splits the current bounds vertically into two sub-bounds using a pixel value.
    /// The top bound will have a height equal to `top_height` (clamped to the bounds’ height),
    /// and the bottom bound receives the remaining height.
    ///
    /// # Parameters
    /// - `top_height`: The height in pixels for the top sub-bound.
    ///
    /// # Returns
    /// A tuple `(top_bounds, bottom_bounds)`.
    pub fn split_vertical_at<T: NumCast>(&self, top_height: T) -> (Self, Self) {
        // Convert the provided top height to an i32 and clamp it within [0, self.h].
        let top_height: i32 = NumCast::from(top_height).unwrap_or(0);
        let top_height = top_height.clamp(0, self.h as i32) as u32;
        let bottom_h = self.h.saturating_sub(top_height);
        let top = Self {
            x: self.x,
            y: self.y,
            w: self.w,
            h: top_height,
        };
        let bottom = Self {
            x: self.x,
            y: self.y + top_height as i32,
            w: self.w,
            h: bottom_h,
        };
        (top, bottom)
    }

    /// Splits the current bounds horizontally into two sub-bounds.
    /// The left bound takes `ratio` fraction of the width, and the right gets the remainder.
    ///
    /// # Parameters
    /// - `ratio`: A value between 0.0 and 1.0 representing the fraction of width for the left sub-bound.
    ///
    /// # Returns
    /// A tuple `(left_bounds, right_bounds)`.
    pub fn split_horizontal_by_fraction(&self, ratio: f32) -> (Self, Self) {
        let left_w = ((self.w as f32) * ratio).round() as u32;
        let right_w = self.w.saturating_sub(left_w);
        let left = Self {
            x: self.x,
            y: self.y,
            w: left_w,
            h: self.h,
        };
        let right = Self {
            x: self.x + left_w as i32,
            y: self.y,
            w: right_w,
            h: self.h,
        };
        (left, right)
    }

    /// Splits the current bounds vertically into two sub-bounds.
    /// The top bound takes `ratio` fraction of the height, and the bottom gets the remainder.
    ///
    /// # Parameters
    /// - `ratio`: A value between 0.0 and 1.0 representing the fraction of height for the top sub-bound.
    ///
    /// # Returns
    /// A tuple `(top_bounds, bottom_bounds)`.
    pub fn split_vertical_by_fraction(&self, ratio: f32) -> (Self, Self) {
        let top_h = ((self.h as f32) * ratio).round() as u32;
        let bottom_h = self.h.saturating_sub(top_h);
        let top = Self {
            x: self.x,
            y: self.y,
            w: self.w,
            h: top_h,
        };
        let bottom = Self {
            x: self.x,
            y: self.y + top_h as i32,
            w: self.w,
            h: bottom_h,
        };
        (top, bottom)
    }

    /// Returns a new `Bounds` that is inset by a uniform margin on all sides.
    /// Useful for creating padding within a parent bounds.
    ///
    /// # Parameters
    /// - `margin`: The amount to inset from each side. Should be non-negative.
    ///
    /// # Returns
    /// A new `Bounds` with reduced size.
    pub fn inset<T: NumCast>(&self, margin: T) -> Self {
        let margin = NumCast::from(margin).unwrap_or(0);
        let new_x = self.x + margin;
        let new_y = self.y + margin;
        let new_w = self.w.saturating_sub(margin as u32 * 2);
        let new_h = self.h.saturating_sub(margin as u32 * 2);
        Self {
            x: new_x,
            y: new_y,
            w: new_w,
            h: new_h,
        }
    }

    /// Returns a new `Bounds` that is inset by a uniform margin on the top side.
    /// This shifts the top edge down by the given margin.
    ///
    /// # Parameters
    /// - `margin`: The amount to inset from the top.
    ///
    /// # Returns
    /// A new `Bounds` with the top edge moved down.
    pub fn inset_top<T: NumCast>(&self, margin: T) -> Self {
        let margin = NumCast::from(margin).unwrap_or(0);
        let new_y = self.y + margin;
        let new_h = self.h.saturating_sub(margin as u32);
        Self {
            x: self.x,
            y: new_y,
            w: self.w,
            h: new_h,
        }
    }

    /// Returns a new `Bounds` that is inset by a uniform margin on the bottom side.
    /// This reduces the height such that the bottom edge is moved up by the given margin.
    ///
    /// # Parameters
    /// - `margin`: The amount to inset from the bottom.
    ///
    /// # Returns
    /// A new `Bounds` with the height reduced to simulate a bottom inset.
    pub fn inset_bottom<T: NumCast>(&self, margin: T) -> Self {
        let margin = NumCast::from(margin).unwrap_or(0);
        let new_h = self.h.saturating_sub(margin);
        Self {
            x: self.x,
            y: self.y,
            w: self.w,
            h: new_h,
        }
    }

    /// Returns a new `Bounds` that is inset by a uniform margin on the left side.
    /// Useful for creating padding within a parent bounds.
    ///
    /// # Parameters
    /// - `margin`: The amount to inset from each side. Should be non-negative.
    ///
    /// # Returns
    /// A new `Bounds` with reduced size.
    pub fn inset_left<T: NumCast>(&self, margin: T) -> Self {
        let margin = NumCast::from(margin).unwrap_or(0);
        let new_x = self.x + margin;
        let new_w = self.w.saturating_sub(margin as u32);
        Self {
            x: new_x,
            y: self.y,
            w: new_w,
            h: self.h,
        }
    }

    /// Returns a new `Bounds` that is inset by a uniform margin on the right side.
    /// Useful for creating padding within a parent bounds.
    ///
    /// # Parameters
    /// - `margin`: The amount to inset from each side. Should be non-negative.
    ///
    /// # Returns
    /// A new `Bounds` with reduced size.
    pub fn inset_right<T: NumCast>(&self, margin: T) -> Self {
        let margin = NumCast::from(margin).unwrap_or(0);
        let new_w = self.w.saturating_sub(margin);
        Self {
            x: self.x,
            y: self.y,
            w: new_w,
            h: self.h,
        }
    }

    /// Anchors the current bounds to the left edge of the container.
    /// The returned bounds maintains the same size and original vertical position.
    pub fn anchor_left(&self, container: &Self) -> Self {
        Self {
            x: container.x,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }

    /// Anchors the current bounds to the right edge of the container.
    /// The returned bounds maintains the same size and original vertical position.
    pub fn anchor_right(&self, container: &Self) -> Self {
        Self {
            x: container.x + container.w as i32 - self.w as i32,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }

    /// Anchors the current bounds to the top edge of the container.
    /// The returned bounds maintains the same size and original horizontal position.
    pub fn anchor_top(&self, container: &Self) -> Self {
        Self {
            x: self.x,
            y: container.y,
            w: self.w,
            h: self.h,
        }
    }

    /// Anchors the current bounds to the bottom edge of the container.
    /// The returned bounds maintains the same size and original horizontal position.
    pub fn anchor_bottom(&self, container: &Self) -> Self {
        Self {
            x: self.x,
            y: container.y + container.h as i32 - self.h as i32,
            w: self.w,
            h: self.h,
        }
    }

    /// Positions the current bounds at the center of the provided container bounds.
    /// Returns a new `Bounds` with the same dimensions, repositioned to be centered.
    ///
    /// # Parameters
    /// - `container`: The parent bounds within which to center this bounds.
    ///
    /// # Example
    /// ```
    /// let child = Bounds::new(50, 50);
    /// let parent = Bounds::new(200, 200);
    /// let centered = child.center_in(&parent);
    /// ```
    pub fn anchor_center(&self, container: &Self) -> Self {
        let new_x = container.x + ((container.w as i32 - self.w as i32) / 2);
        let new_y = container.y + ((container.h as i32 - self.h as i32) / 2);
        Self {
            x: new_x,
            y: new_y,
            w: self.w,
            h: self.h,
        }
    }

    /// Aligns the current bounds horizontally within a container.
    /// This method centers the bounds along the x‑axis of the container,
    /// preserving its y position and size.
    pub fn anchor_center_x(&self, container: &Self) -> Self {
        let new_x = container.x + ((container.w as i32 - self.w as i32) / 2);
        Self {
            x: new_x,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }

    /// Aligns the current bounds vertically within a container.
    /// This method centers the bounds along the y‑axis of the container,
    /// preserving its x position and size.
    pub fn anchor_center_y(&self, container: &Self) -> Self {
        let new_y = container.y + ((container.h as i32 - self.h as i32) / 2);
        Self {
            x: self.x,
            y: new_y,
            w: self.w,
            h: self.h,
        }
    }

    /// Returns a new `Bounds` positioned immediately above the container
    pub fn above(&self, container: &Self) -> Self {
        Self {
            x: self.x,
            y: container.y - (self.h as i32),
            w: self.w,
            h: self.h,
        }
    }

    /// Returns a new `Bounds` positioned immediately below the container
    pub fn below(&self, container: &Self) -> Self {
        Self {
            x: self.x,
            y: container.y + container.h as i32,
            w: self.w,
            h: self.h,
        }
    }

    /// Returns a new `Bounds` positioned immediately to the left of the container
    pub fn left_of(&self, container: &Self) -> Self {
        Self {
            x: container.x - self.w as i32,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }

    /// Returns a new `Bounds` positioned immediately to the right of the container
    pub fn right_of(&self, container: &Self) -> Self {
        Self {
            x: container.x + container.w as i32,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }

    /// Returns a new `Bounds` positioned immediately above the current one.
    pub const fn above_self(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - self.h as i32,
            w: self.w,
            h: self.h,
        }
    }

    /// Returns a new `Bounds` positioned immediately below the current one.
    pub const fn below_self(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + self.h as i32,
            w: self.w,
            h: self.h,
        }
    }

    /// Returns a new `Bounds` positioned immediately to the right of the current one.
    pub const fn right_of_self(&self) -> Self {
        Self {
            x: self.x + self.w as i32,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }

    /// Returns a new `Bounds` positioned immediately to the left of the current one,
    /// separated by the specified spacing.
    ///
    /// # Parameters
    /// - `spacing`: The horizontal space between the current bounds and the new one.
    pub const fn left_of_self(&self) -> Self {
        Self {
            x: self.x - self.w as i32,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }

    /// Divides the current bounds horizontally into `count` equal sub-bounds.
    /// Note: Any remainder from integer division is discarded.
    ///
    /// # Returns
    /// A vector containing each sub-bound from left to right.
    pub fn columns(&self, count: u32) -> Vec<Self> {
        let part_w = self.w / count;
        (0..count)
            .map(|i| Self {
                x: self.x + (i * part_w) as i32,
                y: self.y,
                w: part_w,
                h: self.h,
            })
            .collect()
    }

    /// Divides the current bounds vertically into `count` equal sub-bounds.
    /// Note: Any remainder from integer division is discarded.
    ///
    /// # Returns
    /// A vector containing each sub-bound from top to bottom.
    pub fn rows(&self, count: u32) -> Vec<Self> {
        let part_h = self.h / count;
        (0..count)
            .map(|i| Self {
                x: self.x,
                y: self.y + (i * part_h) as i32,
                w: self.w,
                h: part_h,
            })
            .collect()
    }

    /// Divides the current bounds horizontally into `count` sub-bounds with a fixed gap between columns,
    /// ensuring the first column starts at the left edge and the last column ends at the right edge.
    ///
    /// The gap between columns is fixed at `gap` pixels for all columns except possibly the gap
    /// before the last column, which may absorb any extra pixels from integer division.
    ///
    /// # Parameters
    /// - `count`: The number of columns.
    /// - `gap`: The desired gap (in pixels) between adjacent columns.
    ///
    /// # Returns
    /// A vector of sub-bounds from left to right.
    pub fn columns_with_gap(&self, count: u32, gap: u32) -> Vec<Self> {
        if count == 0 {
            return Vec::new();
        }
        if count == 1 {
            return vec![*self];
        }
        // Total fixed gap between columns.
        let total_gap = gap.saturating_mul(count.saturating_sub(1));
        // The width available for columns after subtracting the fixed gaps.
        let available_width = self.w.saturating_sub(total_gap);
        // Compute the base width for each column using integer division.
        let col_w = available_width / count;
        let mut columns = Vec::with_capacity(count as usize);
        for i in 0..count {
            // Compute the x position: For each column, add (base width + gap) for each prior column.
            let x = self.x + (i * (col_w + gap)) as i32;
            // For all but the last column, use the computed base width.
            // For the last column, compute the remaining width to ensure flush alignment.
            let width = if i == count - 1 {
                // Ensure the last column's right edge equals self.x + self.w.
                (self.x + self.w as i32 - x) as u32
            } else {
                col_w
            };
            columns.push(Self {
                x,
                y: self.y,
                w: width,
                h: self.h,
            });
        }
        columns
    }

    /// Divides the current bounds vertically into `count` sub-bounds with a fixed gap between rows,
    /// ensuring the first row starts at the top edge and the last row ends at the bottom edge.
    ///
    /// The gap between rows is fixed at `gap` pixels for all rows except possibly the gap
    /// before the last row, which may absorb any extra pixels from integer division.
    ///
    /// # Parameters
    /// - `count`: The number of rows.
    /// - `gap`: The desired gap (in pixels) between adjacent rows.
    ///
    /// # Returns
    /// A vector of sub-bounds from top to bottom.
    pub fn rows_with_gap(&self, count: u32, gap: u32) -> Vec<Self> {
        if count == 0 {
            return Vec::new();
        }
        if count == 1 {
            return vec![*self];
        }
        // Total fixed gap between rows.
        let total_gap = gap.saturating_mul(count.saturating_sub(1));
        // The height available for rows after subtracting the fixed gaps.
        let available_height = self.h.saturating_sub(total_gap);
        // Compute the base height for each row using integer division.
        let row_h = available_height / count;
        let mut rows = Vec::with_capacity(count as usize);
        for i in 0..count {
            // Compute the y position: For each row, add (base height + gap) for each prior row.
            let y = self.y + (i * (row_h + gap)) as i32;
            // For all but the last row, use the computed base height.
            // For the last row, compute the remaining height to ensure flush alignment.
            let height = if i == count - 1 {
                (self.y + self.h as i32 - y) as u32
            } else {
                row_h
            };
            rows.push(Self {
                x: self.x,
                y,
                w: self.w,
                h: height,
            });
        }
        rows
    }

    /// Divides the current bounds into a grid layout with the specified number of columns and rows.
    /// Returns a one-dimensional vector of grid cell bounds in row-major order.
    ///
    /// # Parameters
    /// - `columns`: Number of columns in the grid.
    /// - `rows`: Number of rows in the grid.
    ///
    /// # Note
    /// Any remainder from integer division is discarded.
    pub fn grid(&self, columns: u32, rows: u32) -> Vec<Self> {
        // Calculate the width and height of each grid cell.
        let cell_w = self.w / columns;
        let cell_h = self.h / rows;
        // Reserve space for all cells.
        let mut cells = Vec::with_capacity((columns * rows) as usize);
        // Iterate over rows and columns to compute each cell's bounds.
        for row in 0..rows {
            for col in 0..columns {
                let cell_x = self.x + (col * cell_w) as i32;
                let cell_y = self.y + (row * cell_h) as i32;
                cells.push(Self {
                    x: cell_x,
                    y: cell_y,
                    w: cell_w,
                    h: cell_h,
                });
            }
        }
        cells
    }

    /// Stacks multiple bounds horizontally within the container.
    /// Each item is placed sequentially from left to right with the given gap,
    /// and is vertically centered within the container.
    ///
    /// # Parameters
    /// - `items`: A slice of bounds to be arranged.
    /// - `gap`: The horizontal spacing between adjacent items.
    ///
    /// # Returns
    /// A vector of bounds with updated positions.
    pub fn stack_ltr<T: NumCast>(&self, items: &[Bounds], gap: T) -> Vec<Bounds> {
        let gap = NumCast::from(gap).unwrap_or(0);
        let mut positioned = Vec::with_capacity(items.len());
        let mut current_x = self.x;
        for item in items {
            // Vertically center each item within the container.
            let new_y = self.y + ((self.h as i32 - item.h as i32) / 2);
            positioned.push(Bounds {
                x: current_x,
                y: new_y,
                w: item.w,
                h: item.h,
            });
            current_x += item.w as i32 + gap;
        }
        positioned
    }

    /// Stacks multiple bounds vertically within the container.
    /// Each item is placed sequentially from top to bottom with the given gap,
    /// and is horizontally centered within the container.
    ///
    /// # Parameters
    /// - `items`: A slice of bounds to be arranged.
    /// - `gap`: The vertical spacing between adjacent items.
    ///
    /// # Returns
    /// A vector of bounds with updated positions.
    pub fn stack_ttb<T: NumCast>(&self, items: &[Bounds], gap: T) -> Vec<Bounds> {
        let gap = NumCast::from(gap).unwrap_or(0);
        let mut positioned = Vec::with_capacity(items.len());
        let mut current_y = self.y;
        for item in items {
            // Horizontally center each item within the container.
            let new_x = self.x + ((self.w as i32 - item.w as i32) / 2);
            positioned.push(Bounds {
                x: new_x,
                y: current_y,
                w: item.w,
                h: item.h,
            });
            current_y += item.h as i32 + gap;
        }
        positioned
    }

    /// Stacks multiple bounds horizontally within the container from right to left.
    /// Each item is placed sequentially from the right edge of the container with the given gap,
    /// and is vertically centered within the container.
    ///
    /// # Parameters
    /// - `items`: A slice of bounds to be arranged in order.
    /// - `gap`: The horizontal spacing between adjacent items.
    ///
    /// # Returns
    /// A vector of bounds with updated positions in right-to-left order.
    pub fn stack_rtl<T: NumCast>(&self, items: &[Bounds], gap: T) -> Vec<Bounds> {
        let gap = NumCast::from(gap).unwrap_or(0);
        let mut positioned = Vec::with_capacity(items.len());
        // Start at the right edge of the container.
        let mut current_x = self.x + self.w as i32;
        for item in items {
            // Position each item so that its right edge aligns with current_x.
            let new_x = current_x - item.w as i32;
            let new_y = self.y + ((self.h as i32 - item.h as i32) / 2);
            positioned.push(Bounds {
                x: new_x,
                y: new_y,
                w: item.w,
                h: item.h,
            });
            // Move left by the width of the item plus the gap.
            current_x = new_x - gap;
        }
        positioned
    }

    /// Stacks multiple bounds vertically within the container from bottom to top.
    /// Each item is placed sequentially from the bottom edge of the container upward with the given gap,
    /// and is horizontally centered within the container.
    ///
    /// # Parameters
    /// - `items`: A slice of bounds to be arranged in order.
    /// - `gap`: The vertical spacing between adjacent items.
    ///
    /// # Returns
    /// A vector of bounds with updated positions in bottom-to-top order.
    pub fn stack_btt<T: NumCast>(&self, items: &[Bounds], gap: T) -> Vec<Bounds> {
        let gap = NumCast::from(gap).unwrap_or(0);
        let mut positioned = Vec::with_capacity(items.len());
        // Start at the bottom edge of the container.
        let mut current_y = self.y + self.h as i32;
        for item in items {
            // Position each item so that its bottom edge aligns with current_y.
            let new_y = current_y - item.h as i32;
            let new_x = self.x + ((self.w as i32 - item.w as i32) / 2);
            positioned.push(Bounds {
                x: new_x,
                y: new_y,
                w: item.w,
                h: item.h,
            });
            // Move upward by the height of the item plus the gap.
            current_y = new_y - gap;
        }
        positioned
    }

    /// Consumes this `Bounds` and applies a generic mapping function,
    /// returning a value of type `T`.
    ///
    /// This method allows you to transform a `Bounds` into any type `T` by
    /// providing a callback that takes ownership of the bounds.
    ///
    /// # Example
    /// ```
    /// let b = Bounds::with_size(100, 50).position(10, 20);
    /// // Compute the area using the map function.
    /// let area: u32 = b.map(|bounds| bounds.w * bounds.h);
    /// ```
    pub fn map<T, F>(self, f: F) -> T
    where
        F: FnOnce(Self) -> T,
    {
        f(self)
    }
}

impl Add for Bounds {
    type Output = Bounds;
    fn add(self, rhs: Bounds) -> Bounds {
        Bounds {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            w: self.w + rhs.w,
            h: self.h + rhs.h,
        }
    }
}
