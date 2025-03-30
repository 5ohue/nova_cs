//-----------------------------------------------------------------------------
use crate::color::convert::*;
use crate::palette::{Palette, Colors, Mixed};
//-----------------------------------------------------------------------------
// Serializing function:
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
