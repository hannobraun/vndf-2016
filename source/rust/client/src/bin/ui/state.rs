use render::Pos;

use super::base::{
	ProcessInput,
	Status,
};


pub struct BroadcastForm {
	pub text_field       : TextField,
	pub button           : Button,
	pub text_field_status: Status,

	// Transient state
	pub button_width : Pos,
}

impl BroadcastForm {
	pub fn new() -> BroadcastForm {
		BroadcastForm {
			text_field: TextField::new(),
			button    : Button::new(),

			text_field_status: Status::Passive,
			button_width     : 1,
		}
	}
}


pub struct Button {
	pub was_activated: bool,
}

impl Button {
	pub fn new() -> Button {
		Button {
			was_activated: false,
		}
	}
}


pub struct CommTab {
	pub selected_index: u8,
	pub broadcast_form: BroadcastForm,
	pub broadcast_list: List,
}

impl CommTab {
	pub fn new() -> CommTab {
		CommTab {
			selected_index: 0,
			broadcast_form: BroadcastForm::new(),
			broadcast_list: List::new(),
		}
	}

	pub fn selected_element_mut(&mut self) -> &mut ProcessInput {
		match self.selected_index % 2 {
			0 => &mut self.broadcast_form,
			1 => &mut self.broadcast_list,
			_ => panic!("This should not happen"),
		}
	}

	pub fn form_is_selected(&self) -> bool {
		self.selected_index % 2 == 0
	}

	pub fn list_is_selected(&self) -> bool {
		self.selected_index % 2 == 1
	}
}


pub struct InfoSection {
	pub width : Pos,
	pub height: Pos,
}

impl InfoSection {
	pub fn new(width: Pos, height: Pos) -> InfoSection {
		InfoSection {
			width : width,
			height: height,
		}
	}
}


pub struct List {
	pub activated: bool,
	pub first    : usize,

	// Transient state
	pub status: Status,
}

impl List {
	pub fn new() -> List {
		List {
			activated: false,
			first    : 0,

			status: Status::Passive,
		}
	}
}


pub struct MainSection {
	pub width : Pos,
	pub height: Pos,

	pub tab_switcher: TabSwitcher,
}

impl MainSection {
	pub fn new(width: Pos, height: Pos) -> MainSection {
		MainSection {
			width : width,
			height: height,

			tab_switcher: TabSwitcher::new(),
		}
	}
}


pub struct TabHeader {
	// Transient data
	pub status: Status,
}

impl TabHeader {
	pub fn new() -> TabHeader {
		TabHeader {
			status: Status::Passive,
		}
	}
}


pub struct TabSwitcher {
	pub comm_header: TabHeader,
	pub nav_header : TabHeader,

	pub comm_tab: CommTab,
}

impl TabSwitcher {
	pub fn new() -> TabSwitcher {
		TabSwitcher {
			comm_header: TabHeader::new(),
			nav_header : TabHeader::new(),

			comm_tab: CommTab::new(),
		}
	}
}


pub struct TextField {
	pub text: String,
}

impl TextField {
	pub fn new() -> TextField {
		TextField {
			text: String::new(),
		}
	}
}
