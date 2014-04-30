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
	SelfInfo(SelfInfo),
	Create(Create),
	Update(Update),
	Remove(Remove),
	Command(Command),
	Invalid(~str)
}

impl Message {
	pub fn from_str(s: &str) -> Message {
		from_json(s)
	}

	pub fn to_str(&self) -> ~str {
		to_json(self)
	}
}


#[deriving(Decodable, Encodable, Eq, Show)]
pub struct SelfInfo {
	pub id: uint
}

#[deriving(Decodable, Encodable, Eq, Show)]
pub struct Create {
	pub id  : uint,
	pub kind: ~str
}

#[deriving(Decodable, Encodable, Eq, Show)]
pub struct Update {
	pub ships: ~[Ship]
}

#[deriving(Clone, Decodable, Encodable, Eq, Show)]
pub struct Ship {
	pub id  : uint,
	pub body: Body
}

#[deriving(Decodable, Encodable, Eq, Show)]
pub struct Remove {
	pub id: uint
}

#[deriving(Decodable, Encodable, Eq, Show)]
pub struct Command {
	pub attitude: Radians
}
