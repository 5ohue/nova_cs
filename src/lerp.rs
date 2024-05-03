use crate::{RGB, HSV, HSLuv};
use crate::convert::{rgb_to_hsv, hsv_to_rgb, rgb_to_hsluv, hsluv_to_rgb};

//-----------------------------------------------------------------------------

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    return a + (b - a) * t;
}

//-----------------------------------------------------------------------------

pub fn lerp_rgb(a: &RGB, b: &RGB, t: f64) -> RGB {
    let r = lerp(a.r as f64, b.r as f64, t).round();
    let g = lerp(a.g as f64, b.g as f64, t).round();
    let b = lerp(a.b as f64, b.b as f64, t).round();

    return RGB::new(r as u8,
                    g as u8,
                    b as u8);
}

//-----------------------------------------------------------------------------

pub fn lerp_hsv(a: &HSV,
                b: &HSV,
                t: f64,
                clockwise: bool,
                closest: bool) -> HSV {

    let mut h = lerp(a.h, b.h, t);

    if closest {
        let d1 = (a.h         - b.h).abs();
        let d2 = (a.h + 360.0 - b.h).abs();
        let d3 = (a.h - 360.0 - b.h).abs();

        /* Find min */
             if d1 <= d2 && d1 <= d3 { h = lerp(a.h,         b.h, t); }
        else if d2 <= d1 && d2 <= d3 { h = lerp(a.h + 360.0, b.h, t).rem_euclid(360.0); }
        else                         { h = lerp(a.h - 360.0, b.h, t).rem_euclid(360.0); }
    } else {
        if clockwise && a.h > b.h {
            h = lerp(a.h, b.h + 360.0, t).rem_euclid(360.0);
        }
        if !clockwise && a.h < b.h {
            h = lerp(a.h + 360.0, b.h, t).rem_euclid(360.0);
        }
    }

    return HSV::new(h,
                    lerp(a.s, b.s, t),
                    lerp(a.v, b.v, t));
}

pub fn lerp_hsluv(a: &HSLuv,
                  b: &HSLuv,
                  t: f64,
                  clockwise: bool,
                  closest: bool) -> HSLuv {

    let mut h = lerp(a.h, b.h, t);

    if closest {
        let d1 = (a.h         - b.h).abs();
        let d2 = (a.h + 360.0 - b.h).abs();
        let d3 = (a.h - 360.0 - b.h).abs();

        /* Find min */
             if d1 <= d2 && d1 <= d3 { h = lerp(a.h,         b.h, t); }
        else if d2 <= d1 && d2 <= d3 { h = lerp(a.h + 360.0, b.h, t).rem_euclid(360.0); }
        else                         { h = lerp(a.h - 360.0, b.h, t).rem_euclid(360.0); }
    } else {
        if clockwise && a.h > b.h {
            h = lerp(a.h, b.h + 360.0, t).rem_euclid(360.0);
        }
        if !clockwise && a.h < b.h {
            h = lerp(a.h + 360.0, b.h, t).rem_euclid(360.0);
        }
    }

    return HSLuv::new(h,
                      lerp(a.s, b.s, t),
                      lerp(a.v, b.v, t));
}

//-----------------------------------------------------------------------------



//-----------------------------------------------------------------------------
// Linear interpolation by converting to a different color format in between
pub fn lerp_rgb_hsv(a: &RGB,
                    b: &RGB,
                    t: f64,
                    clockwise: bool,
                    closest: bool) -> RGB {
    return hsv_to_rgb(&lerp_hsv(&rgb_to_hsv(a),
                                &rgb_to_hsv(b),
                                t,
                                clockwise,
                                closest));
}

pub fn lerp_rgb_hsluv(a: &RGB,
                      b: &RGB,
                      t: f64,
                      clockwise: bool,
                      closest: bool) -> RGB {
    return hsluv_to_rgb(&lerp_hsluv(&rgb_to_hsluv(a),
                                    &rgb_to_hsluv(b),
                                    t,
                                    clockwise,
                                    closest));
}

//-----------------------------------------------------------------------------
