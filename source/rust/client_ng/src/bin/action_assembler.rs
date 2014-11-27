use protocol_ng::{
	Action,
	Step,
};


pub struct ActionAssembler;

impl ActionAssembler {
	pub fn new() -> ActionAssembler {
		ActionAssembler
	}

	pub fn assemble(&mut self, step: Step) -> Action {
		Action {
			// TODO: Set sequence number
			seq  : 0,
			steps: vec![step],
		}
	}
}
