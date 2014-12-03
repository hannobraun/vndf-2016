use std::io::IoResult;

use root::MAX_PACKET_SIZE;

use super::{
	decode,
	Encoder,
	Header,
	Message,
	Part,
	Seq,
};


#[deriving(Clone, PartialEq, Show)]
pub struct Perception<Id, Percept> {
	pub header  : PerceptionHeader<Id>,
	pub percepts: Vec<Percept>,
}

impl<Id, Percept: Part> Perception<Id, Percept> {
	pub fn decode(message: &[u8]) -> Result<Perception<Id, Percept>, String> {
		let mut percepts = Vec::new();
		// TODO: Simplify generic arguments
		match decode::<Perception<Id, _>, _, _>(message, &mut percepts) {
			Ok(header) =>
				Ok(Perception {
					header  : header,
					percepts: percepts,
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
		let mut perception = encoder.message::<Perception<Id, _>, _, _>(self.header);
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

impl<Id, P: Part> Message<PerceptionHeader<Id>, P> for Perception<Id, P> {}


#[deriving(Clone, PartialEq, Show)]
pub struct PerceptionHeader<Id> {
	pub confirm_action: Seq,
	pub self_id       : Option<Id>,
}

impl<Id> Header for PerceptionHeader<Id> {
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		write!(writer, "{}\n", self.confirm_action)
	}

	fn read(line: &str) -> Result<PerceptionHeader<Id>, String> {
		match from_str(line) {
			Some(action_id) => Ok(PerceptionHeader {
				confirm_action: action_id,
				self_id       : None,
			}),
			None => Err(format!("Header is not a number")),
		}
	}
}
