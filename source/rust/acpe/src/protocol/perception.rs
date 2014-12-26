use std::io::IoResult;

use rustc_serialize::Encodable;
use rustc_serialize::json;

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
	pub header: PerceptionHeader,
	pub update: Vec<Percept>,
}

impl<P: Part> Perception<P> {
	pub fn decode(message: &[u8]) -> Result<Perception<P>, String> {
		let mut update = Vec::new();
		match decode(message, &mut update) {
			Ok(header) =>
				Ok(Perception {
					header: header,
					update: update,
				}),
			Err(error) =>
				Err(error),
		}
	}

	/// This is a convenience method that makes encoding as easy as possible,
	/// ignoring performance and error handling. Please don't use this outside
	/// of test code.
	pub fn encode(self) -> Vec<u8> {
		let mut encoder = Encoder::new();

		let mut perception = encoder.message(&self.header);
		for percept in self.update.iter() {
			perception.add(percept);
		}

		perception.encode().to_vec()
	}
}

impl<P: Part> Message<PerceptionHeader, P> for Perception<P> {}


#[deriving(Clone, RustcDecodable, RustcEncodable, PartialEq, Show)]
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
