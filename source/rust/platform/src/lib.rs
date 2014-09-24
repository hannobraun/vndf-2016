extern crate serialize;

extern crate cgmath;

extern crate physics;


use serialize::json;

use cgmath::{
	Matrix4,
	Point3,
	Quaternion,
	Rad,
	Vector3,
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
			center     : Vector3::zero(),
			perspective: (Rad::zero(), Rad::zero()),
			distance   : 500.0,
		}
	}

	pub fn to_transform(&self) -> Matrix4<f32> {
		let (phi, theta) = self.perspective;

		let x = self.distance * theta.s.sin() * phi.s.cos();
		let y = self.distance * theta.s.sin() * phi.s.sin();
		let z = self.distance * theta.s.cos();

		Matrix4::look_at(
			&Point3::new(
				(self.center[0] + x) as f32,
				(self.center[1] + y) as f32,
				(self.center[2] + z) as f32,
			),
			&Point3::new(
				self.center[0] as f32,
				self.center[1] as f32,
				self.center[2] as f32,
			),
			&Vector3::new(0.0, 0.0, 1.0),
		)
	}
}
