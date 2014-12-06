use serialize::json::{
	mod,
	DecodeResult,
};

use common::protocol::Broadcast;


#[deriving(Decodable, Encodable, Show)]
pub struct Frame {
	pub self_id   : String,
	pub input     : String,
	pub status    : String,
	pub commands  : Vec<String>,
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
