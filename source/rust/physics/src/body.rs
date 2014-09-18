use cgmath::Vector2;

use super::{
	Radians,
	Vec2
};


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Body {
	pub position: Vector2<f64>,
	pub velocity: Vec2,
	pub attitude: Radians
}

impl Body {
	pub fn default() -> Body {
		Body {
			position: Vector2::zero(),
			velocity: Vec2::zero(),
			attitude: Radians(0.0)
		}
	}
}
