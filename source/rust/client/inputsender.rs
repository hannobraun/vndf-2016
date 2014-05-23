use time;

use common::io::Input;

use network::Network;


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

	pub fn update(game_input: &mut InputSender, input: Input, network: &mut Network, period_in_ms: u64) {
		game_input.input_to_send.attitude = input.attitude;
		if time::precise_time_ns() >= game_input.time_of_next_send {
			network.send(input);
			game_input.time_of_next_send =
				time::precise_time_ns() + period_in_ms * 1000 * 1000;
		}
	}
}
