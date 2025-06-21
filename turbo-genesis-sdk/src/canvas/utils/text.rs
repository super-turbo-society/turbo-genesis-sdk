use super::sprite;

pub fn draw(
    font_name: &str,
    text: &str,
    x: i32,
    y: i32,
    color: u32,
    scale: f32,
    rotation: f32,
    flags: u32,
) {
    let font_name_ptr = font_name.as_ptr();
    let font_name_len = font_name.len() as u32;
    let text_ptr = text.as_ptr();
    let text_len = text.len() as u32;
    turbo_genesis_ffi::canvas::text(
        x,
        y,
        color,
        scale,
        rotation,
        font_name_ptr,
        font_name_len,
        text_ptr,
        text_len,
        flags,
    )
}

/// Measure pixel width and height of `text` for a given `font` and `scale`.
///
/// Looks up each glyph as a sprite named "font_{font}_{ch}" and sums widths per line,
/// using the maximum glyph height for line height. Lines split on '\n'.
pub fn measure(font: &str, scale: f32, text: &str) -> (f32, f32) {
    let mut max_line_width = 0.0_f32;
    let mut current_width = 0.0_f32;
    let mut max_glyph_height = 0.0_f32;
    for ch in text.chars() {
        if ch == '\n' {
            if current_width > max_line_width {
                max_line_width = current_width;
            }
            current_width = 0.0;
        } else {
            let key = format!("font_{}_{}", font, ch);
            if let Some(src) = sprite::get_source_data(&key) {
                let w = src.width as f32 * scale;
                let h = src.height as f32 * scale;
                current_width += w;
                if h > max_glyph_height {
                    max_glyph_height = h;
                }
            }
        }
    }
    if current_width > max_line_width {
        max_line_width = current_width;
    }
    let lines = text.lines().count() as f32;
    let total_height = max_glyph_height * lines;
    (max_line_width, total_height)
}
