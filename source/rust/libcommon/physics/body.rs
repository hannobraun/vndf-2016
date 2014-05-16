use physics::{
	Radians,
	Vec2
};

#[deriving(Clone, Decodable, Encodable, Eq, Show)]
pub struct Body {
	pub position: Vec2,
	pub velocity: Vec2,
	pub attitude: Radians
}
