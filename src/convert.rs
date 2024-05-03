use std::cmp::{min, max};

use crate::{RGB, HSV, HSLuv};

//-----------------------------------------------------------------------------
// hex_to functions:
pub fn hex_to_rgb(hex: &str) -> RGB {
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[5..],  16).unwrap_or(0);

    return RGB::new(r, g, b);
}

pub fn hex_to_hsv(hex: &str) -> HSV {
    let rgb = hex_to_rgb(hex);
    return rgb_to_hsv(&rgb);
}

pub fn hex_to_hsluv(hex: &str) -> HSLuv {
    let (h, s, l) = hsluv::hex_to_hsluv(hex);
    return HSLuv::new(h, s, l);
}

//-----------------------------------------------------------------------------
// rgb_to functions:
pub fn rgb_to_hsv(rgb: &RGB) -> HSV {
    let (r, g, b) = (rgb.r as f64, rgb.g as f64, rgb.b as f64);

    let min = min(min(rgb.r, rgb.g), rgb.b) as f64;
    let max = max(max(rgb.r, rgb.g), rgb.b) as f64;

    let h: f64;
    let s: f64;
    let v: f64;

    v = max / 255.0;
    if min == max {
        return HSV::new(0.0, 0.0, v);
    }

    s = (max - min) / max;
    if r == max {
        h = (g - b) / (max - min);
    } else if g == max {
        h = 2.0 + (b - r) / (max - min);
    } else {
        h = 4.0 + (r - g) / (max - min);
    }
    let h = (h * 60.0).rem_euclid(360.0);

    return HSV::new(h, s, v);
}

pub fn rgb_to_hex(rgb: &RGB) -> String {
    return format!("#{:02X}{:02X}{:02X}", rgb.r, rgb.g, rgb.b);
}

pub fn rgb_to_hsluv(rgb: &RGB) -> HSLuv {
    let r = rgb.r as f64 / 255.0;
    let g = rgb.g as f64 / 255.0;
    let b = rgb.b as f64 / 255.0;

    let (h, s, l) = hsluv::rgb_to_hsluv((r, g, b));

    return HSLuv::new(h, s, l);
}

//-----------------------------------------------------------------------------
// hsv_to functions:
fn hsv_to_rgb_float(hsv: &HSV) -> (f64, f64, f64) {
    let r;
    let g;
    let b;

    let s = hsv.s.clamp(0.0, 1.0);
    let v = hsv.v.clamp(0.0, 1.0);

    let i = (hsv.h / 60.0).floor();
    let f = hsv.h / 60.0 - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f*s);
    let t = v * (1.0 - (1.0 - f)*s);

    let i = (i as u8) % 6;
    match i {
        0 => { r = v; g = t; b = p; }
        1 => { r = q; g = v; b = p; }
        2 => { r = p; g = v; b = t; }
        3 => { r = p; g = q; b = v; }
        4 => { r = t; g = p; b = v; }
        5 => { r = v; g = p; b = q; }
        _ => { unreachable!();      }
    }

    let r = r.clamp(0.0, 1.0);
    let g = g.clamp(0.0, 1.0);
    let b = b.clamp(0.0, 1.0);

    return (r, g, b);
}

pub fn hsv_to_rgb(hsv: &HSV) -> RGB {
    let (r, g, b) = hsv_to_rgb_float(hsv);
    return RGB::new((r * 255.0).round() as u8,
                    (g * 255.0).round() as u8,
                    (b * 255.0).round() as u8);
}

pub fn hsv_to_hex(hsv: &HSV) -> String {
    let rgb = hsv_to_rgb(hsv);
    return rgb_to_hex(&rgb);
}

pub fn hsv_to_hsluv(hsv: &HSV) -> HSLuv {
    let rgb = hsv_to_rgb_float(hsv);
    let (h, s, v) = hsluv::rgb_to_hsluv(rgb);
    return HSLuv::new(h, s, v);
}

//-----------------------------------------------------------------------------
// hsvluv_to functions:
pub fn hsluv_to_rgb(hsluv: &HSLuv) -> RGB {
    let (r, g, b) = hsluv::hsluv_to_rgb((hsluv.h, hsluv.s, hsluv.v));

    return RGB::new((r.clamp(0.0, 1.0) * 255.0).round() as u8,
                    (g.clamp(0.0, 1.0) * 255.0).round() as u8,
                    (b.clamp(0.0, 1.0) * 255.0).round() as u8);
}

pub fn hsluv_to_hsv(hsluv: &HSLuv) -> HSV {
    let rgb = hsluv_to_rgb(hsluv);
    return rgb_to_hsv(&rgb);
}

pub fn hsluv_to_hex(hsluv: &HSLuv) -> String {
    let rgb = hsluv_to_rgb(hsluv);
    return rgb_to_hex(&rgb);
}

//-----------------------------------------------------------------------------
