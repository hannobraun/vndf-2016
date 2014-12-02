use serialize::json::{
	mod,
	DecodeResult,
};


#[deriving(Decodable, Encodable, Show)]
pub struct Frame {
	pub broadcasts: Vec<String>,
}

impl Frame {
	pub fn from_json(json: &str) -> DecodeResult<Frame> {
		json::decode(json)
	}

	pub fn to_json(&self) -> String {
		json::encode(self)
	}
}
