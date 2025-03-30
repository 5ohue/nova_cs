//-----------------------------------------------------------------------------
pub mod convert;
pub mod color;
pub mod lerp;
pub mod palette;
//-----------------------------------------------------------------------------
pub use color::*;
//-----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    static HEXES: [&str; 13] = [
        "#FFFFFF",
        "#000000",
        "#00FF00",
        "#FF0000",
        "#0000FF",
        "#7F7FFE",
        "#FE7F00",
        "#018d60",
        "#ec008c",
        "#2a3ab5",
        "#c0fe7f",
        "#3c8eb8",
        "#948096",
    ];
    static RGBS: [RGB; 13] = [
        RGB::new(255, 255, 255),
        RGB::new(0,     0,   0),
        RGB::new(  0, 255,   0),
        RGB::new(255,   0,   0),
        RGB::new(  0,   0, 255),
        RGB::new(127, 127, 254),
        RGB::new(254, 127,   0),
        RGB::new(  1, 141,  96),
        RGB::new(236,   0, 140),
        RGB::new( 42,  58, 181),
        RGB::new(192, 254, 127),
        RGB::new( 60, 142, 184),
        RGB::new(148, 128, 150),
    ];
    static HSVS: [HSV; 13] = [
        HSV::new(  0.0, 0.0,   1.0  ),
        HSV::new(  0.0, 0.0,   0.0  ),
        HSV::new(120.0, 1.0,   1.0  ),
        HSV::new(  0.0, 1.0,   1.0  ),
        HSV::new(240.0, 1.0,   1.0  ),
        HSV::new(240.0, 0.5,   0.996),
        HSV::new( 30.0, 1.0,   0.996),
        HSV::new(160.7, 0.993, 0.553),
        HSV::new(324.4, 1.0,   0.925),
        HSV::new(233.1, 0.768, 0.71 ),
        HSV::new( 89.3, 0.5  , 0.996),
        HSV::new(200.3, 0.674, 0.722),
        HSV::new(294.5, 0.147, 0.588),
    ];

    //-----------------------------------------------------------------------------
    // Compare by three decimals
    fn eps_cmp(a: f64, b: f64, eps: f64) -> bool {
        let delta = a - b;
        return delta.abs() < eps;
    }

    //-----------------------------------------------------------------------------

    #[test]
    fn check_hex_rgb() {
        HEXES.iter().zip(RGBS.iter()).for_each(|(hex, rgb)| {
            assert_eq!(convert::hex_to_rgb(hex), *rgb);
            assert_eq!(convert::rgb_to_hex(rgb).to_lowercase(), hex.to_lowercase());
        });
    }

    #[test]
    fn check_rgb_hsv() {
        RGBS.iter().zip(HSVS.iter()).for_each(|(rgb, hsv)| {
            let rgb_1 = convert::hsv_to_rgb(hsv);
            let hsv_1 = convert::rgb_to_hsv(rgb);

            dbg!(hsv_1);
            dbg!(hsv);

            assert_eq!(*rgb, rgb_1);
            assert!(eps_cmp(hsv_1.h, hsv.h, 1e-1));
            assert!(eps_cmp(hsv_1.s, hsv.s, 1e-3));
            assert!(eps_cmp(hsv_1.v, hsv.v, 1e-3));
        });
    }

    #[test]
    fn check_hsv_hex() {
        HEXES.iter().zip(HSVS.iter()).for_each(|(hex, hsv)| {
            let hex_1 = convert::hsv_to_hex(hsv);
            let hsv_1 = convert::hex_to_hsv(hex);

            assert_eq!(hex.to_lowercase(), hex_1.to_lowercase());
            assert!(eps_cmp(hsv_1.h, hsv.h, 1e-1));
            assert!(eps_cmp(hsv_1.s, hsv.s, 1e-3));
            assert!(eps_cmp(hsv_1.v, hsv.v, 1e-3));
        });
    }

    #[test]
    fn check_hsluv() {
        HEXES.iter().zip(RGBS.iter()).zip(HSVS.iter()).for_each(|((hex, rgb), hsv)| {
            let hsluv = convert::hex_to_hsluv(hex);
            let hex_1 = convert::hsluv_to_hex(&hsluv);
            assert_eq!(hex.to_lowercase(), hex_1.to_lowercase());
            let hsluv = convert::rgb_to_hsluv(rgb);
            let rgb_1 = convert::hsluv_to_rgb(&hsluv);
            assert_eq!(*rgb, rgb_1);
            let hsluv = convert::hsv_to_hsluv(hsv);
            let hsv_1 = convert::hsluv_to_hsv(&hsluv);
            assert!(eps_cmp(hsv_1.h, hsv.h, 1e-1));
            assert!(eps_cmp(hsv_1.s, hsv.s, 1e-3));
            assert!(eps_cmp(hsv_1.v, hsv.v, 1e-3));
        });
    }
}

//-----------------------------------------------------------------------------
