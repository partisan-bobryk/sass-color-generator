extern crate angular_units as angle;

use angle::Deg;
use prisma::{FromColor, Hsl, Rgb};
use sass_color_generator::args::Args;
use std::fs::File;
use std::io::{BufWriter, Result, Write};

fn main() -> Result<()> {
    let args = Args::parse();

    let Args {
        outputfile,
        primary_color_hex,
    } = args;

    let rgb_val: Rgb<f32> = convert_hex_to_rgb(&primary_color_hex).color_cast();

    // Instantiate a write buffer
    // We are using a buffer here to keep things in memory and minimize on system calls
    let mut file_buffer = BufWriter::new(File::create(outputfile)?);

    // Insert primary color into the starting list
    file_buffer
        .write(format!("$primary-color: {};\n\n", &primary_color_hex).as_bytes())
        .unwrap();

    file_buffer.write(format!("\n// Tints - Ligheter").as_bytes()).unwrap();

    for (idx, tint) in calculate_tints(&rgb_val).iter().enumerate() {
        let hex_tint_val = convert_rgb_to_hex(tint);
        file_buffer
            .write(format!("$primary-tint-{}: {};\n", idx + 1, hex_tint_val).as_bytes())
            .unwrap();
    }

    file_buffer.write(format!("\n// Shades").as_bytes()).unwrap();

    for (idx, shade) in calculate_shades(&rgb_val).iter().enumerate() {
        let hex_shade_val = convert_rgb_to_hex(shade);

        file_buffer
            .write(format!("$primary-shade-{}: {};\n", idx + 1, hex_shade_val).as_bytes())
            .unwrap();
    }

    file_buffer.write(format!("\n").as_bytes()).unwrap();
    file_buffer.flush()?;
    Ok(())
}

fn convert_hex_to_rgb(hex_value: &str) -> Rgb<u8> {
    // If hashtag is included let's remove it
    let mut raw_hex_val = String::from(hex_value);

    raw_hex_val.retain(|c| c != '#');

    let raw_red: &str = raw_hex_val.get(0..2).unwrap();
    let red: u8 = u8::from_str_radix(raw_red, 16).unwrap_or_default();
    let raw_green: &str = raw_hex_val.get(2..4).unwrap();
    let green: u8 = u8::from_str_radix(raw_green, 16).unwrap_or_default();

    let raw_blue: &str = raw_hex_val.get(4..).unwrap();
    let blue: u8 = u8::from_str_radix(raw_blue, 16).unwrap_or_default();

    Rgb::new(red, green, blue)
}

fn convert_rgb_to_hex(rgb_val: &Rgb<f32>) -> String {
    let mut hex_display = String::from("#");
    let rgb_tuple: (u8, u8, u8) = (
        (rgb_val.red() * 255f32).floor() as u8,
        (rgb_val.green() * 255f32).floor() as u8,
        (rgb_val.blue() * 255f32).floor() as u8,
    );

    // Represent rgb value with two digits
    hex_display.push_str(format!("{:02x}", rgb_tuple.0).as_str());
    hex_display.push_str(format!("{:02x}", rgb_tuple.1).as_str());
    hex_display.push_str(format!("{:02x}", rgb_tuple.2).as_str());

    hex_display
}

fn calculate_tints(rgb_val: &Rgb<f32>) -> Vec<Rgb<f32>> {
    let number_of_tints = 4;
    let mut tints: Vec<Rgb<f32>> = vec![];

    for n in 1..number_of_tints {
        let mut hsl_val: Hsl<f32, Deg<f32>> = Hsl::from_color(rgb_val);
        let mut new_lightness_val: f32 = hsl_val.lightness() + (0.10 * n as f32);

        // Lightness can't be above 1
        if new_lightness_val >= 1.0 {
            new_lightness_val = 0.95;
        }

        hsl_val.set_lightness(new_lightness_val);
        let new_rgb_val: Rgb<f32> = Rgb::from_color(&hsl_val);
        tints.push(new_rgb_val);
    }

    tints
}

fn calculate_shades(rgb_val: &Rgb<f32>) -> Vec<Rgb<f32>> {
    let number_of_shades = 4;
    let mut shades: Vec<Rgb<f32>> = vec![];

    for n in 1..number_of_shades {
        let mut hsl_val: Hsl<f32, Deg<f32>> = Hsl::from_color(rgb_val);
        let mut new_lightness_val: f32 = hsl_val.lightness() - (0.10 * n as f32);
        // Lightness can't be below 0
        if new_lightness_val <= 0.0 {
            new_lightness_val = 0.05;
        }
        hsl_val.set_lightness(new_lightness_val);
        let new_rgb_val: Rgb<f32> = Rgb::from_color(&hsl_val);
        shades.push(new_rgb_val);
    }

    shades
}
