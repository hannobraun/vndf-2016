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
pub use self::perception::{
	Perception,
	PerceptionHeader,
};


mod action;
mod decode;
mod encode;
mod perception;


pub type Seq = u64;


#[deriving(Clone, PartialEq, Show)]
pub struct Message<Header, Entity> {
	pub header : Header,
	pub update : Vec<Entity>,
	pub destroy: Vec<String>,
}

impl<Header, Entity> Message<Header, Entity>
	where
		Header: Decodable<json::Decoder, json::DecoderError> + Encode,
		Entity: Decodable<json::Decoder, json::DecoderError> + Encode,
{
	pub fn decode(buffer: &[u8]) -> Result<Message<Header, Entity>, String> {
		let mut update = Vec::new();
		match decode(buffer, &mut update) {
			Ok(header) =>
				Ok(Message {
					header : header,
					update : update,
					destroy: Vec::new(),
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

		let mut message = encoder.message(&self.header);
		for entity in self.update.iter() {
			message.update(entity);
		}

		message.encode().to_vec()
	}
}


pub trait Part {
	fn assemble<W: Writer>(&self, writer: &mut W) -> IoResult<()>;
}


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
