use common::json::{
	from_json,
	to_json
};
use common::io::Input;
use physics::{
	Body,
	Vec2
};


#[deriving(Decodable, Encodable, Show)]
pub struct Frame {
	pub input   : Input,
	pub camera  : Vec2,
	pub ships   : Vec<Body>,
	pub missiles: Vec<Body>
}

impl Frame {
	pub fn from_json(s: &str) -> Result<Frame, String> {
		from_json(s)
	}

	pub fn to_json(&self) -> String {
		to_json(self)
	}
}
