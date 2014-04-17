use physics::Radians;
use physics::Vec2;

#[deriving(Eq, Show)]
pub struct Body {
	pub position: Vec2,
	pub velocity: Vec2,
	pub attitude: Radians
}
