use sass_color_generator::args::Args;

fn main() {
    let args = Args::parse();

    let Args {
        outputfile,
        primary_color_hex,
    } = args;

    let rgb_val = convert_hex_to_rgb(&primary_color_hex);
    println!("{:?} => {:?}", primary_color_hex, rgb_val);
}

fn convert_hex_to_rgb(hex_value: &str) -> (u8, u8, u8) {
    // If hashtag is included let's remove it
    let mut raw_hex_val = String::from(hex_value);

    raw_hex_val.retain(|c| c != '#');

    let raw_red: &str = raw_hex_val.get(0..2).unwrap();
    let red: u8 = u8::from_str_radix(raw_red, 16).unwrap_or_default();
    let raw_green: &str = raw_hex_val.get(2..4).unwrap();
    let green: u8 = u8::from_str_radix(raw_green, 16).unwrap_or_default();

    let raw_blue: &str = raw_hex_val.get(4..).unwrap();
    let blue: u8 = u8::from_str_radix(raw_blue, 16).unwrap_or_default();

    (red, green, blue)
}
