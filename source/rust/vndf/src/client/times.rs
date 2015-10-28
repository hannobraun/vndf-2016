use time::precise_time_s;


pub struct Times {
	server_s                      : f64,
	client_at_last_server_update_s: f64,
}

impl Times {
	pub fn new() -> Self {
		Times {
			server_s                      : 0.0,
			client_at_last_server_update_s: precise_time_s(),
		}
	}

	pub fn client_now_s(&self) -> f64 {
		precise_time_s()
	}

	pub fn update_server_s(&mut self, server_s: f64) {
		self.server_s                       = server_s;
		self.client_at_last_server_update_s = self.client_now_s();
	}

	pub fn server_last_known_s(&self) -> f64 {
		self.server_s
	}

	pub fn server_interpolated_s(&self) -> f64 {
		// This should give a good enough approximation, as long as the latency
		// is somewhat constant, which of course it isn't in reality. I think it
		// will be ok, as this isn't the kind of game that needs highly accurate
		// synchronization between client and server. Accuracy will go down
		// around latency spikes, but it should be fine.
		self.server_s
			+ self.client_now_s()
			- self.client_at_last_server_update_s
	}
}
