pub struct Console {
	pub output: Vec<String>,
}

impl Console {
	pub fn new(output: Vec<String>) -> Console {
		Console {
			output: output,
		}
	}
}
