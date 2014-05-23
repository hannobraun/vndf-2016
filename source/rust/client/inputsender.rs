use common::io::Input;


pub struct InputSender {
	pub time_of_next_send: u64,
	pub input_to_send    : Input
}
