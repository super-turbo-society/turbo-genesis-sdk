#[allow(arithmetic_overflow)]
pub fn apply_opacity(color: u32, opacity: f32) -> u32 {
    // Clamp opacity
    let opacity = opacity.clamp(0.0, 1.0);

    // Extract original alpha (lowest byte)
    let original_alpha = color & 0xff;

    // Gamma correction
    let gamma = 2.2;
    let linear_opacity = opacity.powf(1.0 / gamma);

    // Apply relative scaling
    let new_alpha = (original_alpha as f32 * linear_opacity).round() as u32;

    // Return color with adjusted alpha (replace lowest byte)
    (color & 0xffffff00) | new_alpha
}
