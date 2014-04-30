use json::to_json;
use physics::Radians;


#[deriving(Encodable)]
pub struct Input {
	pub attitude: Radians,
	pub send    : bool
}

impl Input {
	pub fn to_json(&self) -> ~str {
		to_json(self)
	}
}
