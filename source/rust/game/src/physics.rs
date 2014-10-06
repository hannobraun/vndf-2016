use cgmath::{
	Quaternion,
	Vector3,
};


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Body {
	pub position: Vector3<f64>,
	pub velocity: Vector3<f64>,
	pub force   : Vector3<f64>,
	pub attitude: Quaternion<f64>,
}

impl Body {
	pub fn new() -> Body {
		Body {
			position: Vector3::zero(),
			velocity: Vector3::zero(),
			force   : Vector3::zero(),
			attitude: Quaternion::zero(),
		}
	}

	pub fn with_position(mut self, position: Vector3<f64>) -> Body {
		self.position = position;
		self
	}

	pub fn with_velocity(mut self, velocity: Vector3<f64>) -> Body {
		self.velocity = velocity;
		self
	}

	pub fn with_attitude(mut self, attitude: Quaternion<f64>) -> Body {
		self.attitude = attitude;
		self
	}
}
