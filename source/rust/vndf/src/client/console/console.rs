pub struct Console {
	pub output: Vec<String>,
	pub input : String,

	pub prompt_index: usize,
}

impl Console {
	pub fn new(output: Vec<String>) -> Console {
		Console {
			output      : output,
			input       : String::new(),
			prompt_index: 0,
		}
	}
}
