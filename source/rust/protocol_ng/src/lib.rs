extern crate serialize;


use serialize::json::{
	mod,
	DecodeResult,
};


#[deriving(Decodable, Encodable, Show)]
pub enum Message {
	Login,
	Broadcast(String),
}

impl Message {
	pub fn from_json(json: &str) -> DecodeResult<Message> {
		json::decode(json)
	}

	pub fn to_json(&self) -> String {
		json::encode(self)
	}
}
