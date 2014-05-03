use json::{
	from_json,
	to_json
};
use physics::{
	Body,
	Radians
};


#[deriving(Decodable, Encodable, Eq, Show)]
pub enum Message {
	Perception(Perception),
	Command(Command),
	Invalid(~str)
}

impl Message {
	pub fn from_str(s: &str) -> Result<Message, ~str> {
		from_json(s)
	}

	pub fn to_str(&self) -> ~str {
		to_json(self)
	}
}

#[deriving(Clone, Decodable, Encodable, Eq, Show)]
pub struct Perception {
	pub self_id: uint,
	pub ships  : ~[Ship]
}

impl Perception {
	pub fn from_str(s: &str) -> Result<Perception, ~str> {
		from_json(s)
	}

	pub fn to_str(&self) -> ~str {
		to_json(self)
	}
}


#[deriving(Clone, Decodable, Encodable, Eq, Show)]
pub struct Ship {
	pub id  : uint,
	pub body: Body
}

#[deriving(Decodable, Encodable, Eq, Show)]
pub struct Command {
	pub attitude: Radians
}
