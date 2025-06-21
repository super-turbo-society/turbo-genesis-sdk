#[allow(arithmetic_overflow)]
pub fn apply_opacity(color: u32, opacity: f32) -> u32 {
    // Fully opaque
    if opacity >= 1.0 {
        return 255 << 32 | (color & 0xffffff00)
    }

    // Apply gamma correction
    let gamma = 2.2;
    let linear_opacity = opacity.powf(1.0 / gamma);

    // Calculate the alpha value
    let alpha = (255.0 * linear_opacity) as u32;

    // Combine the alpha with the color
    alpha << 32 | (color & 0xffffff00)
}
