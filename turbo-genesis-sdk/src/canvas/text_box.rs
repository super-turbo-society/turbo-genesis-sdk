use super::{flags, quad, utils};
use crate::bounds;
use num_traits::NumCast;
use quad::Quad;

#[derive(Debug, Clone, Copy)]
pub enum Align {
    Left,
    Center,
    Right,
}
impl Align {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "left" => Some(Self::Left),
            "center" => Some(Self::Center),
            "right" => Some(Self::Right),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TextBox<'a> {
    text: &'a str,
    font: &'a str,
    scale: f32,
    quad: Quad,
    align: Align,
    start: usize,
    end: usize,
    preserve_spaces: bool,
    preserve_tabs: bool,
    preserve_newlines: bool,
}

impl<'a> TextBox<'a> {
    /// Create a new TextBox with default font, scale, alignment, and quad.
    pub fn new(text: &'a str) -> Self {
        let bounds = bounds::screen();
        let (default_w, default_h) = bounds.wh();
        Self {
            text,
            font: "medium",
            scale: 1.0,
            quad: Quad::default().size(default_w, default_h),
            align: Align::Left,
            start: 0,
            end: text.len(),
            preserve_spaces: true,
            preserve_tabs: true,
            preserve_newlines: true,
        }
    }

    /// Whether or not to preserve spaces when drawing text.
    pub fn preserve_spaces(mut self, preserve_spaces: bool) -> Self {
        self.preserve_spaces = preserve_spaces;
        self
    }

    /// Whether or not to preserve spaces when drawing text.
    pub fn set_preserve_spaces(&mut self, preserve_spaces: bool) {
        self.preserve_spaces = preserve_spaces;
    }

    /// Whether or not to preserve tabs when drawing text.
    pub fn preserve_tabs(mut self, preserve_tabs: bool) -> Self {
        self.preserve_tabs = preserve_tabs;
        self
    }

    /// Whether or not to preserve tabs when drawing text.
    pub fn set_preserve_tabs(&mut self, preserve_tabs: bool) {
        self.preserve_tabs = preserve_tabs;
    }

    /// Whether or not to preserve newlines when drawing text.
    pub fn preserve_newlines(mut self, preserve_newlines: bool) -> Self {
        self.preserve_newlines = preserve_newlines;
        self
    }

    /// Whether or not to preserve newlines when drawing text.
    pub fn set_preserve_newlines(&mut self, preserve_newlines: bool) {
        self.preserve_newlines = preserve_newlines;
    }

    /// Whether or not to preserve all whitespace when drawing text.
    pub fn preserve_whitespace(mut self, preserve_whitespace: bool) -> Self {
        self.preserve_spaces = preserve_whitespace;
        self.preserve_tabs = preserve_whitespace;
        self.preserve_newlines = preserve_whitespace;
        self
    }

    /// Whether or not to preserve all whitespace when drawing text.
    pub fn set_preserve_whitespace(&mut self, preserve_whitespace: bool) {
        self.preserve_spaces = preserve_whitespace;
        self.preserve_tabs = preserve_whitespace;
        self.preserve_newlines = preserve_whitespace;
    }

    /// Ignore camera when drawing.
    pub fn fixed(mut self, fixed: bool) -> Self {
        self.quad = self.quad.fixed(fixed);
        self
    }

    /// Ignore camera when drawing.
    pub fn set_fixed(&mut self, fixed: bool) {
        self.quad.fixed = fixed;
    }

    /// Font name.
    pub fn font(mut self, font: &'a str) -> Self {
        self.font = font;
        self
    }

    /// Font scale.
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    /// Text alignment within the box.
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Position the box.
    pub fn position<X: NumCast, Y: NumCast>(mut self, x: X, y: Y) -> Self {
        let x = NumCast::from(x).unwrap_or(self.quad.x);
        let y = NumCast::from(y).unwrap_or(self.quad.y);
        self.quad = self.quad.position(x, y);
        self
    }
    pub fn set_position<X: NumCast, Y: NumCast>(&mut self, x: X, y: Y) {
        let x = NumCast::from(x).unwrap_or(self.quad.x);
        let y = NumCast::from(y).unwrap_or(self.quad.y);
        self.quad = self.quad.position(x, y);
    }

    /// Position X only.
    pub fn position_x<X: NumCast>(mut self, x: X) -> Self {
        let x = NumCast::from(x).unwrap_or(self.quad.x);
        self.quad = self.quad.position(x, self.quad.y);
        self
    }
    pub fn set_position_x<X: NumCast>(&mut self, x: X) {
        let x = NumCast::from(x).unwrap_or(self.quad.x);
        self.quad = self.quad.position(x, self.quad.y);
    }

    /// Position Y only.
    pub fn position_y<Y: NumCast>(mut self, y: Y) -> Self {
        let y = NumCast::from(y).unwrap_or(self.quad.y);
        self.quad = self.quad.position(self.quad.x, y);
        self
    }
    pub fn set_position_y<Y: NumCast>(&mut self, y: Y) {
        let y = NumCast::from(y).unwrap_or(self.quad.y);
        self.quad = self.quad.position(self.quad.x, y);
    }

    /// Position both.
    pub fn position_xy<X: NumCast, Y: NumCast>(mut self, (x, y): (X, Y)) -> Self {
        let x = NumCast::from(x).unwrap_or(self.quad.x);
        let y = NumCast::from(y).unwrap_or(self.quad.y);
        self.quad = self.quad.position(x, y);
        self
    }
    pub fn set_position_xy<X: NumCast, Y: NumCast>(&mut self, (x, y): (X, Y)) {
        let x = NumCast::from(x).unwrap_or(self.quad.x);
        let y = NumCast::from(y).unwrap_or(self.quad.y);
        self.quad = self.quad.position(x, y);
    }

    /// Box size.
    pub fn size<W: NumCast, H: NumCast>(mut self, w: W, h: H) -> Self {
        let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
        let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
        self.quad = self.quad.size(w, h);
        self
    }
    pub fn set_size<W: NumCast, H: NumCast>(&mut self, w: W, h: H) {
        let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
        let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
        self.quad = self.quad.size(w, h);
    }
    pub fn set_size_w<W: NumCast>(&mut self, w: W) {
        let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
        self.quad = self.quad.size(w, self.quad.h);
    }
    pub fn size_h<H: NumCast>(mut self, h: H) -> Self {
        let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
        self.quad = self.quad.size(self.quad.w, h);
        self
    }
    pub fn set_size_h<H: NumCast>(&mut self, h: H) {
        let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
        self.quad = self.quad.size(self.quad.w, h);
    }
    pub fn size_wh<W: NumCast, H: NumCast>(mut self, (w, h): (W, H)) -> Self {
        let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
        let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
        self.quad = self.quad.size(w, h);
        self
    }
    pub fn set_size_wh<W: NumCast, H: NumCast>(&mut self, (w, h): (W, H)) {
        let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
        let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
        self.quad = self.quad.size(w, h);
    }
    pub fn width<W: NumCast>(mut self, w: W) -> Self {
        let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
        self.quad = self.quad.size(w, self.quad.h);
        self
    }
    pub fn set_width<W: NumCast>(&mut self, w: W) {
        let w: u32 = NumCast::from(w).unwrap_or(self.quad.w);
        self.quad = self.quad.size(w, self.quad.h);
    }
    pub fn height<H: NumCast>(mut self, h: H) -> Self {
        let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
        self.quad = self.quad.size(self.quad.w, h);
        self
    }
    pub fn set_height<H: NumCast>(&mut self, h: H) {
        let h: u32 = NumCast::from(h).unwrap_or(self.quad.h);
        self.quad = self.quad.size(self.quad.w, h);
    }

    /// Offset the box.
    pub fn offset<DX: NumCast, DY: NumCast>(mut self, dx: DX, dy: DY) -> Self {
        let dx = NumCast::from(dx).unwrap_or(0);
        let dy = NumCast::from(dy).unwrap_or(0);
        self.quad = self.quad.offset(dx, dy);
        self
    }
    pub fn set_offset<DX: NumCast, DY: NumCast>(&mut self, dx: DX, dy: DY) {
        let dx = NumCast::from(dx).unwrap_or(0);
        let dy = NumCast::from(dy).unwrap_or(0);
        self.quad = self.quad.offset(dx, dy);
    }

    /// Text color (uses quad.color).
    pub fn color(mut self, col: u32) -> Self {
        self.quad = self.quad.color(col);
        self
    }
    pub fn set_color(&mut self, col: u32) {
        self.quad = self.quad.color(col);
    }

    /// Box opacity.
    pub fn opacity(mut self, o: f32) -> Self {
        self.quad = self.quad.opacity(o);
        self
    }
    pub fn set_opacity(&mut self, o: f32) {
        self.quad = self.quad.opacity(o);
    }

    // Start (inclusive) and end (exclusive)
    pub fn start(mut self, start: usize) -> Self {
        self.start = start;
        self
    }
    pub fn set_start(&mut self, start: usize) {
        self.start = start;
    }
    pub fn end(mut self, end: usize) -> Self {
        self.end = end.min(self.text.len());
        self
    }
    pub fn set_end(&mut self, end: usize) {
        self.end = end.min(self.text.len());
    }

    /// Box rotation around its origin.
    pub fn rotation_deg<A: NumCast>(mut self, deg: A) -> Self {
        let deg = NumCast::from(deg).unwrap_or(0);
        self.quad = self.quad.rotation(deg);
        self
    }
    pub fn set_rotation_deg<A: NumCast>(&mut self, deg: A) {
        let deg = NumCast::from(deg).unwrap_or(0);
        self.quad = self.quad.rotation(deg);
    }

    /// Wrap text to lines that fit `max_width`.
    fn wrap_lines(&self, max_width: f32) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current = String::new();

        // UTF-8-safe slicing
        let text = self
            .text
            .char_indices()
            .skip_while(|(i, _)| *i < self.start)
            .take_while(|(i, _)| *i < self.end)
            .map(|(_, c)| c)
            .collect::<String>();

        // Simple tokenization - just split into words and whitespace
        let tokens: Vec<String> = {
            let mut out = vec![];
            let mut word = String::new();

            for c in text.chars() {
                match c {
                    ' ' | '\t' | '\n' => {
                        if !word.is_empty() {
                            out.push(word.clone());
                            word.clear();
                        }
                        if c == '\n' && !self.preserve_newlines {
                            out.push(' '.to_string());
                        } else {
                            out.push(c.to_string());
                        }
                    }
                    _ => word.push(c),
                }
            }

            if !word.is_empty() {
                out.push(word);
            }

            // Clean up consecutive whitespace based on preservation settings
            let mut cleaned = vec![];
            let mut prev_was_space = false;
            let mut prev_was_tab = false;

            for token in out {
                match token.as_str() {
                    " " => {
                        if self.preserve_spaces {
                            cleaned.push(token);
                            prev_was_space = true;
                            prev_was_tab = false;
                        } else {
                            // Deduplicate spaces
                            if !prev_was_space && !cleaned.is_empty() {
                                cleaned.push(" ".to_string());
                            }
                            prev_was_space = true;
                            prev_was_tab = false;
                        }
                    }
                    "\t" => {
                        if self.preserve_tabs {
                            cleaned.push("\t".to_string());
                            prev_was_space = false;
                            prev_was_tab = true;
                        } else {
                            // Deduplicate tabs
                            if !prev_was_tab && !cleaned.is_empty() {
                                cleaned.push("\t".to_string());
                            }
                            prev_was_space = false;
                            prev_was_tab = true;
                        }
                    }
                    _ => {
                        cleaned.push(token);
                        prev_was_space = false;
                        prev_was_tab = false;
                    }
                }
            }

            cleaned
        };

        // Build lines
        for token in tokens {
            if token == "\n" {
                lines.push(current.clone());
                current.clear();
                continue;
            }

            let candidate = if current.is_empty() {
                token.clone()
            } else {
                format!("{}{}", current, token)
            };

            let (w, _) = utils::text::measure(self.font, self.scale, &candidate);

            if w <= max_width {
                current = candidate;
            } else {
                if !current.is_empty() {
                    match self.align {
                        Align::Left => {
                            if !self.preserve_spaces {
                                current = current.trim_start_matches(' ').to_string();
                            }
                            if !self.preserve_tabs {
                                current = current.trim_start_matches('\t').to_string();
                            }
                        }
                        Align::Center => {
                            if !self.preserve_spaces {
                                current = current.trim_matches(' ').to_string();
                            }
                            if !self.preserve_tabs {
                                current = current.trim_matches('\t').to_string();
                            }
                        }
                        Align::Right => {
                            if !self.preserve_spaces {
                                current = current.trim_end_matches(' ').to_string();
                            }
                            if !self.preserve_tabs {
                                current = current.trim_end_matches('\t').to_string();
                            }
                        }
                    };
                    lines.push(current.clone());
                }
                current = token;
            }
        }

        if !current.is_empty() {
            match self.align {
                Align::Left => {
                    if !self.preserve_spaces {
                        current = current.trim_start_matches(' ').to_string();
                    }
                    if !self.preserve_tabs {
                        current = current.trim_start_matches('\t').to_string();
                    }
                }
                Align::Center => {
                    if !self.preserve_spaces {
                        current = current.trim_matches(' ').to_string();
                    }
                    if !self.preserve_tabs {
                        current = current.trim_matches('\t').to_string();
                    }
                }
                Align::Right => {
                    if !self.preserve_spaces {
                        current = current.trim_end_matches(' ').to_string();
                    }
                    if !self.preserve_tabs {
                        current = current.trim_end_matches('\t').to_string();
                    }
                }
            };
            lines.push(current);
        }

        lines
    }

    /// Draw by rendering each glyph sprite via `utils::sprite::draw`.
    pub fn draw(&self) {
        let flags = if self.quad.fixed {
            flags::POSITION_FIXED
        } else {
            0
        };
        let x0 = self.quad.x;
        let y0 = self.quad.y;
        let box_w = self.quad.w as i32;
        let box_h = self.quad.h as i32;

        // approximate line height
        let (_, line_h) = utils::text::measure(self.font, self.scale, "M");
        let mut y = y0 as f32;

        let draw_color = utils::color::apply_opacity(self.quad.color, self.quad.opacity);
        let rotation = self.quad.rotation_deg;
        let origin_x = self.quad.origin_x;
        let origin_y = self.quad.origin_y;

        let mut num_chars = 0;
        for line in self.wrap_lines(self.quad.w as f32) {
            if y > y0 as f32 + box_h as f32 {
                break;
            }
            // compute starting x based on alignment
            let line_w = utils::text::measure(self.font, self.scale, &line).0;
            let mut x = match self.align {
                Align::Left => x0 as f32,
                Align::Center => x0 as f32 + ((box_w as f32 - line_w) * 0.5),
                Align::Right => x0 as f32 + (box_w as f32 - line_w),
            };

            for ch in line.chars() {
                if num_chars > self.end {
                    break;
                }
                num_chars += 1;
                // Use space char for tabs
                let key = format!("font_{}_{}", self.font, if ch == '\t' { ' ' } else { ch });
                if let Some(glyph) = utils::sprite::get_source_data(&key) {
                    let dw = (glyph.width as f32 * self.scale) as u32;
                    let dh = (glyph.height as f32 * self.scale) as u32;
                    let sx = glyph.x;
                    let sy = glyph.y;
                    let sw = glyph.width as i32;
                    let sh = glyph.height as i32;
                    let dx = x as i32;
                    let dy = y as i32;

                    // compute how many pixels the glyph extends beyond each edge
                    let left_over = x0 - dx;
                    let right_over = dx + dw as i32 - (x0 + box_w);
                    // let top_over = y0 - dy;
                    let bottom_over = dy + dh as i32 - (y0 + box_h);
                    let glyph_w = dw;
                    let (dx, _dyh, dw, dh, tx, ty) = {
                        let dh = dh - (bottom_over as u32);
                        let dw = dw - (right_over as u32);
                        (dx + left_over, dy, dw - left_over as u32, dh, -left_over, 0)
                    };
                    utils::sprite::draw(
                        dx, dy, dw, dh, sx, sy, sw, sh, tx, ty, draw_color, 0, 0, 0, 0, origin_x,
                        origin_y, rotation, flags,
                    );
                    x += glyph_w as f32;
                }
            }
            y += line_h;
        }
    }
}
