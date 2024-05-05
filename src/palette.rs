use crate::convert::*;
use crate::RGB;
use std::fmt::{Debug, Display};
use toml::Table;
use crate::lerp::*;

//-----------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct Colors {
    pub red:     RGB,
    pub yellow:  RGB,
    pub green:   RGB,
    pub cyan:    RGB,
    pub blue:    RGB,
    pub magenta: RGB,
}

#[derive(Debug, Clone, Default)]
pub struct Mixed {
    pub red_yellow:   RGB,
    pub yellow_green: RGB,
    pub green_cyan:   RGB,
    pub cyan_blue:    RGB,
    pub blue_magenta: RGB,
    pub magenta_red:  RGB,
}

#[derive(Debug, Clone, Default)]
pub struct Palette {
    pub fg: RGB,
    pub bg: RGB,

    pub normal:  Colors,
    pub bright:  Colors,

    /*
     * Calculated colors
     */
    pub bg_normal_25:   Colors,
    pub bg_normal_50:   Colors,
    pub bg_normal_75:   Colors,
    pub normal_bright_25: Colors,
    pub normal_bright_50: Colors,
    pub normal_bright_75: Colors,
    pub bright_fg_25:   Colors,
    pub bright_fg_50:   Colors,
    pub bright_fg_75:   Colors,

    // Additional colors
    pub bright_mix:  Mixed,
    pub normal_mix:  Mixed,

    pub bg_normal_25_mix:   Mixed,
    pub bg_normal_50_mix:   Mixed,
    pub bg_normal_75_mix:   Mixed,
    pub normal_bright_25_mix: Mixed,
    pub normal_bright_50_mix: Mixed,
    pub normal_bright_75_mix: Mixed,
    pub bright_fg_25_mix:   Mixed,
    pub bright_fg_50_mix:   Mixed,
    pub bright_fg_75_mix:   Mixed,

    pub grays: Vec<(i32, RGB)>,
}

impl Palette {
    pub fn new() -> Self {
        return Self::default();
    }
}

//-----------------------------------------------------------------------------

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

pub fn load_palette(toml: &str) -> Result<Palette, LoadPaletteError> {
    /*
     * Parse toml
     */
    let parsed = toml.parse::<Table>().map_err(|_| LoadPaletteError::TOMLFail)?;

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

pub fn calculate_more_colors(palette: &mut Palette) {
    /*
     * Between colors
     */
    palette.bg_normal_25.red     = lerp_rgb(&palette.bg, &palette.normal.red,     0.25);
    palette.bg_normal_25.yellow  = lerp_rgb(&palette.bg, &palette.normal.yellow,  0.25);
    palette.bg_normal_25.green   = lerp_rgb(&palette.bg, &palette.normal.green,   0.25);
    palette.bg_normal_25.cyan    = lerp_rgb(&palette.bg, &palette.normal.cyan,    0.25);
    palette.bg_normal_25.blue    = lerp_rgb(&palette.bg, &palette.normal.blue,    0.25);
    palette.bg_normal_25.magenta = lerp_rgb(&palette.bg, &palette.normal.magenta, 0.25);

    palette.bg_normal_50.red     = lerp_rgb(&palette.bg, &palette.normal.red,     0.50);
    palette.bg_normal_50.yellow  = lerp_rgb(&palette.bg, &palette.normal.yellow,  0.50);
    palette.bg_normal_50.green   = lerp_rgb(&palette.bg, &palette.normal.green,   0.50);
    palette.bg_normal_50.cyan    = lerp_rgb(&palette.bg, &palette.normal.cyan,    0.50);
    palette.bg_normal_50.blue    = lerp_rgb(&palette.bg, &palette.normal.blue,    0.50);
    palette.bg_normal_50.magenta = lerp_rgb(&palette.bg, &palette.normal.magenta, 0.50);

    palette.bg_normal_75.red     = lerp_rgb(&palette.bg, &palette.normal.red,     0.75);
    palette.bg_normal_75.yellow  = lerp_rgb(&palette.bg, &palette.normal.yellow,  0.75);
    palette.bg_normal_75.green   = lerp_rgb(&palette.bg, &palette.normal.green,   0.75);
    palette.bg_normal_75.cyan    = lerp_rgb(&palette.bg, &palette.normal.cyan,    0.75);
    palette.bg_normal_75.blue    = lerp_rgb(&palette.bg, &palette.normal.blue,    0.75);
    palette.bg_normal_75.magenta = lerp_rgb(&palette.bg, &palette.normal.magenta, 0.75);


    palette.normal_bright_25.red     = lerp_rgb(&palette.normal.red,     &palette.bright.red,     0.25);
    palette.normal_bright_25.yellow  = lerp_rgb(&palette.normal.yellow,  &palette.bright.yellow,  0.25);
    palette.normal_bright_25.green   = lerp_rgb(&palette.normal.green,   &palette.bright.green,   0.25);
    palette.normal_bright_25.cyan    = lerp_rgb(&palette.normal.cyan,    &palette.bright.cyan,    0.25);
    palette.normal_bright_25.blue    = lerp_rgb(&palette.normal.blue,    &palette.bright.blue,    0.25);
    palette.normal_bright_25.magenta = lerp_rgb(&palette.normal.magenta, &palette.bright.magenta, 0.25);

    palette.normal_bright_50.red     = lerp_rgb(&palette.normal.red,     &palette.bright.red,     0.50);
    palette.normal_bright_50.yellow  = lerp_rgb(&palette.normal.yellow,  &palette.bright.yellow,  0.50);
    palette.normal_bright_50.green   = lerp_rgb(&palette.normal.green,   &palette.bright.green,   0.50);
    palette.normal_bright_50.cyan    = lerp_rgb(&palette.normal.cyan,    &palette.bright.cyan,    0.50);
    palette.normal_bright_50.blue    = lerp_rgb(&palette.normal.blue,    &palette.bright.blue,    0.50);
    palette.normal_bright_50.magenta = lerp_rgb(&palette.normal.magenta, &palette.bright.magenta, 0.50);

    palette.normal_bright_75.red     = lerp_rgb(&palette.normal.red,     &palette.bright.red,     0.75);
    palette.normal_bright_75.yellow  = lerp_rgb(&palette.normal.yellow,  &palette.bright.yellow,  0.75);
    palette.normal_bright_75.green   = lerp_rgb(&palette.normal.green,   &palette.bright.green,   0.75);
    palette.normal_bright_75.cyan    = lerp_rgb(&palette.normal.cyan,    &palette.bright.cyan,    0.75);
    palette.normal_bright_75.blue    = lerp_rgb(&palette.normal.blue,    &palette.bright.blue,    0.75);
    palette.normal_bright_75.magenta = lerp_rgb(&palette.normal.magenta, &palette.bright.magenta, 0.75);

    palette.bright_fg_25.red     = lerp_rgb(&palette.bright.red,     &palette.fg, 0.25);
    palette.bright_fg_25.yellow  = lerp_rgb(&palette.bright.yellow,  &palette.fg, 0.25);
    palette.bright_fg_25.green   = lerp_rgb(&palette.bright.green,   &palette.fg, 0.25);
    palette.bright_fg_25.cyan    = lerp_rgb(&palette.bright.cyan,    &palette.fg, 0.25);
    palette.bright_fg_25.blue    = lerp_rgb(&palette.bright.blue,    &palette.fg, 0.25);
    palette.bright_fg_25.magenta = lerp_rgb(&palette.bright.magenta, &palette.fg, 0.25);

    palette.bright_fg_50.red     = lerp_rgb(&palette.bright.red,     &palette.fg, 0.50);
    palette.bright_fg_50.yellow  = lerp_rgb(&palette.bright.yellow,  &palette.fg, 0.50);
    palette.bright_fg_50.green   = lerp_rgb(&palette.bright.green,   &palette.fg, 0.50);
    palette.bright_fg_50.cyan    = lerp_rgb(&palette.bright.cyan,    &palette.fg, 0.50);
    palette.bright_fg_50.blue    = lerp_rgb(&palette.bright.blue,    &palette.fg, 0.50);
    palette.bright_fg_50.magenta = lerp_rgb(&palette.bright.magenta, &palette.fg, 0.50);

    palette.bright_fg_75.red     = lerp_rgb(&palette.bright.red,     &palette.fg, 0.75);
    palette.bright_fg_75.yellow  = lerp_rgb(&palette.bright.yellow,  &palette.fg, 0.75);
    palette.bright_fg_75.green   = lerp_rgb(&palette.bright.green,   &palette.fg, 0.75);
    palette.bright_fg_75.cyan    = lerp_rgb(&palette.bright.cyan,    &palette.fg, 0.75);
    palette.bright_fg_75.blue    = lerp_rgb(&palette.bright.blue,    &palette.fg, 0.75);
    palette.bright_fg_75.magenta = lerp_rgb(&palette.bright.magenta, &palette.fg, 0.75);

    /*
     * Mixed colors
     */
    fn calculate_mixed(mix: &mut Mixed, colors: &Colors) {
        mix.red_yellow   = lerp_rgb_hsluv(&colors.red,     &colors.yellow,  0.5, true, true);
        mix.yellow_green = lerp_rgb_hsluv(&colors.yellow,  &colors.green,   0.5, true, true);
        mix.green_cyan   = lerp_rgb_hsluv(&colors.green,   &colors.cyan,    0.5, true, false);
        mix.cyan_blue    = lerp_rgb_hsluv(&colors.cyan,    &colors.blue,    0.5, true, true);
        mix.blue_magenta = lerp_rgb_hsluv(&colors.blue,    &colors.magenta, 0.5, true, false);
        mix.magenta_red  = lerp_rgb_hsluv(&colors.magenta, &colors.red,     0.5, true, false);
    }

    calculate_mixed(&mut palette.normal_mix, &palette.normal);
    calculate_mixed(&mut palette.bright_mix, &palette.bright);

    calculate_mixed(&mut palette.bg_normal_25_mix,     &palette.bg_normal_25);
    calculate_mixed(&mut palette.bg_normal_50_mix,     &palette.bg_normal_50);
    calculate_mixed(&mut palette.bg_normal_75_mix,     &palette.bg_normal_75);
    calculate_mixed(&mut palette.normal_bright_25_mix, &palette.normal_bright_25);
    calculate_mixed(&mut palette.normal_bright_50_mix, &palette.normal_bright_50);
    calculate_mixed(&mut palette.normal_bright_75_mix, &palette.normal_bright_75);
    calculate_mixed(&mut palette.bright_fg_25_mix,     &palette.bright_fg_25);
    calculate_mixed(&mut palette.bright_fg_50_mix,     &palette.bright_fg_50);
    calculate_mixed(&mut palette.bright_fg_75_mix,     &palette.bright_fg_75);

    /*
     * Gray shades
     */
    for i in -5..=98 {
        let t = (i as f64) / 100.0;
        palette.grays.push((i, lerp_rgb(&palette.bg, &palette.fg, t)));
    }
}

//-----------------------------------------------------------------------------
/*
 * Gigachad format! serialization
 */
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

pub fn palette_to_lua(palette: &Palette) -> String {
    let mut lua;
    lua = format!("return {{\n    fg = \"{}\",\n    bg = \"{}\",",
                  rgb_to_hex(&palette.fg),
                  rgb_to_hex(&palette.bg));

    fn add_colors(lua: &mut String, name: &str, colors: &Colors) {
        *lua = format!(
"{}
    {} = {{
        red     = \"{}\",
        yellow  = \"{}\",
        green   = \"{}\",
        cyan    = \"{}\",
        blue    = \"{}\",
        magenta = \"{}\",
    }},",
lua,
name,
rgb_to_hex(&colors.red),
rgb_to_hex(&colors.yellow),
rgb_to_hex(&colors.green),
rgb_to_hex(&colors.cyan),
rgb_to_hex(&colors.blue),
rgb_to_hex(&colors.magenta))
    }

    add_colors(&mut lua, "bright", &palette.bright);
    add_colors(&mut lua, "normal", &palette.normal);
    add_colors(&mut lua, "bg_normal_25",     &palette.bg_normal_25);
    add_colors(&mut lua, "bg_normal_50",     &palette.bg_normal_50);
    add_colors(&mut lua, "bg_normal_75",     &palette.bg_normal_75);
    add_colors(&mut lua, "normal_bright_25", &palette.normal_bright_25);
    add_colors(&mut lua, "normal_bright_50", &palette.normal_bright_50);
    add_colors(&mut lua, "normal_bright_75", &palette.normal_bright_75);
    add_colors(&mut lua, "bright_fg_25",     &palette.bright_fg_25);
    add_colors(&mut lua, "bright_fg_50",     &palette.bright_fg_50);
    add_colors(&mut lua, "bright_fg_75",     &palette.bright_fg_75);

    fn add_mixed(lua: &mut String, name: &str, mix: &Mixed) {
        *lua = format!(
"{}
    mixed_{} = {{
        red_yellow   = \"{}\",
        yellow_green = \"{}\",
        green_cyan   = \"{}\",
        cyan_blue    = \"{}\",
        blue_magenta = \"{}\",
        magenta_red  = \"{}\",
    }},",
lua,
name,
rgb_to_hex(&mix.red_yellow),
rgb_to_hex(&mix.yellow_green),
rgb_to_hex(&mix.green_cyan),
rgb_to_hex(&mix.cyan_blue),
rgb_to_hex(&mix.blue_magenta),
rgb_to_hex(&mix.magenta_red))
    }

    add_mixed(&mut lua, "bright", &palette.bright_mix);
    add_mixed(&mut lua, "normal", &palette.normal_mix);
    add_mixed(&mut lua, "bg_normal_25",     &palette.bg_normal_25_mix);
    add_mixed(&mut lua, "bg_normal_50",     &palette.bg_normal_50_mix);
    add_mixed(&mut lua, "bg_normal_75",     &palette.bg_normal_75_mix);
    add_mixed(&mut lua, "normal_bright_25", &palette.normal_bright_25_mix);
    add_mixed(&mut lua, "normal_bright_50", &palette.normal_bright_50_mix);
    add_mixed(&mut lua, "normal_bright_75", &palette.normal_bright_75_mix);
    add_mixed(&mut lua, "bright_fg_25",     &palette.bright_fg_25_mix);
    add_mixed(&mut lua, "bright_fg_50",     &palette.bright_fg_50_mix);
    add_mixed(&mut lua, "bright_fg_75",     &palette.bright_fg_75_mix);

    let mut grays = String::new();
    for (i, gray) in palette.grays.iter() {
        if *i < 0 {
            grays = format!("{}\n    gray_n{:02} = \"{}\",", grays, -i, rgb_to_hex(&gray));
        } else {
            grays = format!("{}\n    gray_{:02} = \"{}\",", grays, i, rgb_to_hex(&gray));
        }
    }

    lua = format!("{}\n{}\n}}", lua, grays);

    return lua;
}

//-----------------------------------------------------------------------------
