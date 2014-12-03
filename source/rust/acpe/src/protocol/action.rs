use root::MAX_PACKET_SIZE;

use super::{
	decode,
	Encoder,
	Message,
	Part,
	Seq,
};


#[deriving(Clone, PartialEq, Show)]
pub struct Action<Step> {
	pub header: ActionHeader,
	pub steps : Vec<Step>,
}

impl<Step: Part> Action<Step> {
	pub fn decode(message: &[u8]) -> Result<Action<Step>, String> {
		let mut steps = Vec::new();
		match decode(message, &mut steps) {
			Ok(seq) =>
				Ok(Action {
					header: ActionHeader { id: seq },
					steps : steps,
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

		// TODO: Simplify generic arguments.
		// TODO: Pass header directly.
		let mut action = encoder.message::<Action<_>, _, _>(self.header.id);
		for step in self.steps.iter() {
			action.add(step);
		}

		let message = action
			.encode(&mut buffer)
			.unwrap_or_else(|error|
				panic!("Error encoding action: {}", error)
			);

		message.to_vec()
	}
}

impl<Step: Part> Message<Seq, Step> for Action<Step> {}


#[deriving(Clone, PartialEq, Show)]
pub struct ActionHeader {
	pub id: Seq,
}
