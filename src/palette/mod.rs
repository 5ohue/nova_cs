//-----------------------------------------------------------------------------
// Imports:

pub mod toml;
pub mod lua;

//-----------------------------------------------------------------------------
// Types:

use crate::RGB;

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
// Calculating colors:

pub fn calculate_more_colors(palette: &mut Palette) {
    use crate::lerp::*;

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
