use json::{
	from_json,
	to_json
};
use physics::Radians;


#[deriving(Decodable, Encodable)]
pub struct Input {
	pub attitude: Radians,
	pub send    : bool
}

impl Input {
	pub fn from_json(s: &str) -> Input {
		match from_json(s) {
			Ok(input)  => input,
			Err(error) => fail!(error)
		}
	}

	pub fn to_json(&self) -> ~str {
		to_json(self)
	}
}
