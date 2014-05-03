use json::{
	from_json,
	to_json
};
use physics::{
	Body,
	Vec2
};


#[deriving(Decodable, Encodable)]
pub struct Frame {
	pub camera: Vec2,
	pub ships : ~[Body]
}

impl Frame {
	pub fn from_json(s: &str) -> Frame {
		match from_json(s) {
			Ok(frame)  => frame,
			Err(error) => fail!(error)
		}
	}

	pub fn to_json(&self) -> ~str {
		to_json(self)
	}
}
