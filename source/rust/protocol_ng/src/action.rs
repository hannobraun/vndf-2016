use serialize::json::{
	mod,
	DecodeResult,
	DecoderError,
};
use std::str::from_utf8;

use super::Seq;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Action {
	pub seq  : Seq,
	pub steps: Vec<Step>,
}

impl Action {
	pub fn from_json(buffer: &[u8]) -> DecodeResult<Action> {
		let message = match from_utf8(buffer) {
			Some(message) =>
				message,
			None =>
				return Err(
					DecoderError::ApplicationError(
						format!("Received invalid UTF-8 string: {}", buffer)
					)
				)
		};

		json::decode(message)
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
