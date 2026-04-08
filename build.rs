#![allow(dead_code)]

#[path = "src/color_geometry.rs"]
mod color_geometry;
#[path = "src/minecraft_color.rs"]
mod minecraft_color;

use std::io::Write;

fn main() {
    std::fs::create_dir_all("./pkg").unwrap(); // makes path
    let file_path = "./pkg/color_accuracy_data.js";

    if std::path::Path::new(file_path).exists() {
        // skip if the file exists
        return;
    }

    let file = std::fs::File::create(file_path).unwrap(); // overwrites file
    let mut writer = std::io::BufWriter::new(file);

    let lightness_max = 100;
    let chroma_max = 100;
    writeln!(writer, "const color_accuracy_data = [").unwrap();
    for lightness in 0..=lightness_max {
        write!(writer, "    [").unwrap();
        for chroma in 0..=chroma_max {
            let colors = color_geometry::generate_uniform_colors(100, lightness, chroma, 0, false);
            let mut valid_count = 0;
            for color in colors {
                if stack_possible_oklab(color) {
                    valid_count += 1;
                }
            }
            write!(writer, "{valid_count}").unwrap();

            if chroma != chroma_max {
                write!(writer, ", ").unwrap();
            }
        }
        write!(writer, "]").unwrap();
        if lightness != lightness_max {
            write!(writer, ",").unwrap();
        }
        writeln!(writer).unwrap();
    }
    writeln!(writer, "]").unwrap();
}

const JUST_NOTICEABLE_DIFFERENCE: f64 = 0.023; // CIELAB is 2.3. Oklab is 100 times smaller

fn oklab_perceived_distance(value_1: oklab::Oklab, value_2: oklab::Oklab) -> f64 {
    let dl = f64::from(value_1.l) - f64::from(value_2.l);
    let da = f64::from(value_1.a) - f64::from(value_2.a);
    let db = f64::from(value_1.b) - f64::from(value_2.b);
    #[allow(clippy::suboptimal_flops)]
    let euclidean_distance = (dl.powi(2) + da.powi(2) + db.powi(2)).sqrt();
    euclidean_distance / JUST_NOTICEABLE_DIFFERENCE // perceptual distance
}

fn rgb_perceived_distance(value_1: oklab::Rgb<u8>, value_2: oklab::Rgb<u8>) -> f64 {
    let oklab_1 = oklab::srgb_to_oklab(value_1);
    let oklab_2 = oklab::srgb_to_oklab(value_2);

    oklab_perceived_distance(oklab_1, oklab_2)
}

fn stack_possible_oklab(oklab: oklab::Oklab) -> bool {
    // is this color possible in rgb?
    let rgb = oklab::oklab_to_srgb(oklab);
    let okalb_after = oklab::srgb_to_oklab(rgb);
    let oklab_perceived_difference = oklab_perceived_distance(oklab, okalb_after);

    let stack = minecraft_color::best_guess_rgb(rgb);
    let stack_rgb = minecraft_color::rgb_from_stack(&stack);
    let rgb_perceived_difference = rgb_perceived_distance(rgb, stack_rgb);

    let max_diff = 1.0; // 1.0 means just one noticeable difference
    oklab_perceived_difference < max_diff && rgb_perceived_difference < max_diff
}
