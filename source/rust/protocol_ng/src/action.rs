use serialize::Encodable;
use serialize::json;
use std::io::IoResult;

use acpe::MAX_PACKET_SIZE;

use super::{
	decode,
	MessagePart,
	Seq,
};


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Action {
	pub seq  : Seq,
	pub steps: Vec<Step>,
}

impl Action {
	pub fn decode(buffer: &[u8]) -> Result<Action, String> {
		let mut steps = Vec::new();
		match decode(buffer, &mut steps) {
			Ok(seq) =>
				Ok(Action {
					seq  : seq,
					steps: steps,
				}),
			Err(error) =>
				Err(error),
		}
	}

	pub fn encode(self) -> Vec<u8> {
		let mut buffer  = [0, ..MAX_PACKET_SIZE];
		let mut encoder = super::Encoder::new();

		let mut action = encoder.action(self.seq);
		for step in self.steps.iter() {
			action.add(step);
		}

		let message = action
			.encode(&mut buffer)
			.unwrap_or_else(|error|
				panic!("Error encoding action: {}", error)
			);

		message.to_vec()
	}
}


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub enum Step {
	Login,
	Broadcast(String),
}

impl MessagePart for Step {
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		try!(self.encode(&mut json::Encoder::new(writer)));
		try!(writer.write_char('\n'));

		Ok(())
	}

	fn read(line: &str) -> Result<Step, String> {
		match json::decode(line) {
			Ok(part) =>
				Ok(part),
			Err(error) =>
				Err(format!(
					"Error decoding step. \
					Error: {}; Step: {}",
					error, line,
				)),
		}
	}
}
