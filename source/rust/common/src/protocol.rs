use std::io::IoResult;

use acpe::protocol::Part;
use rustc_serialize::Encodable;
use rustc_serialize::json;


#[deriving(Clone, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub enum Step {
	Login,
	Broadcast(String),
	StopBroadcast,
}

impl Part for Step {
	fn assemble<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		self.encode(&mut json::Encoder::new(writer))
	}
}


#[deriving(Clone, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub enum Percept {
	Broadcast(Broadcast),
}

impl Part for Percept {
	fn assemble<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		self.encode(&mut json::Encoder::new(writer))
	}
}


#[deriving(Clone, RustcDecodable, RustcEncodable, PartialEq, Show)]
pub struct Broadcast {
	pub sender : String,
	pub message: String,
}
