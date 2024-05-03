#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct HSV {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct HSLuv {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

//-----------------------------------------------------------------------------

impl RGB {
    pub const fn new(r: u8,
                     g: u8,
                     b: u8) -> Self {
        return RGB { r, g, b };
    }
}

impl Default for RGB {
    fn default() -> Self {
        return RGB { r: 0, g: 0, b: 0 };
    }
}

//-----------------------------------------------------------------------------

impl HSV {
    pub const fn new(h: f64,
                     s: f64,
                     v: f64) -> Self {
        return HSV { h, s, v };
    }
}

impl Default for HSV {
    fn default() -> Self {
        return HSV { h: 0.0, s: 0.0, v: 0.0 };
    }
}

//-----------------------------------------------------------------------------

impl HSLuv {
    pub const fn new(h: f64,
                     s: f64,
                     v: f64) -> Self {
        return HSLuv { h, s, v }
    }
}

impl Default for HSLuv {
    fn default() -> Self {
        return HSLuv { h: 0.0, s: 0.0, v: 0.0 };
    }
}

//-----------------------------------------------------------------------------
