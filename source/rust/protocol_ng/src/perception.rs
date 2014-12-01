use serialize::Encodable;
use serialize::json;
use std::io::{
	BufReader,
	IoResult,
};

use super::{
	MAX_PACKET_SIZE,
	Seq,
};
use super::buf_writer::BufWriter;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Perception {
	pub last_action: Seq,
	pub percepts   : Vec<Percept>,
}

impl Perception {
	pub fn decode(buffer: &[u8]) -> Result<Perception, String> {
		let mut reader = BufReader::new(buffer);

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

		let last_action = match from_str(header) {
			Some(last_action) =>
				last_action,
			None => {
				return Err(format!("Header is not a number\n"));
			},
		};

		let mut percepts = Vec::new();
		for line in lines.into_iter() {
			if line.len() == 0 {
				continue;
			}

			match json::decode(line) {
				Ok(percept) =>
					percepts.push(percept),
				Err(error) =>
					return Err(format!(
						"Error decoding percept. \
						Error: {}; Percept: {}; Message: {}",
						error, line, message,
					)),
			}
		}

		Ok(Perception {
			last_action: last_action,
			percepts   : percepts,
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


pub struct MessageEncoder<'r> {
	writer: BufWriter<'r>,
}

impl<'r> MessageEncoder<'r> {
	pub fn new(buffer: &mut [u8], confirm_seq: Seq) -> MessageEncoder {
		let mut writer = BufWriter::new(buffer);

		match write!(&mut writer, "{}\n", confirm_seq) {
			Ok(()) =>
				(),
			Err(error) =>
				panic!("Error writing message header: {}", error),
		}

		MessageEncoder {
			writer: writer,
		}
	}

	pub fn add(&mut self, percept: Percept) -> bool {
		let mut buffer = [0, ..MAX_PACKET_SIZE];

		let len = {
			let mut writer = BufWriter::new(&mut buffer);
			match percept.encode(&mut json::Encoder::new(&mut writer)) {
				Ok(())  => (),
				Err(_)  => return false,
			}
			match writer.write_char('\n') {
				Ok(()) => (),
				Err(_) => return false,
			}

			writer.tell().unwrap_or_else(|_|
				panic!(
					"I/O operation on BufWriter that cannot possibly fail \
					still managed to fail somehow."
				)
			)
		};
		let addition = buffer[.. len as uint];

		match self.writer.write(addition) {
			Ok(()) => (),
			Err(_) => return false,
		}

		true
	}

	pub fn encode(self, buffer: &mut [u8]) -> IoResult<&[u8]> {
		let len = {
			let len = self.writer.tell().unwrap_or_else(|_|
				panic!(
					"I/O operation on BufWriter that cannot possibly fail \
					still managed to fail somehow."
				)
			);

			let mut writer = BufWriter::new(buffer);
			match writer.write(self.writer.into_slice()[.. len as uint]) {
				Ok(())     => (),
				Err(error) => return Err(error),
			};

			len
		};

		Ok(buffer[.. len as uint])
	}
}
