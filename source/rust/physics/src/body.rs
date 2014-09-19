use cgmath::{
	Rad,
	Vector2,
	Vector3,
};


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Body {
	pub position: Vector3<f64>,
	pub velocity: Vector2<f64>,
	pub attitude: Rad<f64>,
}

impl Body {
	pub fn default() -> Body {
		Body {
			position: Vector3::zero(),
			velocity: Vector2::zero(),
			attitude: Rad::zero(),
		}
	}
}
