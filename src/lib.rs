//! Color creation and manipulation
//!
//! ```
//! use tint::Color;
//! use tint::Colour; // Alternatively
//! let green = Color::from("green");
//! let green = Color::from("00FF00");
//! let green = Color::from("00ff00");
//! let green = Color::from("#00ff00");
//! let green = Color::from("#00FF00");
//! let green = Color::from((0,255,0));
//! let green = Color::from([0.,1.,0.]);
//! let green = Color::from(vec![0.,1.,0.]);
//! let green = Color::from(vec![0.,1.,0.,1.0]);
//! let green = Color::from(&vec![0.,1.,0.]);
//! let green = Color::from(&vec![0.,1.,0.,1.0]);
//!
//! let green = Color::new(0.0, 1.0, 0.0, 1.0);
//! let green = Color::from_rgb1(0.0, 1.0, 0.0);
//! let green = Color::from_rgb1v(&[0.0, 1.0, 0.0]);
//! let green = Color::name("green");
//! let green = Colour::name("green");
//! ```
//!
//! # Color names
//!   Typical names (HTML and SVG) are available by default, and
//!   color names defined in the XKCD color database are available
//!   by running the xkcd() function
//! ## Basic Colors
//! https://www.w3.org/TR/css3-color/#html4
//! ## Extended Colors
//! https://www.w3.org/TR/css3-color/#svg-color
//! ## XKCD Colors
//! https://xkcd.com/color/rgb/

use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{BufRead, BufReader, Cursor},
    path::Path,
    sync::Mutex,
};

pub type Colour = Color;

/// Color value
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    /// Red component [0,1]
    pub red: f64,
    /// Green component [0,1]
    pub green: f64,
    /// Blue component [0,1]
    pub blue: f64,
    /// Alpha component [0,1]
    pub alpha: f64,
}
impl Color {
    /// Create new color from components
    ///
    /// ```
    /// use tint::Color;
    /// let red = Color::new(1.0, 0.0, 0.0, 1.0);
    /// let fushcia = Color::new(1.0, 0.0, 1.0, 1.0);
    /// ```
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Color {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    // RGB 1.0
    /// Create new color from RGB components [0. .. 1.0],
    ///   alpha value set to 1.0
    ///
    /// ```
    /// # use tint::Color;
    /// let blue = Color::from_rgb1(0.0, 0.0, 1.0);
    /// ```
    pub fn from_rgb1(r: f64, g: f64, b: f64) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
            alpha: 1.0,
        }
    }
    /// Create new color from RGB f64 vector [0. .. 1.0],
    ///   alpha value set to 1.0
    ///
    /// ```
    /// # use tint::Color;
    /// let green = Color::from_rgb1v(&[0.0, 1.0, 0.0]);
    /// ```
    pub fn from_rgb1v(rgb: &[f64]) -> Color {
        Color {
            red: rgb[0],
            green: rgb[1],
            blue: rgb[2],
            alpha: 1.0,
        }
    }
    /// Convert Color to (f64,f64,f64)
    ///
    /// ```
    /// # use tint::Color;
    /// let purple = Color::new(1.0, 0.0, 1.0, 1.0);
    /// let rgb = purple.to_rgb1();
    /// assert_eq!(rgb, (1.0, 0.0, 1.0));
    /// ```
    pub fn to_rgb1(&self) -> (f64, f64, f64) {
        (self.red, self.green, self.blue)
    }

    // RGB 255
    /// Create new Color from RGB [0 .. 255]
    ///   alpha value set to 1.0
    ///
    /// ```
    /// # use tint::Color;
    /// let purple = Color::from_rgb255(255, 0, 255);
    /// ```
    pub fn from_rgb255(red: u8, green: u8, blue: u8) -> Color {
        Color::from_rgb1(
            f64::from(red) / 255.,
            f64::from(green) / 255.,
            f64::from(blue) / 255.,
        )
    }
    /// Create new Color from RGB u8 vector [0 .. 255]
    ///   alpha value set to 1.0
    ///
    /// ```
    /// # use tint::Color;
    /// let purple = Color::from_rgb255v(&[255, 0, 255]);
    /// assert_eq!(purple.red,   1.0);
    /// assert_eq!(purple.green, 0.0);
    /// assert_eq!(purple.blue,  1.0);
    /// ```
    pub fn from_rgb255v(rgb: &[u8]) -> Color {
        Color::from_rgb255(rgb[0], rgb[1], rgb[2])
    }
    /// Convert color to (u8,u8,u8)
    ///
    /// ```
    /// # use tint::Color;
    /// let purple = Color::new(1.0, 0.0, 1.0, 1.0);
    /// assert_eq!(purple.to_rgb255(), (255,0,255));
    /// ```
    pub fn to_rgb255(&self) -> (u8, u8, u8) {
        (
            (self.red * 255.0) as u8,
            (self.green * 255.0) as u8,
            (self.blue * 255.0) as u8,
        )
    }

    // HEX
    /// Create new Color from Hex String
    ///
    /// ```
    /// # use tint::Color;
    /// let facade = Color::from_hex("#facade");
    /// assert_eq!(facade.to_rgb255(), (250, 202, 222));
    /// ```
    pub fn from_hex(hex: &str) -> Color {
        let n = if hex.chars().nth(0).unwrap() == '#' {
            1
        } else {
            0
        };
        let r = u8::from_str_radix(&hex[n..n + 2], 16).unwrap();
        let g = u8::from_str_radix(&hex[n + 2..n + 4], 16).unwrap();
        let b = u8::from_str_radix(&hex[n + 4..n + 6], 16).unwrap();
        Color::from_rgb255(r, g, b)
    }
    /// Convert Color into Hex String
    ///
    /// ```
    /// # use tint::Color;
    /// let coffee = Color::from_rgb255(192, 255, 238);
    /// assert_eq!(coffee.to_hex(), "#c0ffee");
    /// ```
    pub fn to_hex(&self) -> String {
        let (r, g, b) = self.to_rgb255();
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }
    //pub fn from_hexs(hex: &str) -> Vec<Color> {
    //    hex.split(',').map(|x| Color::from_hex(x)).collect()
    //}

    // Named Color
    /// Get Color from exiting named colors
    ///  Colors are defined from w3c Basic and Extended colors
    ///  and colors from the XKCD database if loaded
    ///
    /// ```
    /// # use tint::Color;
    /// let chartreuse = Color::name("chartreuse");
    /// assert_eq!(chartreuse, Some(Color::from_hex("7fff00")));
    /// let olive_drab = Color::name("olivedrab").unwrap();
    /// assert_eq!(olive_drab.to_rgb255(), (107,142,35));
    ///
    /// tint::xkcd();
    /// let butterscotch = Color::name("butterscotch").unwrap();
    /// assert_eq!(butterscotch.to_hex(), "#fdb147");
    ///
    /// let avocado = Color::name("avocado green").unwrap();
    /// assert_eq!(avocado.to_rgb255(), (135, 169, 34));
    /// ```
    pub fn name(name: &str) -> Option<Color> {
        match COLOR_MAP.lock().unwrap().get(name) {
            Some(&c) => Some(c),
            None => None,
        }
    }

    // HSV
    /// Convert Color to HSV
    pub fn to_hsv(&self) -> (f64, f64, f64) {
        rgb2hsv(self.red, self.green, self.blue)
    }
    /// Create new Color from HSV
    ///   alpha value set to 1.0
    pub fn from_hsv(&self) -> Color {
        let (r, g, b) = hsv2rgb(self.red, self.green, self.blue);
        Color::new(r, g, b, 1.0)
    }
    // HSL
    /// Convert Color to HSL
    pub fn to_hsl(&self) -> (f64, f64, f64) {
        rgb2hsl(self.red, self.green, self.blue)
    }
    /// Create new Color from HSL
    ///   alpha value set to 1.0
    pub fn from_hsl(&self) -> Color {
        let (r, g, b) = hsl2rgb(self.red, self.green, self.blue);
        Color::new(r, g, b, 1.0)
    }
    // YIQ
    /// Convert Color to YIQ
    pub fn to_yiq(&self) -> (f64, f64, f64) {
        rgb2yiq(self.red, self.green, self.blue)
    }
    /// Create new Color from YIQ
    ///   alpha value set to 1.0
    pub fn from_yiq(&self) -> Color {
        let (r, g, b) = yiq2rgb(self.red, self.green, self.blue);
        Color::new(r, g, b, 1.0)
    }
}

// Strings

/// Convert from named color or a hex string
///
/// This may fail
impl From<String> for Color {
    fn from(s: String) -> Color {
        match Color::name(&s) {
            None => Color::from_hex(&s),
            Some(c) => c,
        }
    }
}
/// Convert from named color or a hex string
///
/// This may fail
impl<'a> From<&'a String> for Color {
    fn from(s: &'a String) -> Color {
        match Color::name(s) {
            None => Color::from_hex(s),
            Some(c) => c,
        }
    }
}
/// Convert from named color or a hex string
///
/// This may fail
impl<'a> From<&'a str> for Color {
    fn from(s: &'a str) -> Color {
        match Color::name(s) {
            None => Color::from_hex(s),
            Some(c) => c,
        }
    }
}

// Tuples
/// Convert from a u8 triple, red, green, blue
impl From<(u8, u8, u8)> for Color {
    fn from(c: (u8, u8, u8)) -> Color {
        Color::from_rgb255(c.0, c.1, c.2)
    }
}
/// Convert from a f64 triple, red, green, blue
impl From<(f64, f64, f64)> for Color {
    fn from(c: (f64, f64, f64)) -> Color {
        Color::from_rgb1(c.0, c.1, c.2)
    }
}
/// Convert from a f32 triple, red, green, blue
impl From<(f32, f32, f32)> for Color {
    fn from(c: (f32, f32, f32)) -> Color {
        Color::from_rgb1(f64::from(c.0), f64::from(c.1), f64::from(c.2))
    }
}

// Arrays
/// Convert from a u8 triple, red, green, blue
impl<'a> From<&'a [u8; 3]> for Color {
    fn from(c: &'a [u8; 3]) -> Color {
        Color::from_rgb255v(c)
    }
}
/// Convert from a f64 triple, red, green, blue
impl<'a> From<&'a [f64; 3]> for Color {
    fn from(c: &'a [f64; 3]) -> Color {
        Color::from_rgb1v(c)
    }
}
/// Convert from a f32 triple, red, green, blue
impl<'a> From<&'a [f32; 3]> for Color {
    fn from(c: &'a [f32; 3]) -> Color {
        Color::new(f64::from(c[0]), f64::from(c[1]), f64::from(c[2]), 1.0)
    }
}
/// Convert from a u8 triple, red, green, blue
impl From<[u8; 3]> for Color {
    fn from(c: [u8; 3]) -> Color {
        Color::from_rgb255v(&c)
    }
}
/// Convert from a f64 triple, red, green, blue
impl From<[f64; 3]> for Color {
    fn from(c: [f64; 3]) -> Color {
        Color::from_rgb1v(&c)
    }
}
/// Convert from a f32 triple, red, green, blue
impl From<[f32; 3]> for Color {
    fn from(c: [f32; 3]) -> Color {
        Color::new(f64::from(c[0]), f64::from(c[1]), f64::from(c[2]), 1.0)
    }
}

// Vecs
/// Convert from a f64 Vec, red, green, blue, maybe alpha
///
/// This may fail
impl<'a> From<&'a Vec<f64>> for Color {
    fn from(c: &'a Vec<f64>) -> Color {
        match c.len() {
            3 => Color::new(c[0], c[1], c[2], 1.0),
            4 => Color::new(c[0], c[1], c[2], c[3]),
            _ => panic!("Expected three or four color components"),
        }
    }
}
/// Convert from a f32 Vec, red, green, blue, maybe alpha
///
/// This may fail
impl<'a> From<&'a Vec<f32>> for Color {
    fn from(c: &'a Vec<f32>) -> Color {
        let c64: Vec<_> = c.iter().map(|x| f64::from(*x)).collect();
        Color::from(&c64)
    }
}
/// Convert from a f64 Vec, red, green, blue, maybe alpha
///
/// This may fail
impl From<Vec<f64>> for Color {
    fn from(c: Vec<f64>) -> Color {
        Color::from(&c)
    }
}
/// Convert from a f32 Vec, red, green, blue, maybe alpha
///
/// This may fail
impl From<Vec<f32>> for Color {
    fn from(c: Vec<f32>) -> Color {
        let c64: Vec<_> = c.into_iter().map(f64::from).collect();
        Color::from(&c64)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:5.3}, {:5.3}, {:5.3}, {:5.3})",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

fn parse_rgb_name(line: &str) -> Option<(String, Vec<u8>)> {
    // R G B Color Names
    let rgb: Vec<_> = line
        .split_whitespace()
        .take(3)
        .map(|x| x.parse::<u8>())
        .collect();
    let is_rgb = rgb.iter().all(|x| x.is_ok());
    if is_rgb && rgb.len() == 3 {
        let rgb = rgb.into_iter().map(|x| x.unwrap()).collect::<Vec<u8>>();
        let name = line
            .split_whitespace()
            .skip(3)
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
            .join(" ");
        return Some((name, rgb));
    }
    None
}

fn parse_name_hex(line: &str) -> Option<(String, Color)> {
    // Color Names #RRGGBB
    let vals = line.split('#').map(|x| x.trim()).collect::<Vec<&str>>();
    if vals.len() == 2 {
        let name = vals[0].to_owned();
        let hex = vals[1];
        if hex.len() == 6 {
            return Some((name, Color::from_hex(hex)));
        }
    }
    None
}

/// Load a buffer and return a Vec<String, Color)> of names and colors
///
///   Available formats include:
///      name  #hex-value
///      r255 g255 b255 name
///   Lines beginning with # are ignored
pub fn read_buffer<T>(buf: T) -> Vec<(String, Color)>
where
    T: BufRead,
{
    let mut out = vec![];
    for xline in buf.lines() {
        let line = xline.unwrap();

        if let Some((name, rgb)) = parse_rgb_name(&line) {
            out.push((name, Color::from_rgb255v(&rgb)));
        } else if let Some((name, color)) = parse_name_hex(&line) {
            out.push((name, color))
        } else {
            //println!("Unknown line: {}", line);
        }
    }
    out
}

/// Read a file and return a Vec<String, Color)> of names and colors
pub fn read_file<P>(file: P) -> Vec<(String, Color)>
where
    P: AsRef<Path>,
{
    let fp = File::open(file).unwrap();
    let fp = BufReader::new(&fp);
    read_buffer(fp)
}

/// Load a buffer into the existing Named Color database.
///
///   Existing colors will not be overwritten and a warning will be issued.
pub fn load_rgb_buffer<T>(buf: T)
where
    T: BufRead,
{
    for (xname, color) in read_buffer(buf).into_iter() {
        let name = xname.to_lowercase();
        if COLOR_MAP.lock().unwrap().contains_key(&name) {
            println!("warning: color already exists: {}", name);
            continue;
        }
        COLOR_MAP.lock().unwrap().insert(name, color);
    }
}
/// Load a file into the existing Named Color database.
///
///   Existing colors will not be overwritten and a warning will be issued.
pub fn load_rgb_file<P>(file: P)
where
    P: AsRef<Path>,
{
    let fp = File::open(file).unwrap();
    let fp = BufReader::new(&fp);
    load_rgb_buffer(fp);
}

lazy_static! {
    static ref COLOR_MAP: Mutex<HashMap<String, Color>> = {
        let mut m: HashMap<String, Color> = HashMap::new();
        for s in [COLORS_BASIC, COLORS_EXTENDED].iter() {
            for (ref xname, color) in read_buffer(Cursor::new(s)).into_iter() {
                let name = xname.to_lowercase();
                m.entry(name).or_insert(color);
            }
        }
        Mutex::new(m)
    };
}
/// Load colors from the XKCD Color Database
pub fn xkcd() {
    load_rgb_buffer(Cursor::new(COLORS_XKCD));
}

/// Return names of available named colors
pub fn names() -> Vec<String> {
    let map = COLOR_MAP.lock().unwrap();
    map.keys().cloned().collect()
}

fn cmp3(a: (f64, f64, f64), b: (f64, f64, f64)) -> std::cmp::Ordering {
    if a.0 > b.0 {
        return std::cmp::Ordering::Greater;
    } else if a.0 < b.0 {
        return std::cmp::Ordering::Less;
    }
    if a.1 > b.1 {
        return std::cmp::Ordering::Greater;
    } else if a.1 < b.1 {
        return std::cmp::Ordering::Less;
    }
    if a.2 > b.2 {
        return std::cmp::Ordering::Greater;
    } else if a.2 < b.2 {
        return std::cmp::Ordering::Less;
    }
    std::cmp::Ordering::Equal
}

/// Compare Colors by red, then green, then blue
pub fn compare_by_rgb(a: &Color, b: &Color) -> std::cmp::Ordering {
    cmp3(a.to_rgb1(), b.to_rgb1())
}

/// Compare Colors by hue, then saturation, then value
pub fn compare_by_hsv(a: &Color, b: &Color) -> std::cmp::Ordering {
    cmp3(a.to_hsv(), b.to_hsv())
}

// https://en.wikipedia.org/wiki/YIQ#From_RGB_to_YIQ
// FCC NTSC Standard
fn rgb2yiq(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let y = 0.30 * r + 0.59 * g + 0.11 * b;
    let i = 0.74 * (r - y) - 0.27 * (b - y);
    let q = 0.48 * (r - y) + 0.41 * (b - y);
    (y, i, q)
}
fn yiq2rgb(y: f64, i: f64, q: f64) -> (f64, f64, f64) {
    let v33 = 1.709_006_928_406_466_6;
    let v32 = -1.108_545_034_642_032_2;
    let v22 = -0.274_787_646_298_978_34;
    let v23 = -0.635_691_079_187_380_1;
    let v13 = 0.623_556_581_986_143_3;
    let v12 = 0.946_882_217_090_069_3;
    let r = y + v12 * i + v13 * q;
    let g = y + v22 * i + v23 * q;
    let b = y + v32 * i + v33 * q;
    (r, g, b)
}

fn fmin(v: &[f64]) -> f64 {
    let mut val = v[0];
    for vi in v {
        if *vi < val {
            val = *vi;
        }
    }
    val
}
fn fmax(v: &[f64]) -> f64 {
    let mut val = v[0];
    for vi in v {
        if *vi > val {
            val = *vi;
        }
    }
    val
}

// https://en.wikipedia.org/wiki/HSL_and_HSV#Converting_to_RGB
// https://stackoverflow.com/a/6930407
/// h : [0, 360]
/// s : [0, 1]
/// v : [0, 1]
fn rgb2hsv(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let cmax = fmax(&[r, g, b]);
    let cmin = fmin(&[r, g, b]);
    if (cmax - cmin).abs() < 1e-5 {
        return (0., 0., cmax);
    }
    let v = cmax;
    let delta = cmax - cmin;
    let s = delta / cmax;
    //println!("rgb2hsv: {} {} {} {}", r,g,b,cmax);
    let mut h = if r >= cmax {
        (g - b) / delta
    } else if g >= cmax {
        2.0 + (b - r) / delta
    } else if b >= cmax {
        4.0 + (r - g) / delta
    } else {
        0.0
    };
    h *= 60.0;
    if h < 0.0 {
        h += 360.0;
    }
    (h, s, v)
}

fn hsv2rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    //println!("hsv: {} {} {}", h,s,v);
    if s <= 0.0 {
        return (v, v, v);
    }
    let mut hh = h;
    if hh >= 360.0 {
        hh = 0.0;
    }
    hh /= 60.0;
    let i = hh.floor() as u64;
    let ff = hh - i as f64;
    let p = v * (1.0 - s);
    let q = v * (1.0 - (s * ff));
    let t = v * (1.0 - (s * (1.0 - ff)));
    //println!("hsv: i {} {} {} {} {}", i, p,q,t,v);
    match i {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        _ => panic!("Unexpected value in hsv2rgb: i: {} h: {}", i, h),
    }
}

fn rgb2hsl(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let cmax = fmax(&[r, g, b]);
    let cmin = fmin(&[r, g, b]);
    let l = (cmax + cmin) / 2.0;
    if (cmax - cmin).abs() <= 1e-5 {
        return (0., 0., l);
    }
    let d = cmax - cmin;
    let s = if l <= 0.5 {
        d / (cmax + cmin)
    } else {
        d / (2.0 - cmax - cmin)
    };
    let mut h = if r >= cmax {
        let dh = if g < b { 6.0 } else { 0.0 };
        (g - b) / d + dh
    } else if g >= cmax {
        2.0 + (b - r) / d
    } else if b >= cmax {
        4.0 + (r - g) / d
    } else {
        0.0
    };
    h /= 6.0;
    (h, s, l)
}

fn hue2rgb(p: f64, q: f64, t: f64) -> f64 {
    let mut tt = t;
    if tt < 0.0 {
        tt += 1.0;
    }
    if tt > 1.0 {
        tt -= 1.0
    }
    if tt < 1. / 6. {
        return p + (q - p) * 6.0 * tt;
    }
    if tt < 1. / 2. {
        return q;
    }
    if tt < 2. / 3. {
        return p + (q - p) * (2. / 3. - tt) * 6.0;
    }
    p
}

fn hsl2rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    if s.abs() <= 1e-5 {
        return (l, l, l);
    }
    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - (l * s)
    };
    let p = 2.0 * l - q;
    let r = hue2rgb(p, q, h + 1. / 3.);
    let g = hue2rgb(p, q, h);
    let b = hue2rgb(p, q, h - 1. / 3.);
    (r, g, b)
}

//include!("extended.rs");

static COLORS_BASIC: &'static str = include_str!("w3c_basic.txt");
static COLORS_EXTENDED: &'static str = include_str!("w3c_extended.txt");
static COLORS_XKCD: &'static str = include_str!("xkcd.txt");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(Color::name("black"), Some(Color::new(0., 0., 0., 1.)));
        assert_eq!(Color::name("white"), Some(Color::new(1., 1., 1., 1.)));
        assert_eq!(Color::name("red"), Some(Color::new(1., 0., 0., 1.)));
        assert_eq!(
            Color::name("green"),
            Some(Color::new(0., 128. / 255., 0., 1.))
        );
        assert_eq!(Color::name("blue"), Some(Color::new(0., 0., 1., 1.)));
        assert_eq!(Color::name("yellow"), Some(Color::new(1., 1., 0., 1.)));
        assert_eq!(Color::name("cyan"), Some(Color::new(0., 1., 1., 1.)));
        assert_eq!(
            Color::name("orange"),
            Some(Color::new(1., 165. / 255., 0., 1.))
        );
        assert_eq!(Color::name("fuchsia"), Some(Color::new(1., 0., 1., 1.)));
        for name in [
            "black", "silver", "gray", "white", "maroon", "red", "purple", "fuchsia", "green",
            "lime", "olive", "yellow", "navy", "blue", "teal", "aqua",
        ]
        .iter()
        {
            assert_eq!(Color::name(name).is_some(), true);
        }
    }

    #[test]
    fn hex() {
        assert_eq!(Color::name("black").unwrap().to_hex(), "#000000");
        assert_eq!(Color::name("silver").unwrap().to_hex(), "#c0c0c0");
        assert_eq!(Color::name("gray").unwrap().to_hex(), "#808080");
        assert_eq!(Color::name("white").unwrap().to_hex(), "#ffffff");
        assert_eq!(Color::name("maroon").unwrap().to_hex(), "#800000");
        assert_eq!(Color::name("red").unwrap().to_hex(), "#ff0000");
        assert_eq!(Color::name("purple").unwrap().to_hex(), "#800080");
        assert_eq!(Color::name("fuchsia").unwrap().to_hex(), "#ff00ff");
        assert_eq!(Color::name("green").unwrap().to_hex(), "#008000");
        assert_eq!(Color::name("lime").unwrap().to_hex(), "#00ff00");
        assert_eq!(Color::name("olive").unwrap().to_hex(), "#808000");
        assert_eq!(Color::name("yellow").unwrap().to_hex(), "#ffff00");
        assert_eq!(Color::name("navy").unwrap().to_hex(), "#000080");
        assert_eq!(Color::name("blue").unwrap().to_hex(), "#0000ff");
        assert_eq!(Color::name("teal").unwrap().to_hex(), "#008080");
        assert_eq!(Color::name("aqua").unwrap().to_hex(), "#00ffff");

        assert_eq!(Color::name("cyan").unwrap().to_hex(), "#00ffff");
        assert_eq!(Color::name("orange").unwrap().to_hex(), "#ffa500");
    }

    #[test]
    fn extended() {
        let ext_names = [
            "aliceblue",
            "antiquewhite",
            "aqua",
            "aquamarine",
            "azure",
            "beige",
            "bisque",
            "black",
            "blanchedalmond",
            "blue",
            "blueviolet",
            "brown",
            "burlywood",
            "cadetblue",
            "chartreuse",
            "chocolate",
            "coral",
            "cornflowerblue",
            "cornsilk",
            "crimson",
            "cyan",
            "darkblue",
            "darkcyan",
            "darkgoldenrod",
            "darkgray",
            "darkgreen",
            "darkgrey",
            "darkkhaki",
            "darkmagenta",
            "darkolivegreen",
            "darkorange",
            "darkorchid",
            "darkred",
            "darksalmon",
            "darkseagreen",
            "darkslateblue",
            "darkslategray",
            "darkslategrey",
            "darkturquoise",
            "darkviolet",
            "deeppink",
            "deepskyblue",
            "dimgray",
            "dimgrey",
            "dodgerblue",
            "firebrick",
            "floralwhite",
            "forestgreen",
            "fuchsia",
            "gainsboro",
            "ghostwhite",
            "gold",
            "goldenrod",
            "gray",
            "green",
            "greenyellow",
            "grey",
            "honeydew",
            "hotpink",
            "indianred",
            "indigo",
            "ivory",
            "khaki",
            "lavender",
            "lavenderblush",
            "lawngreen",
            "lemonchiffon",
            "lightblue",
            "lightcoral",
            "lightcyan",
            "lightgoldenrodyellow",
            "lightgray",
            "lightgreen",
            "lightgrey",
            "lightpink",
            "lightsalmon",
            "lightseagreen",
            "lightskyblue",
            "lightslategray",
            "lightslategrey",
            "lightsteelblue",
            "lightyellow",
            "lime",
            "limegreen",
            "linen",
            "magenta",
            "maroon",
            "mediumaquamarine",
            "mediumblue",
            "mediumorchid",
            "mediumpurple",
            "mediumseagreen",
            "mediumslateblue",
            "mediumspringgreen",
            "mediumturquoise",
            "mediumvioletred",
            "midnightblue",
            "mintcream",
            "mistyrose",
            "moccasin",
            "navajowhite",
            "navy",
            "oldlace",
            "olive",
            "olivedrab",
            "orange",
            "orangered",
            "orchid",
            "palegoldenrod",
            "palegreen",
            "paleturquoise",
            "palevioletred",
            "papayawhip",
            "peachpuff",
            "peru",
            "pink",
            "plum",
            "powderblue",
            "purple",
            "red",
            "rosybrown",
            "royalblue",
            "saddlebrown",
            "salmon",
            "sandybrown",
            "seagreen",
            "seashell",
            "sienna",
            "silver",
            "skyblue",
            "slateblue",
            "slategray",
            "slategrey",
            "snow",
            "springgreen",
            "steelblue",
            "tan",
            "teal",
            "thistle",
            "tomato",
            "turquoise",
            "violet",
            "wheat",
            "white",
            "whitesmoke",
            "yellow",
            "yellowgreen",
        ];
        for name in ext_names.iter() {
            assert_eq!(Color::name(name).is_some(), true);
        }
    }
    #[test]
    fn bad_name() {
        assert_eq!(Color::name("asdf").is_none(), true);
    }
    #[test]
    fn test_xkcd() {
        xkcd();
        assert_eq!(Color::name("toxic green").is_some(), true);
        assert_eq!(Color::name("blood").is_some(), true);
        assert_eq!(Color::name("vomit").is_some(), true);
        assert_eq!(Color::name("baby poop").is_some(), true);
    }
    #[test]
    fn test_from() {
        let red = Color::name("red").unwrap();
        assert_eq!(Color::from("#ff0000"), red);
        assert_eq!(Color::from("#ff0000".to_string()), red);
        assert_eq!(Color::from((255, 0, 0)), red);
        assert_eq!(Color::from(&[255, 0, 0]), red);
        assert_eq!(Color::from(&[1., 0., 0.]), red);
        let rgb = vec![1., 0., 0.];
        assert_eq!(Color::from(&rgb), red);
        assert_eq!(Color::from(rgb), red);
        let rgb = vec![1.0f32, 0., 0.];
        assert_eq!(Color::from(&rgb), red);
        assert_eq!(Color::from(rgb), red);
        assert_eq!(Color::from("red"), red);
        assert_eq!(Color::from("red".to_string()), red);
    }

    #[test]
    fn test_into() {
        let red = Color::name("red").unwrap();
        assert_eq!(red, (255, 0, 0).into());
        assert_eq!(red, "red".into());
        assert_eq!(red, "#ff0000".into());
        assert_eq!(red, (1.0, 0.0, 0.0).into());
    }
    #[test]
    fn test_display() {
        let red = Color::name("red").unwrap();
        assert_eq!(format!("{}", red), "(1.000, 0.000, 0.000, 1.000)");
    }

    fn assert_tol(a: (f64, f64, f64), b: (f64, f64, f64), tol: f64) {
        if (a.0 - b.0).abs() > tol {
            assert_eq!(a, b);
        }
        if (a.1 - b.1).abs() > tol {
            assert_eq!(a, b);
        }
        if (a.2 - b.2).abs() > tol {
            assert_eq!(a, b);
        }
    }
    /*
    fn cprint(c0: (f64,f64,f64),c1:(f64,f64,f64),c2:(f64,f64,f64),c3:(i64,i64,i64)) {
        println!("{:.5} {:.5} {:.5} -> {:.5} {:.5} {:.5} -> {:.5} {:.5} {:.5} ({:3} {:3} {:3})",
                 c0.0,c0.1,c0.2,
                 c1.0,c1.1,c1.2,
                 c2.0,c2.1,c2.2,
                 c3.0,c3.1,c3.2)
    }
    */
    #[test]
    #[ignore]
    fn yiq() {
        for r in 0..256 {
            for g in 0..256 {
                for b in 0..256 {
                    let rf = f64::from(r) / 255.0;
                    let gf = f64::from(g) / 255.0;
                    let bf = f64::from(b) / 255.0;

                    let (y, i, q) = rgb2yiq(rf, gf, bf);
                    let (r0, g0, b0) = yiq2rgb(y, i, q);
                    //cprint((rf,gf,bf),(y,i,q),(r0,g0,b0),(r,g,b));
                    assert_tol((rf, gf, bf), (r0, g0, b0), 1e-13);
                }
            }
        }
    }
    #[test]
    #[ignore]
    fn hsl() {
        for r in 0..256 {
            for g in 0..256 {
                for b in 0..256 {
                    let rf = f64::from(r) / 255.0;
                    let gf = f64::from(g) / 255.0;
                    let bf = f64::from(b) / 255.0;

                    let (h, s, l) = rgb2hsl(rf, gf, bf);
                    let (r0, g0, b0) = hsl2rgb(h, s, l);
                    //cprint((rf,gf,bf),(h,s,l),(r0,g0,b0),(r,g,b));
                    assert_tol((rf, gf, bf), (r0, g0, b0), 1e-13);
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn hsv() {
        assert_eq!(rgb2hsv(0., 0., 0.), (0., 0., 0.));
        assert_eq!(rgb2hsv(1., 0., 0.), (0., 1., 1.));
        assert_eq!(rgb2hsv(0., 1., 0.), (120., 1., 1.));
        assert_eq!(rgb2hsv(0., 0., 1.), (240., 1., 1.));

        assert_eq!(rgb2hsv(1., 1., 0.), (60., 1., 1.));
        assert_eq!(rgb2hsv(1., 0., 1.), (300., 1., 1.));
        assert_eq!(rgb2hsv(0., 1., 1.), (180., 1., 1.));

        assert_eq!(rgb2hsv(1.0, 0.0, std::f64::EPSILON), (360., 1.0, 1.0));

        assert_eq!(
            rgb2hsv(1. / 255., 1. / 255., 2. / 255.),
            (240., 0.5, 2. / 255.)
        );
        assert_eq!(
            hsv2rgb(240., 0.5, 2. / 255.),
            (1. / 255., 1. / 255., 2. / 255.)
        );

        assert_eq!(
            rgb2hsv(1. / 255., 0. / 255., 1. / 255.),
            (300., 1., 1. / 255.)
        );
        assert_eq!(
            hsv2rgb(300., 1.0, 1. / 255.),
            (1. / 255., 0. / 255., 1. / 255.)
        );

        for r in 0..256 {
            for g in 0..256 {
                for b in 0..256 {
                    let rf = f64::from(r) / 255.0;
                    let gf = f64::from(g) / 255.0;
                    let bf = f64::from(b) / 255.0;

                    let (h, s, v) = rgb2hsv(rf, gf, bf);
                    let (r0, g0, b0) = hsv2rgb(h, s, v);
                    //cprint((rf,gf,bf),(h,s,v),(r0,g0,b0),(r,g,b));
                    assert_tol((rf, gf, bf), (r0, g0, b0), 1e-13);
                }
            }
        }
    }
}
