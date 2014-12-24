use serialize::json::{
	mod,
	DecodeResult,
};

use common::protocol::Broadcast;


#[deriving(Clone, Decodable, Encodable, Eq, PartialEq, Show)]
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


#[deriving(Decodable, Encodable, Show)]
pub struct Frame {
	pub self_id   : String,
	pub status    : Status,
	pub broadcasts: Vec<Broadcast>,
}

impl Frame {
	pub fn from_json(json: &str) -> DecodeResult<Frame> {
		json::decode(json)
	}

	pub fn to_json(&self) -> String {
		json::encode(self)
	}
}


#[deriving(Decodable, Encodable, Eq, PartialEq, Show)]
pub enum Status {
	Notice(String),
	Error(String),
	None,
}

impl Status {
	pub fn is_notice(&self) -> bool {
		if let &Status::Notice(_) = self {
			true
		}
		else {
			false
		}
	}

	pub fn is_error(&self) -> bool {
		if let &Status::Error(_) = self {
			true
		}
		else {
			false
		}
	}

	pub fn is_none(&self) -> bool {
		if let &Status::None = self {
			true
		}
		else {
			false
		}
	}
}
