use std::io::{
	BufReader,
	IoResult,
};

use super::{
	MAX_PACKET_SIZE,
	Encoder,
	Seq,
};
use super::buf_writer::BufWriter;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Perception {
	pub last_action: Seq,
	pub broadcasts : Vec<String>,
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

		let mut broadcasts = Vec::new();
		for line in lines.iter() {
			let mut splits: Vec<String> = line.splitn(1, ' ')
				.map(|s| s.to_string())
				.collect();

			if splits.len() != 2 && splits[0].as_slice() != "UPDATE" {
				continue;
			}

			let broadcast = match splits.pop() {
				Some(broadcast) =>
					broadcast,
				None => {
					return Err(
						format!("Invalid line, broadcast missing: {}\n", line)
					);
				},
			};

			broadcasts.push(broadcast);
		}

		Ok(Perception {
			last_action: last_action,
			broadcasts : broadcasts,
		})
	}

	/// This is a convenience method that makes encoding as easy as possible,
	/// ignoring performance and error handling. Please don't use this outside
	/// of test code.
	pub fn encode(&self) -> Vec<u8> {
		let mut encoder = Encoder::new();
		let mut buffer  = Vec::from_elem(MAX_PACKET_SIZE, 0);

		let mut perception = encoder.perception(self.last_action);
		for broadcast in self.broadcasts.iter() {
			perception.add(broadcast.as_slice());
		}

		perception
			.encode(buffer.as_mut_slice())
			.unwrap_or_else(|error|
				panic!("Error encoding perception: {}", error)
			);
		buffer
	}
}


pub struct PerceptionEnc<'r> {
	writer: BufWriter<'r>,
}

impl<'r> PerceptionEnc<'r> {
	pub fn new(buffer: &mut [u8], confirm_seq: Seq) -> PerceptionEnc {
		let mut writer = BufWriter::new(buffer);

		match write!(&mut writer, "{}\n", confirm_seq) {
			Ok(()) =>
				(),
			Err(error) =>
				panic!("Error writing message header: {}", error),
		}

		PerceptionEnc {
			writer: writer,
		}
	}

	pub fn add(&mut self, broadcast: &str) -> bool {
		let mut buffer = [0, ..MAX_PACKET_SIZE];

		let len = {
			let mut writer = BufWriter::new(&mut buffer);
			match write!(&mut writer, "UPDATE {}\n", broadcast) {
				Ok(())  => (),
				Err(_)  => return false,
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
			let mut writer = BufWriter::new(buffer);
			match writer.write(self.writer.into_slice()) {
				Ok(())     => (),
				Err(error) => return Err(error),
			};

			writer.tell().unwrap_or_else(|_|
				panic!(
					"I/O operation on BufWriter that cannot possibly fail \
					still managed to fail somehow."
				)
			)
		};

		Ok(buffer[.. len as uint])
	}
}
