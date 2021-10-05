use clap::{App, Arg};

pub struct Args {
    pub primary_color_hex: String,
    pub outputfile: String,
}

impl Args {
    pub fn parse() -> Self {
        let matches = App::new("sass-color-generator")
            .arg(
                Arg::with_name("primaryColorHex")
                    .short("p")
                    .long("primary")
                    .takes_value(true)
                    .help("Main theme color in hex format"),
            )
            .arg(
                Arg::with_name("outputfile")
                    .short("o")
                    .long("outfile")
                    .takes_value(true)
                    .default_value("color_theme.scss"),
            )
            .get_matches();

        let primary_color_hex = matches
            .value_of("primaryColorHex")
            .unwrap_or_default()
            .to_string();
        let outputfile = matches
            .value_of("outputfile")
            .unwrap_or_default()
            .to_string();

        Self {
            primary_color_hex,
            outputfile,
        }
    }
}
