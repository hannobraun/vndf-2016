use serialize::json;
use std::error::Error;
use std::str::from_utf8;

use super::Seq;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Action {
	pub seq  : Seq,
	pub steps: Vec<Step>,
}

impl Action {
	pub fn decode(buffer: &[u8]) -> Result<Action, String> {
		let message = match from_utf8(buffer) {
			Some(message) =>
				message,
			None =>
				return Err(
					format!("Received invalid UTF-8 string: {}", buffer)
				),
		};

		match json::decode(message) {
			Ok(action) => Ok(action),
			Err(error) => Err(error.description().to_string()),
		}
	}

	pub fn encode(self) -> Vec<u8> {
		json::encode(&self).into_bytes()
	}
}


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub enum Step {
	Login,
	Broadcast(String),
}
