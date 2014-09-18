use serialize::json;

use cgmath::Rad;


#[deriving(Decodable, Encodable, PartialEq, Show)]
pub struct Action {
	pub attitude: Rad<f64>,
	pub missile : u64
}

impl Action {
	pub fn from_string(s: &str) -> json::DecodeResult<Action> {
		json::decode(s)
	}

	pub fn to_string(&self) -> String {
		json::encode(self)
	}
}
