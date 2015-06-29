use nalgebra::Vec2;
use rustc_serialize::json::{
	self,
	DecodeResult,
};

use shared::game::Broadcast;


#[derive(Clone, Debug, RustcDecodable, RustcEncodable, PartialEq)]
pub enum InputEvent {
	StartBroadcast(String),
	StopBroadcast,

	ScheduleManeuver(f32),

	Quit,
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


#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Frame {
	pub ship_id   : Option<String>,
	pub message   : Message,
	pub broadcasts: Vec<Broadcast>,
	pub position  : Vec2<f32>,
	pub velocity  : Vec2<f32>,
}

impl Frame {
	pub fn new() -> Frame {
		Frame {
			ship_id   : None,
			message   : Message::None,
			broadcasts: Vec::new(),
			position  : Vec2::new(0.0, 0.0),
			velocity  : Vec2::new(0.0, 0.0),
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


#[derive(Clone, Debug, RustcDecodable, RustcEncodable, Eq, PartialEq)]
pub enum Message {
	Notice(String),
	Error(String),
	None,
}

impl Message {
	pub fn is_notice(&self) -> bool {
		if let &Message::Notice(_) = self {
			true
		}
		else {
			false
		}
	}

	pub fn is_error(&self) -> bool {
		if let &Message::Error(_) = self {
			true
		}
		else {
			false
		}
	}

	pub fn is_none(&self) -> bool {
		if let &Message::None = self {
			true
		}
		else {
			false
		}
	}
}
