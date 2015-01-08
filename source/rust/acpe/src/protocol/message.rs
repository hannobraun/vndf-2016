use std::default::Default;

use super::{
	decode,
	Decode,
	Encode,
	Encoder,
};


#[derive(Clone, PartialEq, Show)]
pub struct Message<Header, Id, Entity> {
	pub header: Header,

	// TODO: Control flow data

	// Main payload
	pub update : Vec<(Id, Entity)>,
	pub destroy: Vec<Id>,

	// TODO: Additional payload (generic)
}

impl<Header, Id, Entity> Message<Header, Id, Entity>
	where
		Header: Decode + Encode + Default,
		Id    : Decode + Encode,
		Entity: Decode + Encode,
{
	pub fn new() -> Message<Header, Id, Entity> {
		Message {
			header : Default::default(),
			update : Vec::new(),
			destroy: Vec::new(),
		}
	}

	pub fn decode(
		buffer: &[u8]
	) -> Result<Message<Header, Id, Entity>, String> {
		// Performance note:
		// These allocations can be avoided by reusing and existing Message. In
		// that scheme, Message::new() would create an empty message, then
		// message.decode would decode a buffer, saving the result in that
		// message.
		let mut message = Message::new();

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
		for (id, entity) in self.update.into_iter() {
			message.update(&id, &entity);
		}

		message.encode().to_vec()
	}
}
