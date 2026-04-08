#[derive(Clone, Copy)]
pub enum MinecraftColor {
    White,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
}

// used for simple iteration
const MINECRAFT_COLORS: [MinecraftColor; 16] = [
    MinecraftColor::White,
    MinecraftColor::Orange,
    MinecraftColor::Magenta,
    MinecraftColor::LightBlue,
    MinecraftColor::Yellow,
    MinecraftColor::Lime,
    MinecraftColor::Pink,
    MinecraftColor::Gray,
    MinecraftColor::LightGray,
    MinecraftColor::Cyan,
    MinecraftColor::Purple,
    MinecraftColor::Blue,
    MinecraftColor::Brown,
    MinecraftColor::Green,
    MinecraftColor::Red,
    MinecraftColor::Black,
];

impl MinecraftColor {
    pub const fn name(&self) -> &str {
        match self {
            Self::White => "white_stained_glass",
            Self::Orange => "orange_stained_glass",
            Self::Magenta => "magenta_stained_glass",
            Self::LightBlue => "light_blue_stained_glass",
            Self::Yellow => "yellow_stained_glass",
            Self::Lime => "lime_stained_glass",
            Self::Pink => "pink_stained_glass",
            Self::Gray => "gray_stained_glass",
            Self::LightGray => "light_gray_stained_glass",
            Self::Cyan => "cyan_stained_glass",
            Self::Purple => "purple_stained_glass",
            Self::Blue => "blue_stained_glass",
            Self::Brown => "brown_stained_glass",
            Self::Green => "green_stained_glass",
            Self::Red => "red_stained_glass",
            Self::Black => "black_stained_glass",
        }
    }
    const fn hex(&self) -> &str {
        match self {
            Self::White => "#F9FFFE",
            Self::Orange => "#F9801D",
            Self::Magenta => "#C74EBD",
            Self::LightBlue => "#3AB3DA",
            Self::Yellow => "#FED83D",
            Self::Lime => "#80C71F",
            Self::Pink => "#F38BAA",
            Self::Gray => "#474F52",
            Self::LightGray => "#9D9D97",
            Self::Cyan => "#169C9C",
            Self::Purple => "#8932B8",
            Self::Blue => "#3C44AA",
            Self::Brown => "#835432",
            Self::Green => "#5E7C16",
            Self::Red => "#B02E26",
            Self::Black => "#1D1D21",
        }
    }
    fn rgb(&self) -> oklab::Rgb<u8> {
        oklab::Rgb {
            r: u8::from_str_radix(&self.hex()[1..3], 16).unwrap(),
            g: u8::from_str_radix(&self.hex()[3..5], 16).unwrap(),
            b: u8::from_str_radix(&self.hex()[5..7], 16).unwrap(),
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn rgb_from_stack(stack: &[MinecraftColor]) -> oklab::Rgb<u8> {
    // https://minecraft.wiki/w/Stained_Glass
    // trying to match this website as a target
    // whether good or bad, I am assuming these people knew what they were doing

    // if there are 0 elements, return white
    if stack.is_empty() {
        return MinecraftColor::White.rgb();
    }
    let mut rgb_sum: oklab::Rgb<u8> = stack[0].rgb();

    // second element up
    for color in &stack[1..] {
        let element_rgb = color.rgb();

        rgb_sum.r = (f32::from(rgb_sum.r) / 2.0 + f32::from(element_rgb.r) / 2.0) as u8;
        rgb_sum.g = (f32::from(rgb_sum.g) / 2.0 + f32::from(element_rgb.g) / 2.0) as u8;
        rgb_sum.b = (f32::from(rgb_sum.b) / 2.0 + f32::from(element_rgb.b) / 2.0) as u8;
    }
    rgb_sum
}

fn add_to_stack_list(stack_list: &mut Vec<Vec<MinecraftColor>>) {
    if stack_list.is_empty() {
        stack_list.push(vec![]);
    }
    let stack_list_length = stack_list.len();
    for i in 0..stack_list_length {
        for new_color in MINECRAFT_COLORS {
            let mut new_stack = stack_list[i].clone();
            new_stack.insert(0, new_color);
            stack_list.push(new_stack);
        }
    }
    stack_list.drain(0..stack_list_length);
}

fn rgb_distance(value_1: oklab::Rgb<u8>, value_2: oklab::Rgb<u8>) -> f64 {
    let dr = i32::from(value_1.r) - i32::from(value_2.r);
    let dg = i32::from(value_1.g) - i32::from(value_2.g);
    let db = i32::from(value_1.b) - i32::from(value_2.b);
    f64::from(dr.pow(2) + dg.pow(2) + db.pow(2)).sqrt()
}

pub fn best_guess_rgb(target_rgb: oklab::Rgb<u8>) -> Vec<MinecraftColor> {
    let iteration_limit = 16; // with my testing this should never create a
    // stack more than 15, so it's set to 16 to be safe
    let mut stack_list: Vec<Vec<MinecraftColor>> = vec![];
    let mut best_stack: Vec<MinecraftColor> = vec![];
    for _ in 0..iteration_limit {
        add_to_stack_list(&mut stack_list);
        let mut best_difference = f64::MAX;
        // this valuable get overwritten on the first iteration
        let mut best_rgb = oklab::Rgb { r: 0, g: 0, b: 0 };

        for stack in &stack_list {
            let stack_rgb = rgb_from_stack(stack);
            let difference = rgb_distance(target_rgb, stack_rgb);
            if difference < best_difference {
                best_stack.clone_from(stack);
                best_difference = difference;
                best_rgb = stack_rgb;
            }
        }

        // if it's already a match, return it early
        if target_rgb == best_rgb {
            break;
        }

        stack_list.clear();
        stack_list.push(best_stack.clone());
    }
    // if we couldn't create a perfect match, it's possible it can be made shorter
    if target_rgb != rgb_from_stack(&best_stack) {
        let best_stack_original = best_stack.clone();
        let mut best_difference = f64::MAX;
        for i in 1..best_stack_original.len() {
            let stack_slice = &best_stack_original[i..];
            let difference = rgb_distance(target_rgb, rgb_from_stack(stack_slice));
            if difference <= best_difference {
                best_stack = stack_slice.to_vec();
                best_difference = difference;
            }
        }
    }
    best_stack
}
