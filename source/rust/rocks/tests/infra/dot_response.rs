pub struct DotResponse {
	code    : u16,
	location: String,
}

impl DotResponse {
	pub fn new(code: u16, location: &str) -> DotResponse {
		DotResponse {
			code    : code,
			location: location.to_string(),
		}
	}

	pub fn build(&self) -> String {
		format!(
			"
				code     = {}
				location = \"{}\"
			",
			self.code,
			self.location,
		)
	}
}
