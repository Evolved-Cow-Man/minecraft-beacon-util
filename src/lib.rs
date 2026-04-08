use wasm_bindgen::prelude::*;
mod color_geometry;
mod minecraft_color;

#[must_use]
#[wasm_bindgen]
pub fn generate_positions(
    size: u16,
    user_min_radius: u16,
    user_max_radius: u16,
    position_phase_degrees: i16,
) -> Vec<i16> {
    let mut output = Vec::new();
    let positions = color_geometry::generate_uniform_positions(
        size,
        user_min_radius,
        user_max_radius,
        position_phase_degrees,
    );
    for position in positions {
        output.push(position.x);
        output.push(position.y);
    }
    output
}

#[must_use]
#[wasm_bindgen]
pub fn generate_colors(
    size: u16,
    user_lightness: u8,
    user_chroma: u8,
    chroma_phase_degrees: i16,
    clockwise_hue: bool,
) -> Vec<String> {
    let mut output = Vec::new();

    let oklab_vec = color_geometry::generate_uniform_colors(
        size,
        user_lightness,
        user_chroma,
        chroma_phase_degrees,
        clockwise_hue,
    );

    for oklab in oklab_vec {
        let target_rgb = oklab.to_srgb();
        let glass_stack = minecraft_color::best_guess_rgb(target_rgb);
        let actual_rgb = minecraft_color::rgb_from_stack(&glass_stack);

        let r_hex = format!("{:02x}", actual_rgb.r);
        let g_hex = format!("{:02x}", actual_rgb.g);
        let b_hex = format!("{:02x}", actual_rgb.b);
        let hex_string = format!("#{r_hex}{g_hex}{b_hex}");
        output.push(hex_string);

        for glass in glass_stack {
            output.push(glass.name().to_string());
        }
    }

    output
}
