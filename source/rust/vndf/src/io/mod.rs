use serialize::json;

use physics::{
	Body,
	Radians,
	Vec2,
};


pub mod cli;
pub mod desktop;


pub trait Platform {
	fn input(&mut self) -> Input;
	fn render(&mut self, frame: &Frame);
}


#[deriving(Decodable, Encodable, Show)]
pub struct Frame {
	pub input   : Input,
	pub camera  : Vec2,
	pub ships   : Vec<Body>,
	pub missiles: Vec<Body>
}

impl Frame {
	pub fn from_json(s: &str) -> json::DecodeResult<Frame> {
		json::decode(s)
	}

	pub fn to_json(&self) -> String {
		json::encode(self)
	}
}


#[deriving(Decodable, Encodable, Show)]
pub struct Input {
	pub exit    : bool,
	pub attitude: Radians,
	pub missile : u64
}

impl Input {
	pub fn default() -> Input {
		Input {
			exit    : false,
			attitude: Radians(0.0),
			missile : 0
		}
	}

	pub fn from_json(s: &str) -> json::DecodeResult<Input> {
		json::decode(s)
	}

	pub fn to_json(&self) -> String {
		json::encode(self)
	}
}
