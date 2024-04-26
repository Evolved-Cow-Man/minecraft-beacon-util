use oklab::*;
use std::collections::BTreeMap;
use std::f64::consts::PI;
use MinecraftColor::*;

fn deg_to_rad(deg: f64) -> f64 {
    deg / 180.0 * PI
}

fn user_input_number() -> i32 {
    loop {
        let mut input_text = String::new();

        std::io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read line");

        match input_text.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid number!");
                // The loop will continue to prompt the user again
            }
        }
    }
}

fn print_color(colors: Vec<MinecraftColor>) {
    for color in colors {
        match color {
            White => print!("White"),
            LightGray => print!("Light gray"),
            Gray => print!("Gray"),
            Black => print!("Black"),
            Brown => print!("Brown"),
            Red => print!("Red"),
            Orange => print!("Orange"),
            Yellow => print!("Yellow"),
            Lime => print!("Lime"),
            Green => print!("Green"),
            Cyan => print!("Cyan"),
            LightBlue => print!("LightBlue"),
            Blue => print!("Blue"),
            Purple => print!("Purple"),
            Magenta => print!("Magenta"),
            Pink => print!("Pink"),
        }
        print!(", ");
    }
    println!();
}

fn rgb_distance(first: &RGB<u8>, second: &RGB<u8>) -> f64 {
    let hyp1: f64 = ((first.r as f64 - second.r as f64).powi(2)
        + (first.g as f64 - second.g as f64).powi(2))
    .sqrt();
    let hyp2: f64 = (hyp1.powi(2) + (first.b as f64 - second.b as f64).powi(2)).sqrt();
    hyp2
}

fn lab_distance(first: &RGB<u8>, second: &RGB<u8>) -> f64 {
    let first = srgb_to_oklab(*first);
    let second = srgb_to_oklab(*second);
    let hyp1: f64 = ((first.l as f64 - second.l as f64).powi(2)
        + (first.a as f64 - second.a as f64).powi(2))
    .sqrt();
    let hyp2: f64 = (hyp1.powi(2) + (first.b as f64 - second.b as f64).powi(2)).sqrt();
    hyp2
}

fn mean(data: &[f64]) -> Option<f64> {
    let sum = data.iter().sum::<f64>() as f64;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

fn std_deviation(data: &[f64]) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f64);

                diff * diff
            }).sum::<f64>() / count as f64;

            Some(variance.sqrt())
        },
        _ => None
    }
}

#[derive(Clone, Copy)]
enum MinecraftColor {
    White,
    LightGray,
    Gray,
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Lime,
    Green,
    Cyan,
    LightBlue,
    Blue,
    Purple,
    Magenta,
    Pink,
}

const fn mc_color_to_rgb(mc_color: MinecraftColor) -> RGB<u8> {
    match mc_color {
        MinecraftColor::White => RGB::<u8> {
            r: 249,
            g: 255,
            b: 254,
        },
        MinecraftColor::LightGray => RGB::<u8> {
            r: 157,
            g: 157,
            b: 151,
        },
        MinecraftColor::Gray => RGB::<u8> {
            r: 71,
            g: 79,
            b: 82,
        },
        MinecraftColor::Black => RGB::<u8> {
            r: 29,
            g: 29,
            b: 33,
        },
        MinecraftColor::Brown => RGB::<u8> {
            r: 131,
            g: 84,
            b: 50,
        },
        MinecraftColor::Red => RGB::<u8> {
            r: 176,
            g: 46,
            b: 38,
        },
        MinecraftColor::Orange => RGB::<u8> {
            r: 249,
            g: 128,
            b: 29,
        },
        MinecraftColor::Yellow => RGB::<u8> {
            r: 254,
            g: 216,
            b: 61,
        },
        MinecraftColor::Lime => RGB::<u8> {
            r: 128,
            g: 199,
            b: 31,
        },
        MinecraftColor::Green => RGB::<u8> {
            r: 94,
            g: 124,
            b: 22,
        },
        MinecraftColor::Cyan => RGB::<u8> {
            r: 22,
            g: 156,
            b: 156,
        },
        MinecraftColor::LightBlue => RGB::<u8> {
            r: 58,
            g: 179,
            b: 218,
        },
        MinecraftColor::Blue => RGB::<u8> {
            r: 60,
            g: 68,
            b: 170,
        },
        MinecraftColor::Purple => RGB::<u8> {
            r: 137,
            g: 50,
            b: 184,
        },
        MinecraftColor::Magenta => RGB::<u8> {
            r: 199,
            g: 78,
            b: 189,
        },
        MinecraftColor::Pink => RGB::<u8> {
            r: 243,
            g: 139,
            b: 170,
        },
    }
}

fn blocks_to_color(blocks: Vec<MinecraftColor>) -> RGB<u8> {
    fn calculate_channel_avg(channel: Vec<u8>) -> f64 {
        let n = channel.len() - 1; // Since channel contains c_0 to c_n, n is length - 1.
        let mut sum: f64 = 0.0; // Initialize sum as a floating-point number.
        let c_0 = channel[0] as f64; // Convert the first element to f64.

        // Calculate the summation part.
        for (i, &c_i) in channel.iter().enumerate().skip(1) {
            // Skip the first element.
            sum += 2f64.powi((i - 1) as i32) * c_i as f64;
        }

        // Calculate the final value of c.
        let c = (c_0 + sum) / 2f64.powi((n) as i32);

        c // Return the result.
    }

    let mut color_channel_r = Vec::new();
    let mut color_channel_g = Vec::new();
    let mut color_channel_b = Vec::new();

    for block in blocks {
        let rgb = mc_color_to_rgb(block);
        // Convert u8 color values to f64 and push them to the vector
        color_channel_r.push(rgb.r);
        color_channel_g.push(rgb.g);
        color_channel_b.push(rgb.b);
    }

    // Now you have a Vec<f64> with color channels, you can use it as &[f64]
    let r_final = calculate_channel_avg(color_channel_r) as u8;
    let g_final = calculate_channel_avg(color_channel_g) as u8;
    let b_final = calculate_channel_avg(color_channel_b) as u8;

    RGB::<u8> {
        r: r_final,
        g: g_final,
        b: b_final,
    }
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_lossless)]
fn main() {
    println!("Total beacons:");
    let total_beacons = user_input_number();
    let deg_per_beacon = 360.0 / total_beacons as f64;
    println!("deg_per_beacon: {deg_per_beacon}");
    let rad_per_beacon = deg_to_rad(360.0) / total_beacons as f64;
    println!("rad_per_beacon: {rad_per_beacon}");

    println!("Minimum distance to beacon:");
    let beacon_distance_min = user_input_number() as f64;

    println!("Maximum distance to beacon:");
    let beacon_distance_max = user_input_number() as f64;

    #[allow(clippy::items_after_statements)]
    struct BeaconData {
        rad: f64,
        x: f64,
        y: f64,
        delta: f64,
    }

    let mut beacon_data: BTreeMap<i32, BeaconData> = BTreeMap::new();

    let mut current_beacon = 0;
    let mut current_beacon_rad: f64 = 0.0;

    while current_beacon < total_beacons {
        let mut current_beacon_distance = beacon_distance_min;

        while current_beacon_distance < beacon_distance_max {
            let exact_x = current_beacon_rad.cos() * current_beacon_distance;
            let exact_y = current_beacon_rad.sin() * current_beacon_distance;

            let round_x = exact_x.round();
            let round_y = exact_y.round();

            let current_delta = ((exact_x - round_x).powi(2) + (exact_y - round_y).powi(2)).sqrt();

            let possible_entry = BeaconData {
                rad: current_beacon_rad,
                x: round_x,
                y: round_y,
                delta: current_delta,
            };

            let old_delta;

            if beacon_data.contains_key(&current_beacon) {
                old_delta = beacon_data.get(&current_beacon).unwrap().delta;
                if current_delta < old_delta {
                    beacon_data.insert(current_beacon, possible_entry);
                }
            } else {
                beacon_data.insert(current_beacon, possible_entry);
            }

            current_beacon_distance += 0.1;
        }

        current_beacon += 1;
        current_beacon_rad += rad_per_beacon;
    }

    /*
    Color time
    */
    for luminance_num in 0..=10 {
    let luminance = (luminance_num as f32/10.0);

    for saturation_num in 0..=10 {
    let saturation = (saturation_num as f32/10.0);

    //select all needed colors
    let mut beacon_colors: BTreeMap<i32, RGB<u8>> = BTreeMap::new();

    let mut current_color_number = 0;
    let mut current_color_rad: f64 = 0.0;
    while current_color_number < total_beacons {
        let lab_a: f32 = current_color_rad.cos() as f32 * saturation;
        let lab_b: f32 = current_color_rad.sin() as f32 * saturation;

        let new_color = oklab_to_srgb(Oklab {
            l: luminance,
            a: lab_a,
            b: lab_b,
        });

        //println!("{new_color}");

        beacon_colors.insert(current_color_number, new_color);

        current_color_number += 1;
        current_color_rad += rad_per_beacon;
    }

    //find combo that works best for minecraft blocks

    let possible_minecraft_colors = vec![
        White, LightGray, Gray, Black, Brown, Red, Orange, Yellow, Lime, Green, Cyan, LightBlue,
        Blue, Purple, Magenta, Pink,
    ];

    //ugh
    let mut all_block_glass_list: BTreeMap<RGB<u8>, Vec<MinecraftColor>> = BTreeMap::new();

    for possible_color_1 in possible_minecraft_colors.clone() {
        for possible_color_2 in possible_minecraft_colors.clone() {
            for possible_color_3 in possible_minecraft_colors.clone() {
                for possible_color_4 in possible_minecraft_colors.clone() {
                    let blocks = vec![
                        possible_color_1,
                        possible_color_2,
                        possible_color_3,
                        possible_color_4,
                    ];
                    let possible_color_rgb = blocks_to_color(blocks.clone());
                    all_block_glass_list.insert(possible_color_rgb, blocks);
                }
            }
        }
    }

    let mut final_list_of_beacon_glass: BTreeMap<i32, Vec<MinecraftColor>> = BTreeMap::new();

    let mut beacon_number = 0;
    for wanted_color in beacon_colors {
        let mut best_delta_e = 100.0;
        for possible_color in all_block_glass_list.clone() {
            let mut delta_e = lab_distance(&wanted_color.1, &possible_color.0);
            delta_e = delta_e.abs();
            if delta_e < best_delta_e {
                best_delta_e = delta_e;
                final_list_of_beacon_glass.insert(beacon_number,possible_color.1);
            }
        }

        beacon_number += 1;
    }

    for (key, value) in &beacon_data {
        //println!("{}. /tp {} 56 {}", key + 1, value.x + 0.5, value.y + 0.5);
        // println!("beacon number: {key}"); PUT THIS BACK
        //println!("{}, {}", value.x + 0.5, value.y + 0.5); PUT THIS BACK
        //println!("rad: {}", value.rad);
        //println!("delta: {}", value.delta);
        let colors = final_list_of_beacon_glass.get_key_value(&key).unwrap().1;
        //print_color(colors.clone()); PUT THIS BACK
    }

    let mut list_list_of_colors = vec![];

    for color_to_stat in final_list_of_beacon_glass {
        list_list_of_colors.push(color_to_stat.1)
    }

    let mut list_of_de = vec![];

    for list_number in 0..total_beacons {
        //println!("{}", list_number as usize);
        let color_1 = list_list_of_colors.get(list_number as usize).unwrap();
        let color_2;

        if list_number == 0 {
            color_2 = list_list_of_colors.get((total_beacons as usize - 1)).unwrap();
            //println!("{}", total_beacons as usize - 1)
        } else {
            color_2 = list_list_of_colors.get((list_number as usize - 1)).unwrap();
            //println!("{}", list_number as usize - 1)
        }

        let color_1_rgb = blocks_to_color(color_1.clone());
        let color_2_rgb = blocks_to_color(color_2.clone());

        let single_de = lab_distance(&color_1_rgb, &color_2_rgb);

        list_of_de.push(single_de);

        print!("{single_de}, ");
    }

    print!("{luminance}, ");
    println!("{saturation}");
    //print!("{}, ", mean(&list_of_de[..]).unwrap());
    //println!("{}, ", std_deviation(&list_of_de[..]).unwrap().powi(2));
    }
    }
}
