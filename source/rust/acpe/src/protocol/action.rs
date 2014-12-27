use std::io::IoResult;

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
			Ok(header) =>
				Ok(Action {
					header: header,
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
		let mut encoder = Encoder::new();

		let mut action = encoder.message(&self.header);
		for step in self.steps.iter() {
			action.add(step);
		}

		action.encode().to_vec()
	}
}

impl<Step: Part> Message<ActionHeader, Step> for Action<Step> {}


#[deriving(Clone, Copy, PartialEq, Show)]
pub struct ActionHeader {
	pub id: Seq,
}

impl Part for ActionHeader {
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		write!(writer, "{}\n", self.id)
	}

	fn read(line: &str) -> Result<ActionHeader, String> {
		match line.parse() {
			Some(id) => Ok(ActionHeader { id: id }),
			None     => Err(format!("Header is not a number")),
		}
	}
}
