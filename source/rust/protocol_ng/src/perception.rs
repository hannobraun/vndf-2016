use serialize::Encodable;
use serialize::json;
use std::io::{
	BufReader,
	IoResult,
};

use super::{
	MAX_PACKET_SIZE,
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
		let mut reader = BufReader::new(message);

		let message = match reader.read_to_string() {
			Ok(message) =>
				message,
			Err(error) => {
				return Err(
					format!("Error converting message to string: {}\n", error)
				);
			},
		};

		let mut lines: Vec<&str> = message.split('\n').collect();

		let header = match lines.remove(0) {
			Some(header) =>
				header,
			None => {
				return Err(format!("Header line is missing\n"));
			},
		};

		let confirmed_seq = match from_str(header) {
			Some(confirmed_seq) =>
				confirmed_seq,
			None => {
				return Err(format!("Header is not a number\n"));
			},
		};

		let mut parts = Vec::new();
		for line in lines.into_iter() {
			if line.len() == 0 {
				continue;
			}

			match json::decode(line) {
				Ok(part) =>
					parts.push(part),
				Err(error) =>
					return Err(format!(
						"Error decoding percept. \
						Error: {}; Percept: {}; Message: {}",
						error, line, message,
					)),
			}
		}

		Ok(Perception {
			last_action: confirmed_seq,
			percepts   : parts,
		})
	}

	/// This is a convenience method that makes encoding as easy as possible,
	/// ignoring performance and error handling. Please don't use this outside
	/// of test code.
	pub fn encode(self) -> Vec<u8> {
		let mut encoder = super::Encoder::new();
		let mut buffer  = Vec::from_elem(MAX_PACKET_SIZE, 0);

		let mut perception = encoder.perception(self.last_action);
		for percept in self.percepts.into_iter() {
			perception.add(percept);
		}

		perception
			.encode(buffer.as_mut_slice())
			.unwrap_or_else(|error|
				panic!("Error encoding perception: {}", error)
			);
		buffer
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
}
