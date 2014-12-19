#[deriving(Clone, Eq, PartialEq)]
pub struct Input {
	pub broadcast: Option<String>,
	pub command  : (String, Vec<String>),
	pub error    : Option<(String, String)>,
}

impl Input {
	pub fn new() -> Input {
		Input {
			broadcast: None,
			command  : (String::new(), Vec::new()),
			error    : None,
		}
	}
}
