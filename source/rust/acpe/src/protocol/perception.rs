use root::MAX_PACKET_SIZE;

use super::{
	decode,
	Encoder,
	MessagePart,
	Seq,
};


#[deriving(Clone, PartialEq, Show)]
pub struct Perception<Id, Percept> {
	pub last_action: Seq,
	pub self_id    : Option<Id>,
	pub percepts   : Vec<Percept>,
}

impl<Id, Percept: MessagePart> Perception<Id, Percept> {
	pub fn decode(message: &[u8]) -> Result<Perception<Id, Percept>, String> {
		let mut percepts = Vec::new();
		match decode(message, &mut percepts) {
			Ok(last_action) =>
				Ok(Perception {
					last_action: last_action,
					// TODO: Add support for self id to decode
					self_id    : None,
					percepts   : percepts,
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

		let mut perception = encoder.message(self.last_action);
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
