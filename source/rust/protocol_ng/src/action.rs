use serialize::json::{
	mod,
	DecodeResult,
};

use super::Seq;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Action {
	pub seq  : Seq,
	pub steps: Vec<Step>,
}

impl Action {
	pub fn from_json(json: &str) -> DecodeResult<Action> {
		json::decode(json)
	}

	pub fn to_json(&self) -> String {
		json::encode(self)
	}
}


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub enum Step {
	Login,
	Broadcast(String),
}
