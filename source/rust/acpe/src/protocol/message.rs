use std::default::Default;

use super::{
	decode,
	Decode,
	Encode,
	Encoder,
};


#[deriving(Clone, PartialEq, Show)]
pub struct Message<Header, Id, Entity> {
	pub header : Header,
	pub update : Vec<Entity>,
	pub destroy: Vec<Id>,
}

impl<Header, Id, Entity> Message<Header, Id, Entity>
	where
		Header: Decode + Encode + Default,
		Id    : Decode + Encode,
		Entity: Decode + Encode,
{
	pub fn decode(
		buffer: &[u8]
	) -> Result<Message<Header, Id, Entity>, String> {
		let mut message = Message {
			header : Default::default(),
			update : Vec::new(),
			destroy: Vec::new(),
		};

		match decode(buffer, &mut message) {
			Ok(())     => Ok(message),
			Err(error) => Err(error),
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
