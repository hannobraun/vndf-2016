use common::io::Input;


pub struct InputSender {
	pub time_of_next_send: u64,
	pub input_to_send    : Input
}

impl InputSender {
	pub fn new() -> InputSender {
		InputSender {
			time_of_next_send: 0,
			input_to_send    : Input::default()
		}
	}
}
