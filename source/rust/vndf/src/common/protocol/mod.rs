use common::json::{
	from_json,
	to_json
};
use physics::Radians;


pub use self::perception::Perception;


mod perception;


#[deriving(Decodable, Encodable, PartialEq, Show)]
pub struct Action {
	pub attitude: Radians,
	pub missile : u64
}

impl Action {
	pub fn from_string(s: &str) -> Result<Action, String> {
		from_json(s)
	}

	pub fn to_string(&self) -> String {
		to_json(self)
	}
}
