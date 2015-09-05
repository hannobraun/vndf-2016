pub struct Console {
	pub output: Vec<String>,
	pub input : String,
}

impl Console {
	pub fn new(output: Vec<String>) -> Console {
		Console {
			output: output,
			input : String::new(),
		}
	}
}
