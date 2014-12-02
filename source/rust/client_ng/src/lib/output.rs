use serialize::json::{
	mod,
	DecodeResult,
};


#[deriving(Decodable, Encodable, Show)]
pub struct Frame {
	pub self_id   : String,
	pub broadcasts: Vec<Broadcast>,
}

impl Frame {
	pub fn from_json(json: &str) -> DecodeResult<Frame> {
		json::decode(json)
	}

	pub fn to_json(&self) -> String {
		json::encode(self)
	}
}


#[deriving(Decodable, Encodable, PartialEq, Show)]
pub struct Broadcast {
	pub sender : String,
	pub message: String,
}
