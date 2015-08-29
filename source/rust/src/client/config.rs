pub struct Config {
	pub scaling_factor: f32,
}

impl Config {
	pub fn load() -> Config {
		Config {
			scaling_factor: 2.0,
		}
	}
}
