extern crate serialize;

extern crate cgmath;

extern crate game;


use serialize::json;
use std::num::FloatMath;

use cgmath::{
	Matrix4,
	Point3,
	Quaternion,
	Rad,
	Vector3,
	zero,
};

use game::ecs::Planet;
use game::physics::Body;


pub trait Platform {
	fn input(&mut self) -> Result<Input, String>;
	fn render(&mut self, frame: &Frame);
}


#[deriving(Decodable, Encodable, Show)]
pub struct Frame {
	pub input   : Input,
	pub camera  : Camera,
	pub ships   : Vec<Body>,
	pub missiles: Vec<Body>,
	pub planets : Vec<Planet>,
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
	pub attitude: Quaternion<f64>,
	pub thrust  : bool,
	pub missile : u64,

	pub camera_angle   : (Rad<f64>, Rad<f64>),
	pub camera_distance: f64,
}

impl Input {
	pub fn default() -> Input {
		Input {
			exit    : false,
			attitude: Quaternion::zero(),
			thrust  : false,
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
	pub center     : Vector3<f64>,
	pub perspective: (Rad<f64>, Rad<f64>),
	pub distance   : f64,
}

impl Camera {
	pub fn new() -> Camera {
		Camera {
			center     : zero(),
			perspective: (Rad::zero(), Rad::zero()),
			distance   : 500.0,
		}
	}

	pub fn eye(&self) -> Vector3<f64> {
		let (phi, theta) = self.perspective;

		self.center + Vector3::new(
			self.distance * theta.s.sin() * phi.s.cos(),
			self.distance * theta.s.sin() * phi.s.sin(),
			self.distance * theta.s.cos(),
		)
	}

	pub fn to_transform(&self) -> Matrix4<f32> {
		let eye = self.eye();

		Matrix4::look_at(
			&Point3::new(
				eye.x as f32,
				eye.y as f32,
				eye.z as f32,
			),
			&Point3::new(
				self.center.x as f32,
				self.center.y as f32,
				self.center.z as f32,
			),
			&Vector3::new(0.0, 0.0, 1.0),
		)
	}
}
