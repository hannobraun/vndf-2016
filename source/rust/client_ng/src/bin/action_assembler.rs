use acpe::protocol::{
	ActionHeader,
	Encoder,
	Seq,
};

use common::protocol::Step;


pub struct ActionAssembler {
	next_seq : Seq,
	added    : Vec<Step>,
	assembled: Option<Vec<u8>>,
}

impl ActionAssembler {
	pub fn new() -> ActionAssembler {
		ActionAssembler {
			next_seq : 0,
			added    : Vec::new(),
			assembled: None,
		}
	}

	pub fn add_step(&mut self, step: Step) {
		self.added.push(step);
	}

	// TODO(85118724): If no action is confirmed, this will keep returning the
	//                 same message over and over. This is fine in production,
	//                 since messages are usually confirmed, but it complicates
	//                 writing test cases with a mock game service. A better
	//                 behavior would be this: If new steps have been added
	//                 since the last assembling, the message should be
	//                 re-assembled with the steps being added.
	pub fn assemble(&mut self, encoder: &mut Encoder) -> Vec<u8> {
		match self.assembled {
			Some(ref message) => return message.clone(),
			None              => (),
		}

		let mut action = encoder.message(&ActionHeader { id: self.next_seq });

		let mut is_first_step = true;
		loop {
			let step = if self.added.len() > 0 {
				self.added.remove(0)
			}
			else {
				break
			};

			if !action.update(&step) {
				if is_first_step {
					panic!(
						"Failed to add first step of an action. Since the \
						action is still empty when adding the first step, this \
						means the step is too large to ever be added to an \
						action. This is a bug, as such a step should have been \
						rejected when it was created."
					);
				}
				self.added.insert(0, step);
				break;
			}

			is_first_step = false;
		}

		let message = action.encode();

		let mut assembled = Vec::new();
		assembled.push_all(message);

		self.assembled = Some(assembled.clone());
		assembled
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
