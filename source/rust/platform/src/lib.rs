extern crate serialize;

extern crate physics;


use serialize::json;

use physics::{
	Body,
	Degrees,
	Radians,
	Vec2,
};


pub trait Platform {
	fn input(&mut self) -> Result<Input, String>;
	fn render(&mut self, frame: &Frame);
}


#[deriving(Decodable, Encodable, Show)]
pub struct Frame {
	pub input   : Input,
	pub camera  : Camera,
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
	pub camera  : (Radians, Radians),
	pub missile : u64,
}

impl Input {
	pub fn default() -> Input {
		Input {
			exit    : false,
			attitude: Radians(0.0),
			camera  : (Radians(0.0), Radians(0.0)),
			missile : 0,
		}
	}

	pub fn from_json(s: &str) -> json::DecodeResult<Input> {
		json::decode(s)
	}

	pub fn to_json(&self) -> String {
		json::encode(self)
	}
}


#[deriving(Decodable, Encodable, Show)]
pub struct Camera {
	pub position   : Vec2,
	pub perspective: (Radians, Radians),
}

impl Camera {
	pub fn new() -> Camera {
		Camera {
			position   : Vec2::zero(),
			perspective: (Radians(0.0), Radians(0.0)),
		}
	}
}
