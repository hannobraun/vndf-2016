use std::io::IoResult;


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
	pub header: Header,
	pub update: Vec<Entity>,
}

impl<Header: Part, Entity: Part> Message<Header, Entity> {
	pub fn decode(buffer: &[u8]) -> Result<Message<Header, Entity>, String> {
		let mut update = Vec::new();
		match decode(buffer, &mut update) {
			Ok(header) =>
				Ok(Message {
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

		let mut message = encoder.message(&self.header);
		for body_part in self.update.iter() {
			message.add(body_part);
		}

		message.encode().to_vec()
	}
}


pub trait Part {
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()>;
	fn read(line: &str) -> Result<Self, String>;
}
