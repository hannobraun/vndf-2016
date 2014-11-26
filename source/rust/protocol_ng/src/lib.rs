extern crate serialize;


use serialize::json::{
	mod,
	DecodeResult,
};


#[deriving(Decodable, Encodable, Show)]
pub enum Action {
	Login,
	Broadcast(String),
}

impl Action {
	pub fn from_json(json: &str) -> DecodeResult<Action> {
		json::decode(json)
	}

	pub fn to_json(&self) -> String {
		json::encode(self)
	}
}


#[deriving(Decodable, Encodable, Show)]
pub struct Perception {
	pub broadcasts: Vec<String>,
}

impl Perception {
	pub fn from_json(json: &str) -> DecodeResult<Perception> {
		json::decode(json)
	}

	pub fn to_json(&self) -> String {
		json::encode(self)
	}
}
