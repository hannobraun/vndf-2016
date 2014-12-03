use serialize::Encodable;
use serialize::json;
use std::io::IoResult;

use root::MAX_PACKET_SIZE;

use super::{
	decode,
	Encoder,
	Header,
	Message,
	Part,
	Seq,
};


#[deriving(Clone, PartialEq, Show)]
pub struct Perception<Percept> {
	pub header  : PerceptionHeader,
	pub percepts: Vec<Percept>,
}

impl<P: Part> Perception<P> {
	pub fn decode(message: &[u8]) -> Result<Perception<P>, String> {
		let mut percepts = Vec::new();
		match decode(message, &mut percepts) {
			Ok(header) =>
				Ok(Perception {
					header  : header,
					percepts: percepts,
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
		let mut encoder = Encoder::new();

		let mut perception = encoder.message(&self.header);
		for percept in self.percepts.iter() {
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

impl<P: Part> Message<PerceptionHeader, P> for Perception<P> {}


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct PerceptionHeader {
	pub confirm_action: Seq,
	pub self_id       : Option<String>,
}

impl Header for PerceptionHeader {
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		try!(self.encode(&mut json::Encoder::new(writer)));
		try!(writer.write_char('\n'));

		Ok(())
	}

	fn read(line: &str) -> Result<PerceptionHeader, String> {
		json::decode(line)
			.map_err(|error|
				format!("Error decoding perception header: {}", error)
			)
	}
}
