use time::precise_time_s;


pub struct Times;

impl Times {
	pub fn new() -> Self {
		Times
	}

	pub fn client_now_s(&self) -> f64 {
		precise_time_s()
	}
}
