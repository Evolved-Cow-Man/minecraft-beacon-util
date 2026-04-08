use std::f64::consts::PI;

pub struct Position {
    pub x: i16,
    pub y: i16,
}

fn normalize_from_100(new_min: f64, new_max: f64, input: u8) -> f64 {
    let old_min: f64 = 0.0;
    let old_max: f64 = 100.0;

    new_min + (f64::from(input) - old_min) * (new_max - new_min) / (old_max - old_min)
}

pub fn generate_uniform_colors(
    size: u16,
    user_lightness: u8,
    user_chroma: u8,
    chroma_phase_degrees: i16,
    clockwise_hue: bool,
) -> Vec<oklab::Oklab> {
    // target: 0.63
    let min_lightness = 0.232_449; // the lightness of black stained glass
    let lightness = normalize_from_100(min_lightness, 1.0, user_lightness);

    // target: 0.058
    let max_chroma = 0.204_371_487_954_799_45; // the maximum chroma seen when trying to replicate every RGB value with stained glass
    let chroma = normalize_from_100(0.0, max_chroma, user_chroma);
    let chroma_phase_radians = f64::from(chroma_phase_degrees).to_radians();

    let mut color_vec = vec![];

    for i in 0..size {
        // reassign i to a value that can be negative, flip if clockwise_hue
        let i = if clockwise_hue {
            -i32::from(i)
        } else {
            i32::from(i)
        };
        // if the hue is clockwise we need to reverse which direction the phase offset goes
        let theta = if clockwise_hue {
            (2.0 * PI / f64::from(size)).mul_add(f64::from(i), chroma_phase_radians)
        } else {
            (2.0 * PI / f64::from(size)).mul_add(f64::from(i), -chroma_phase_radians)
        };

        let x = theta.cos() * chroma;
        let y = theta.sin() * chroma;
        #[allow(clippy::cast_possible_truncation)]
        let oklab_color = oklab::Oklab {
            l: lightness as f32,
            a: x as f32,
            b: y as f32,
        };
        color_vec.push(oklab_color);
    }

    color_vec
}

pub fn generate_uniform_positions(
    size: u16,
    user_min_radius: u16,
    user_max_radius: u16,
    position_phase_degrees: i16,
) -> Vec<Position> {
    let min_radius = if user_min_radius <= user_max_radius {
        i32::from(user_min_radius)
    } else {
        i32::from(user_max_radius)
    };
    let max_radius = i32::from(user_max_radius);

    let position_phase_radians = f64::from(position_phase_degrees).to_radians();

    let mut position_vec = vec![];

    // 1 is every whole value. 10 is every 0.1
    let resolution = 10;
    let resolution_min_radius = min_radius * resolution;
    let resolution_max_radius = max_radius * resolution;
    let middle_value = i32::midpoint(resolution_min_radius, resolution_max_radius);

    for i in 0..size {
        let theta = (2.0 * PI / f64::from(size)).mul_add(f64::from(i), position_phase_radians);

        let mut best_distance = f64::MAX;
        // so long as the calculated distance below is less than the f64
        // max this initial value won't be used
        let mut position = Position { x: 0, y: 0 };
        for distance_from_middle in 0..=resolution_max_radius - middle_value {
            for add in [true, false] {
                let hypotenuse = if add {
                    (f64::from(middle_value) + f64::from(distance_from_middle))
                        / f64::from(resolution)
                } else {
                    (f64::from(middle_value) - f64::from(distance_from_middle))
                        / f64::from(resolution)
                };
                let x = theta.cos() * hypotenuse;
                let y = theta.sin() * hypotenuse;
                let x_rounded = x.round();
                let y_rounded = y.round();
                let distance = (x - x_rounded).hypot(y - y_rounded);
                // if our new distance is better than our best by some margin
                let margin = 0.0005;
                #[allow(clippy::cast_possible_truncation)]
                if distance + margin < best_distance {
                    best_distance = distance;
                    position = Position {
                        x: x_rounded as i16,
                        y: y_rounded as i16,
                    };
                }
            }
        }
        position_vec.push(position);
    }
    position_vec
}
