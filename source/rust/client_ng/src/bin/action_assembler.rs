use protocol_ng::{
	Action,
	Step,
};


pub struct ActionAssembler {
	steps: Vec<Step>,
}

impl ActionAssembler {
	pub fn new() -> ActionAssembler {
		ActionAssembler {
			steps: Vec::new(),
		}
	}

	pub fn add_step(&mut self, step: Step) {
		self.steps.push(step);
	}

	pub fn assemble(&mut self) -> Action {
		let action = Action {
			// TODO: Set sequence number
			seq  : 0,
			steps: self.steps.clone(),
		};
		self.steps.clear();

		action
	}
}
