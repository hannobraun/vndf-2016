use serialize::json::{
	mod,
	DecodeResult,
};


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Perception {
	pub last_action: u64,
	pub broadcasts : Vec<String>,
}

impl Perception {
	pub fn decode(json: &str) -> DecodeResult<Perception> {
		json::decode(json)
	}

	pub fn encode(&self) -> String {
		json::encode(self)
	}
}
