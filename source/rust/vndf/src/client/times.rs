use time::precise_time_s;


pub struct Times {
	server_s: f64,
}

impl Times {
	pub fn new() -> Self {
		Times {
			server_s: 0.0,
		}
	}

	pub fn client_now_s(&self) -> f64 {
		precise_time_s()
	}

	pub fn update_server_s(&mut self, server_s: f64) {
		self.server_s = server_s;
	}

	pub fn server_last_known_s(&self) -> f64 {
		self.server_s
	}

	pub fn server_interpolated_s(&self) -> f64 {
		// TODO: Implement
		0.0
	}
}
