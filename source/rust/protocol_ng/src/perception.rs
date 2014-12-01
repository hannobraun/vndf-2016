use serialize::Encodable;
use serialize::json;
use std::io::IoResult;

use super::{
	MAX_PACKET_SIZE,
	decode,
	MessagePart,
	Seq,
};


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Perception {
	pub last_action: Seq,
	pub percepts   : Vec<Percept>,
}

impl Perception {
	pub fn decode(message: &[u8]) -> Result<Perception, String> {
		let mut percepts = Vec::new();
		match decode(message, &mut percepts) {
			Ok(last_action) =>
				Ok(Perception {
					last_action: last_action,
					percepts   : percepts,
				}),
			Err(error) =>
				Err(error),
		}
	}

	/// This is a convenience method that makes encoding as easy as possible,
	/// ignoring performance and error handling. Please don't use this outside
	/// of test code.
	pub fn encode(self) -> Vec<u8> {
		let mut buffer  = [0, ..MAX_PACKET_SIZE];
		let mut encoder = super::Encoder::new();

		let mut perception = encoder.perception(self.last_action);
		for percept in self.percepts.into_iter() {
			perception.add(percept);
		}

		let message = perception
			.encode(&mut buffer)
			.unwrap_or_else(|error|
				panic!("Error encoding perception: {}", error)
			);

		message.to_vec()
	}
}


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub enum Percept {
	Broadcast(String),
}

impl MessagePart for Percept {
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		try!(self.encode(&mut json::Encoder::new(writer)));
		try!(writer.write_char('\n'));

		Ok(())
	}

	fn read(line: &str) -> Result<Percept, String> {
		match json::decode(line) {
			Ok(part) =>
				Ok(part),
			Err(error) =>
				Err(format!(
					"Error decoding part. \
					Error: {}; Part: {}",
					error, line,
				)),
		}
	}
}
