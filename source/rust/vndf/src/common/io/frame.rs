use serialize::json;

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
	pub fn from_json(s: &str) -> json::DecodeResult<Frame> {
		json::decode(s)
	}

	pub fn to_json(&self) -> String {
		json::encode(self)
	}
}
