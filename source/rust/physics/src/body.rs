use cgmath::{
	Quaternion,
	Vector3,
};


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Body {
	pub position: Vector3<f64>,
	pub velocity: Vector3<f64>,
	pub attitude: Quaternion<f64>,
}

impl Body {
	pub fn default() -> Body {
		Body {
			position: Vector3::zero(),
			velocity: Vector3::zero(),
			attitude: Quaternion::zero(),
		}
	}
}
