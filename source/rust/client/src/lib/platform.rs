use rustc_serialize::json::{
	self,
	DecodeResult,
};

use common::game::Broadcast;


#[derive(Clone, Debug, RustcDecodable, RustcEncodable, Eq, PartialEq)]
pub enum InputEvent {
	StartBroadcast(String),
	StopBroadcast,
}

impl InputEvent {
	pub fn from_json(json: &str) -> DecodeResult<InputEvent> {
		json::decode(json)
	}

	pub fn to_json(&self) -> String {
		match json::encode(self) {
			Ok(encoded) => encoded,
			Err(error)  => panic!("Encoding error: {}", error)
		}
	}
}


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Frame {
	pub self_id   : String,
	pub status    : Status,
	pub broadcasts: Vec<Broadcast>,
}

impl Frame {
	pub fn new() -> Frame {
		Frame {
			self_id   : String::new(),
			status    : Status::None,
			broadcasts: Vec::new(),
		}
	}

	pub fn from_json(json: &str) -> DecodeResult<Frame> {
		json::decode(json)
	}

	pub fn to_json(&self) -> String {
		match json::encode(self) {
			Ok(encoded) => encoded,
			Err(error)  => panic!("Encoding error: {}", error)
		}
	}
}


#[derive(Debug, RustcDecodable, RustcEncodable, Eq, PartialEq)]
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
