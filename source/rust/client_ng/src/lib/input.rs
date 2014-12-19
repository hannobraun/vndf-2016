use serialize::json::{
	mod,
	DecodeResult,
};


#[deriving(Clone, Decodable, Encodable, Eq, PartialEq)]
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

	pub fn from_json(json: &str) -> DecodeResult<Input> {
		json::decode(json)
	}

	pub fn to_json(&self) -> String {
		json::encode(self)
	}
}
