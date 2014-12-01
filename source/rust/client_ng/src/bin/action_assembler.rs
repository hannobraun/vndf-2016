use acpe::MAX_PACKET_SIZE;

use protocol_ng::{
	Encoder,
	Seq,
	Step,
};


pub struct ActionAssembler<'r> {
	buffer   : [u8, ..MAX_PACKET_SIZE],
	next_seq : Seq,
	added    : Vec<Step>,
	assembled: Option<&'r [u8]>,
}

impl<'r> ActionAssembler<'r> {
	pub fn new() -> ActionAssembler<'r> {
		ActionAssembler {
			buffer   : [0, ..MAX_PACKET_SIZE],
			next_seq : 0,
			added    : Vec::new(),
			assembled: None,
		}
	}

	pub fn add_step(&mut self, step: Step) {
		self.added.push(step);
	}

	pub fn assemble(&mut self, encoder: &mut Encoder) -> &[u8] {
		match self.assembled {
			Some(message) => return message.clone(),
			None          => (),
		}

		let mut action = encoder.message(self.next_seq);

		loop {
			let step = match self.added.remove(0) {
				Some(step) => step,
				None       => break,
			};

			if !action.add(&step) {
				self.added.insert(0, step);
				break;
			}
		}

		let message = action
			.encode(&mut self.buffer)
			.unwrap_or_else(|error|
				panic!("Error encoding action: {}", error)
			);

		self.assembled = Some(message.clone());
		message
	}

	pub fn process_receipt(&mut self, seq: Seq) {
		let is_confirmed = match self.assembled {
			Some(_) => seq >= self.next_seq,
			None    => false,
		};

		if is_confirmed {
			self.assembled = None;
			self.next_seq += 1;
		}
	}
}
