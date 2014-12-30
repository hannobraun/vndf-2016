use std::io::{
	IoError,
	IoResult,
};

use rustc_serialize::{
	json,
	Decodable,
	Encodable,
};


pub use self::action::{
	Action,
	ActionHeader,
};
pub use self::decode::decode;
pub use self::encode::{
	Encoder,
	MessageEncoder,
};
pub use self::message::Message;
pub use self::perception::{
	Perception,
	PerceptionHeader,
};


mod action;
mod decode;
mod encode;
mod message;
mod perception;


pub type Seq = u64;


pub trait Encode {
	fn encode<W: Writer>(&self, writer: &mut W) -> IoResult<()>;
}

impl<'e, T> Encode for T where T: Encodable<json::Encoder<'e>, IoError> {
	fn encode<'a, W: Writer>(&self, writer: &'a mut W) -> IoResult<()> {
		// The API used here is inefficient, since it allocates a String for
		// each encoding. There's a more efficient, Writer-based one, but I
		// couldn't get it to work due to lifetime issues. This should be good
		// enough for now.
		write!(writer, "{}", json::encode(self))
	}
}


pub trait Decode {
	fn decode(s: &str) -> Result<Self, String>;
}

impl<T> Decode for T where T: Decodable<json::Decoder, json::DecoderError> {
	fn decode(s: &str) -> Result<Self, String> {
		json::decode(s)
			.map_err(|error|
				format!("JSON decoding error: {}", error)
			)
	}
}
