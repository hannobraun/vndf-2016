use serialize::json;

use physics::Radians;


#[deriving(Decodable, Encodable, Show)]
pub struct Input {
	pub exit    : bool,
	pub attitude: Radians,
	pub missile : u64
}

impl Input {
	pub fn default() -> Input {
		Input {
			exit    : false,
			attitude: Radians(0.0),
			missile : 0
		}
	}

	pub fn from_json(s: &str) -> json::DecodeResult<Input> {
		json::decode(s)
	}

	pub fn to_json(&self) -> String {
		json::encode(self)
	}
}
