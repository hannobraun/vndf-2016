extern crate serialize;

extern crate cgmath;

extern crate physics;


use serialize::json;

use cgmath::{
	Rad,
	Vector2,
};

use physics::Body;


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
	pub attitude: Rad<f64>,
	pub missile : u64,

	pub camera_angle   : (Rad<f64>, Rad<f64>),
	pub camera_distance: f64,
}

impl Input {
	pub fn default() -> Input {
		Input {
			exit    : false,
			attitude: Rad::zero(),
			missile : 0,

			camera_angle   : (Rad::zero(), Rad::zero()),
			camera_distance: 500.0,
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
	pub center     : Vector2<f64>,
	pub perspective: (Rad<f64>, Rad<f64>),
	pub distance   : f64,
}

impl Camera {
	pub fn new() -> Camera {
		Camera {
			center     : Vector2::zero(),
			perspective: (Rad::zero(), Rad::zero()),
			distance   : 500.0,
		}
	}
}
