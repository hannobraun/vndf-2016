use protocol_ng::{
	Action,
	Step,
};


pub struct ActionAssembler {
	added: Vec<Step>,
}

impl ActionAssembler {
	pub fn new() -> ActionAssembler {
		ActionAssembler {
			added: Vec::new(),
		}
	}

	pub fn add_step(&mut self, step: Step) {
		self.added.push(step);
	}

	pub fn assemble(&mut self) -> Action {
		let action = Action {
			// TODO: Set sequence number
			seq  : 0,
			steps: self.added.clone(),
		};
		self.added.clear();

		action
	}
}
