use std::cmp::max;

use render::Pos;

use super::base::{
	Status,
	Update,
};
use super::state::{
	BroadcastForm,
	CommTab,
	List,
};


const START_BROADCAST: &'static str = "Send Broadcast";
const STOP_BROADCAST : &'static str = "Stop Sending";

pub struct BroadcastFormArgs {
	pub is_selected: bool,
	pub is_sending : bool,
}

impl Update for BroadcastForm {
	type Args = BroadcastFormArgs;

	fn update(&mut self, args: &BroadcastFormArgs) {
		self.text_field_status =
			if args.is_selected {
				if args.is_sending {
					Status::Selected
				}
				else {
					Status::Active
				}
			}
			else {
				Status::Passive
			};

		self.button_status =
			if args.is_selected {
				Status::Active
			}
			else {
				Status::Passive
			};

		self.button_text =
			if args.is_sending {
				STOP_BROADCAST
			}
			else {
				START_BROADCAST
			};

		self.button_width =
			max(
				START_BROADCAST.chars().count(),
				STOP_BROADCAST.chars().count()
			)
			as Pos;
	}
}


impl Update for CommTab {
	type Args = bool;

	fn update(&mut self, &is_sending: &bool) {
		let form_is_selected = self.form_is_selected();
		let list_is_selected = self.list_is_selected();

		self.broadcast_form.update(&BroadcastFormArgs {
			is_selected: form_is_selected,
			is_sending : is_sending,
		});

		self.broadcast_list.update(&list_is_selected);
	}
}


impl Update for List {
	type Args = bool;

	fn update(&mut self, &is_selected: &bool) {
		self.status = if is_selected {
			Status::Selected
		}
		else {
			Status::Passive
		};
	}
}
