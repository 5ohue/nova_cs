//-----------------------------------------------------------------------------
// Includes:
use crate::color::convert::*;
use crate::palette::{Palette, Colors, Mixed};
use std::fmt::{Debug, Display};
//-----------------------------------------------------------------------------
// Error type:
pub enum LoadPaletteError {
    TOMLFail,
    NoColorsTable,
}

impl Display for LoadPaletteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &LoadPaletteError::TOMLFail      => { write!(f, "failed to parse toml") }
            &LoadPaletteError::NoColorsTable => { write!(f, "no colors table in toml file") }
        }
    }
}
impl Debug for LoadPaletteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for LoadPaletteError {}

//-----------------------------------------------------------------------------
// Loading function:
pub fn load_palette(toml: &str) -> Result<Palette, LoadPaletteError> {
    /*
     * Parse toml
     */
    let parsed = toml
        .parse::<toml::Table>()
        .map_err(|_| LoadPaletteError::TOMLFail)?;

    /*
     * Get "colors" table
     */
    let colors = parsed.get("colors").ok_or(LoadPaletteError::NoColorsTable)?;
    let colors = colors.as_table().expect("[colors] should be a table!");

    let mut palette = Palette::new();

    /*
     * Read "colors.primary" table
     */
    let primary_table = colors["primary"].as_table().expect("[colors.primary] should be a table!");

    palette.fg = hex_to_rgb(primary_table["foreground"].as_str().unwrap_or("#FFFFFF"));
    palette.bg = hex_to_rgb(primary_table["background"].as_str().unwrap_or("#000000"));

    /*
     * Read "colors.normal" table
     */
    let normal_table = colors["normal"].as_table().expect("[colors.normal] should be a table!");

    palette.normal.red     = hex_to_rgb(normal_table["red"]    .as_str().unwrap_or("#FF0000"));
    palette.normal.yellow  = hex_to_rgb(normal_table["yellow"] .as_str().unwrap_or("#FFFF00"));
    palette.normal.green   = hex_to_rgb(normal_table["green"]  .as_str().unwrap_or("#00FF00"));
    palette.normal.cyan    = hex_to_rgb(normal_table["cyan"]   .as_str().unwrap_or("#00FFFF"));
    palette.normal.blue    = hex_to_rgb(normal_table["blue"]   .as_str().unwrap_or("#0000FF"));
    palette.normal.magenta = hex_to_rgb(normal_table["magenta"].as_str().unwrap_or("#FF00FF"));

    /*
     * Read "colors.bright" table
     */
    let bright_table = colors["bright"].as_table().expect("[colors.bright] should be a table!");

    palette.bright.red     = hex_to_rgb(bright_table["red"]    .as_str().unwrap_or("#FF0000"));
    palette.bright.yellow  = hex_to_rgb(bright_table["yellow"] .as_str().unwrap_or("#FFFF00"));
    palette.bright.green   = hex_to_rgb(bright_table["green"]  .as_str().unwrap_or("#00FF00"));
    palette.bright.cyan    = hex_to_rgb(bright_table["cyan"]   .as_str().unwrap_or("#00FFFF"));
    palette.bright.blue    = hex_to_rgb(bright_table["blue"]   .as_str().unwrap_or("#0000FF"));
    palette.bright.magenta = hex_to_rgb(bright_table["magenta"].as_str().unwrap_or("#FF00FF"));

    return Ok(palette);
}

//-----------------------------------------------------------------------------
// Serializing function:
pub fn palette_to_toml(palette: &Palette) -> String {
    let mut toml;
    toml = format!("[colors.primary]\nbg = \"{}\"\nfg = \"{}\"\n\n",
                   rgb_to_hex(&palette.bg),
                   rgb_to_hex(&palette.fg));

    /*
     * Adding `Colors`
     */
    fn add_colors(toml: &mut String, name: &str, colors: &Colors) {
        *toml = format!(
"{}[colors.{}]
red     =  \"{}\"
yellow  =  \"{}\"
green   =  \"{}\"
cyan    =  \"{}\"
blue    =  \"{}\"
magenta =  \"{}\"

",
toml,
name,
rgb_to_hex(&colors.red),
rgb_to_hex(&colors.yellow),
rgb_to_hex(&colors.green),
rgb_to_hex(&colors.cyan),
rgb_to_hex(&colors.blue),
rgb_to_hex(&colors.magenta))
    }
    add_colors(&mut toml, "bright", &palette.bright);
    add_colors(&mut toml, "normal", &palette.normal);
    add_colors(&mut toml, "bg_normal_25",   &palette.bg_normal_25);
    add_colors(&mut toml, "bg_normal_50",   &palette.bg_normal_50);
    add_colors(&mut toml, "bg_normal_75",   &palette.bg_normal_75);
    add_colors(&mut toml, "normal_bright_25", &palette.normal_bright_25);
    add_colors(&mut toml, "normal_bright_50", &palette.normal_bright_50);
    add_colors(&mut toml, "normal_bright_75", &palette.normal_bright_75);
    add_colors(&mut toml, "bright_fg_25",   &palette.bright_fg_25);
    add_colors(&mut toml, "bright_fg_50",   &palette.bright_fg_50);
    add_colors(&mut toml, "bright_fg_75",   &palette.bright_fg_75);

    fn add_mixed(toml: &mut String, name: &str, mix: &Mixed) {
        *toml = format!(
"{}[mixed.{}]
red_yellow   = \"{}\"
yellow_green = \"{}\"
green_cyan   = \"{}\"
cyan_blue    = \"{}\"
blue_magenta = \"{}\"
magenta_red  = \"{}\"

",
toml,
name,
rgb_to_hex(&mix.red_yellow),
rgb_to_hex(&mix.yellow_green),
rgb_to_hex(&mix.green_cyan),
rgb_to_hex(&mix.cyan_blue),
rgb_to_hex(&mix.blue_magenta),
rgb_to_hex(&mix.magenta_red))
    }
    add_mixed(&mut toml, "bright", &palette.bright_mix);
    add_mixed(&mut toml, "normal", &palette.normal_mix);
    add_mixed(&mut toml, "bg_normal_25",     &palette.bg_normal_25_mix);
    add_mixed(&mut toml, "bg_normal_50",     &palette.bg_normal_50_mix);
    add_mixed(&mut toml, "bg_normal_75",     &palette.bg_normal_75_mix);
    add_mixed(&mut toml, "normal_bright_25", &palette.normal_bright_25_mix);
    add_mixed(&mut toml, "normal_bright_50", &palette.normal_bright_50_mix);
    add_mixed(&mut toml, "normal_bright_75", &palette.normal_bright_75_mix);
    add_mixed(&mut toml, "bright_fg_25",     &palette.bright_fg_25_mix);
    add_mixed(&mut toml, "bright_fg_50",     &palette.bright_fg_50_mix);
    add_mixed(&mut toml, "bright_fg_75",     &palette.bright_fg_75_mix);

    let mut grays = "[grays]".to_string();
    for (i, gray) in palette.grays.iter() {
        if *i < 0 {
            grays = format!("{}\ngray_n{:02} = \"{}\"", grays, -i, rgb_to_hex(&gray));
        } else {
            grays = format!("{}\ngray_{:02} = \"{}\"", grays, i, rgb_to_hex(&gray));
        }
    }

    toml = format!("{}\n{}", toml, grays);

    return toml;
}

//-----------------------------------------------------------------------------
