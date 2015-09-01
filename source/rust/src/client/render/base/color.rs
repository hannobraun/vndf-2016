extern crate rand;

use rustc_serialize::hex::{FromHex, ToHex};

pub type Color = [f32;3];

pub trait Colorable {
    fn from_bytes(b: &Vec<u8>) -> Color;
    fn from_hex(hex: &str) -> Option<Color>;
    fn to_bytes(&self) -> Vec<u8>;
    fn to_hex(&self) -> String;
    fn mix(&self, other: Color) -> Color;
    fn add(&self, other: Color) -> Color;
    fn greyscale_ntsc(&self) -> Color;
    fn greyscale_atsc(&self) -> Color;
}

// TODO: implement rgba alpha, this will change mix functions
impl Colorable for Color {
    fn from_bytes(b: &Vec<u8>) -> Color {
        [b[0] as f32 / 255.0,
         b[1] as f32 / 255.0,
         b[2] as f32 / 255.0]
    }
    fn from_hex(hex: &str) -> Option<Color> {
        if let Ok(ref _hex) = hex.from_hex() {
            return Some(Color::from_bytes(_hex))
        }

        None
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![(self[0] * 255.0) as u8,
             (self[1] * 255.0) as u8,
             (self[2] * 255.0) as u8]
    }
    fn to_hex(&self) -> String {
        let r = self.to_bytes();
        let mut v: Vec<u8> = vec!();
        v.extend(&r);
        v.to_hex()
    }

    // NOTE: this may be separated out to different trait
    /// this will mix two colors, additively based on average (natural)
    fn mix(&self, other: Color) -> Color {
        [(self[0]+other[0])/2.0,
         (self[1]+other[1])/2.0,
         (self[2]+other[2])/2.0,]
    }
    /// this will add two colors, typical RGB additive
    fn add(&self, other: Color) -> Color {
        [min((self[0]+other[0]),255.0),
         min((self[1]+other[1]),255.0),
         min((self[2]+other[2]),255.0),]
    }
    /// convert to greyscale
    /// https://en.wikipedia.org/wiki/Grayscale#Luma_coding_in_video_systems
    // TODO: fix precision mismatch with clamping?
    // eg: cyan should equal b3b3b3, not b2b2b2
    fn greyscale_ntsc(&self) -> Color {
        let gs =
            self[0] * 0.2989 +
            self[1] * 0.5870 +
            self[2] * 0.1140;

        [gs,gs,gs]
    }
    fn greyscale_atsc(&self) -> Color {
        let gs =
            self[0] * 0.2126 +
            self[1] * 0.7152 +
            self[2] * 0.0722;

        [gs,gs,gs]
    }
}


// predefined colors
pub struct Colors;
impl Colors {
    pub fn black() -> Color {
        [0.0,0.0,0.0]
    }
    pub fn white() -> Color {
        [1.0,1.0,1.0]
    }
    
    pub fn red() -> Color {
        [1.0,0.0,0.0]
    }
    pub fn green() -> Color {
        [0.0,1.0,0.0]
    }
    pub fn blue() -> Color {
        [0.0,0.0,1.0]
    }

    pub fn yellow() -> Color {
        [1.0,1.0,0.0]
    }
    pub fn magenta() -> Color {
        [1.0,0.0,1.0]
    }
    pub fn cyan() -> Color {
        [0.0,1.0,1.0]
    }

    pub fn orange() -> Color {
        Color::from_bytes(&vec!(255, 165, 0))
    }
    pub fn indigo() -> Color {
        Color::from_bytes(&vec!(75, 0, 130))
    }
    pub fn violet() -> Color {
        Color::from_bytes(&vec!(238, 130, 238))
    }

    pub fn red_brick() -> Color {
        Color::from_bytes(&vec!(132,31,39))
    }
    pub fn green_spring() -> Color {
        Color::from_bytes(&vec!(0,255,127))
    }
    pub fn blue_sky() -> Color {
        Color::from_bytes(&vec!(135, 206, 235))
    }
    pub fn gold() -> Color {
        Color::from_bytes(&vec!(255, 215, 0))
    }
    pub fn grey_dark() -> Color {
        Color::from_bytes(&vec!(105,105,105))
    }
    pub fn grey_light() -> Color {
        Color::from_bytes(&vec!(190,190,190))
    }
    pub fn white_smoke() -> Color {
        Color::from_bytes(&vec!(245,245,245))
    }
    pub fn white_ghost() -> Color {
        Color::from_bytes(&vec!(248,248,255))
    }
    
    pub fn random() -> Color {
        Color::from_bytes(&vec!(rand::random::<u8>(),
                                rand::random::<u8>(),
                                rand::random::<u8>()))
    }
}


fn min(a: f32,b: f32) -> f32 {
    if a < b { a }
    else { b }
}
