extern crate serialize;


use serialize::json::{
	mod,
	DecodeResult,
};


pub use action::{
	Action,
	Step,
};


mod action;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Perception {
	pub last_action: u64,
	pub broadcasts : Vec<String>,
}

impl Perception {
	pub fn from_json(json: &str) -> DecodeResult<Perception> {
		json::decode(json)
	}

	pub fn to_json(&self) -> String {
		json::encode(self)
	}
}
