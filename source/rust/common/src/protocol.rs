use std::io::IoResult;

use acpe::protocol::Part;
use rustc_serialize::Encodable;
use rustc_serialize::json;


#[deriving(Clone, RustcDecodable, RustcEncodable, PartialEq, Show)]
pub enum Step {
	Login,
	Broadcast(String),
	StopBroadcast,
}

impl Part for Step {
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


#[deriving(Clone, RustcDecodable, RustcEncodable, PartialEq, Show)]
pub enum Percept {
	Broadcast(Broadcast),
}

impl Part for Percept {
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


#[deriving(Clone, RustcDecodable, RustcEncodable, PartialEq, Show)]
pub struct Broadcast {
	pub sender : String,
	pub message: String,
}
