use time;

use client::network::Network;
use io::Input;


pub struct InputSender {
	period_in_ms     : u64,
	time_of_next_send: u64,
	input_to_send    : Input
}

impl InputSender {
	pub fn new(period_in_ms: u64) -> InputSender {
		InputSender {
			period_in_ms     : period_in_ms,
			time_of_next_send: 0,
			input_to_send    : Input::default()
		}
	}

	pub fn update(&mut self, input: Input, network: &mut Network) {
		self.input_to_send.attitude = input.attitude;
		if time::precise_time_ns() >= self.time_of_next_send {
			network.send(input);
			self.time_of_next_send =
				time::precise_time_ns() + self.period_in_ms * 1000 * 1000;
		}
	}
}
