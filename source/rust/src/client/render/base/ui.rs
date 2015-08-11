use rustc_serialize::hex::{FromHex, ToHex};

pub type Color = [f32;3];

pub trait Colorable {
	fn from_bytes(b: &Vec<u8>) -> Color;
	fn from_hex(hex: &str) -> Option<Color>;
	fn to_bytes(&self) -> Vec<u8>;
	fn to_hex(&self) -> String;
}

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
	
}
