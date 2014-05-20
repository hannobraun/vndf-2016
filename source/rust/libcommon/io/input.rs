use json::{
	from_json,
	to_json
};
use physics::Radians;


#[deriving(Decodable, Encodable)]
pub struct Input {
	pub exit    : bool,
	pub attitude: Radians,
	pub missile : bool
}

impl Input {
	pub fn default() -> Input {
		Input {
			exit    : false,
			attitude: Radians(0.0),
			missile : false
		}
	}

	pub fn from_json(s: &str) -> Result<Input, ~str> {
		from_json(s)
	}

	pub fn to_json(&self) -> ~str {
		to_json(self)
	}
}
