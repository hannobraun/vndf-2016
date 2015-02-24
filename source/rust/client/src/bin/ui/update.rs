use std::cmp::{
	max,
	min,
};
use std::old_io::IoResult;

use render::{
	Pos,
	ScreenBuffer,
};

use super::base::{
	Status,
	Update,
};
use super::state::{
	BroadcastForm,
	Button,
	CommTab,
	InfoSection,
	List,
	MainSection,
	TabHeader,
	TabSwitcher,
	TextField,
};


const START_BROADCAST: &'static str = "Send Broadcast";
const STOP_BROADCAST : &'static str = "Stop Sending";

pub struct BroadcastFormArgs {
	pub is_selected: bool,
	pub is_sending : bool,
}

impl Update for BroadcastForm {
	type Args = BroadcastFormArgs;

	fn update(&mut self, _: &mut ScreenBuffer, _: Pos, _: Pos, args: &BroadcastFormArgs) -> IoResult<()> {
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

		Ok(())
	}
}


impl Update for Button {
	type Args = ();

	fn update(&mut self, _: &mut ScreenBuffer, _: Pos, _: Pos, _: &()) -> IoResult<()> {
		Ok(())
	}
}


pub struct CommTabArgs {
	pub is_sending : bool,
	pub list_length: usize,
	pub list_height: Pos,
}

impl Update for CommTab {
	type Args = CommTabArgs;

	fn update(&mut self, b: &mut ScreenBuffer, x: Pos, y: Pos, args: &CommTabArgs) -> IoResult<()> {
		let form_is_selected = self.form_is_selected();
		let list_is_selected = self.list_is_selected();

		try!(self.broadcast_form.update(
			b,
			x, y,
			&BroadcastFormArgs {
				is_selected: form_is_selected,
				is_sending : args.is_sending,
			},
		));

		self.broadcast_list.update(
			b,
			x, y,
			&ListArgs {
				is_selected: list_is_selected,
				length     : args.list_length,
				height     : args.list_height,
			},
		)
	}
}


impl Update for InfoSection {
	type Args = ();

	fn update(&mut self, _: &mut ScreenBuffer, _: Pos, _: Pos, _: &()) -> IoResult<()> {
		Ok(())
	}
}


pub struct ListArgs {
	pub is_selected: bool,
	pub length     : usize,
	pub height     : Pos,
}

impl Update for List {
	type Args = ListArgs;

	fn update(&mut self, _: &mut ScreenBuffer, _: Pos, _: Pos, args: &ListArgs) -> IoResult<()> {
		self.status = if args.is_selected {
			if self.activated {
				Status::Active
			}
			else {
				Status::Selected
			}
		}
		else {
			Status::Passive
		};

		let max_first = max(0, args.length as isize - args.height as isize);
		self.first = min(self.first, max_first as usize);

		Ok(())
	}
}


pub struct MainArgs {
	pub is_sending : bool,
	pub list_length: usize,
	pub list_height: Pos,
}

impl Update for MainSection {
	type Args = MainArgs;

	fn update(&mut self, b: &mut ScreenBuffer, x: Pos, y: Pos, args: &MainArgs) -> IoResult<()> {
		self.tab_switcher.update(
			b,
			x, y,
			&TabSwitcherArgs {
				is_sending : args.is_sending,
				list_length: args.list_length,
				list_height: args.list_height,
			},
		)
	}
}


impl Update for TabHeader {
	type Args = ();

	fn update(&mut self, _: &mut ScreenBuffer, _: Pos, _: Pos, _: &()) -> IoResult<()> {
		Ok(())
	}
}


pub struct TabSwitcherArgs {
	pub is_sending : bool,
	pub list_length: usize,
	pub list_height: Pos,
}

impl Update for TabSwitcher {
	type Args = TabSwitcherArgs;

	fn update(&mut self, b: &mut ScreenBuffer, x: Pos, y: Pos, args: &TabSwitcherArgs) -> IoResult<()> {
		// TODO: Set currently selected TabHeader to active.

		try!(self.comm_header.update(b, x, y, &()));
		try!(self.nav_header.update(b, x, y, &()));
		self.comm_tab.update(
			b,
			x, y,
			&CommTabArgs {
				is_sending : args.is_sending,
				list_length: args.list_length,
				list_height: args.list_height,
			},
		)
	}
}


impl Update for TextField {
	type Args = ();

	fn update(&mut self, _: &mut ScreenBuffer, _: Pos, _: Pos, _: &()) -> IoResult<()> {
		Ok(())
	}
}
