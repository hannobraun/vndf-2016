use super::{
	decode,
	Decode,
	Encode,
	Encoder,
};


#[deriving(Clone, PartialEq, Show)]
pub struct Message<Header, Entity> {
	pub header : Header,
	pub update : Vec<Entity>,
	pub destroy: Vec<String>,
}

impl<Header, Entity> Message<Header, Entity>
	where
		Header: Decode + Encode,
		Entity: Decode + Encode,
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
