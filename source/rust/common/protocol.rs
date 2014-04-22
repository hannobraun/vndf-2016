use serialize::{Decodable, Encodable};
use serialize::json;
use serialize::json::Encoder;
use std::intrinsics::TypeId;
use std::io::MemWriter;
use std::io::Writer;
use std::str;

use physics::{Body, Radians};


#[deriving(Decodable, Encodable, Show)]
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
		let json_object = match json::from_str(s) {
			Ok(object) => object,
			Err(error) =>
				fail!("Error decoding JSON object from \"{}\": {}", s, error)
		};

		let mut decoder = json::Decoder::new(json_object);

		match Decodable::decode(&mut decoder) {
			Ok(message) => message,
			Err(error)  => fail!("error decoding JSON object ({})", error)
		}
	}

	pub fn to_str(&self) -> ~str {
		let mut m = MemWriter::new();
		{
			let mut encoder = Encoder::new(&mut m as &mut Writer);
			match self.encode(&mut encoder) {
				Ok(()) => (),
				Err(e) => fail!("json encoding error: {}", e)
			};
		}

		str::from_utf8(m.get_ref())
			.expect("expected UTF-8 string")
			.to_owned()
	}

	pub fn type_id(&self) -> TypeId {
		match *self {
			SelfInfo(_) => TypeId::of::<SelfInfo>(),
			Create(_)   => TypeId::of::<Create>(),
			Update(_)   => TypeId::of::<Update>(),
			Remove(_)   => TypeId::of::<Remove>(),
			Command(_)  => TypeId::of::<Command>(),
			Invalid(_)  => TypeId::of::<()>()
		}
	}
}


#[deriving(Decodable, Encodable, Show)]
pub struct SelfInfo {
	pub id: uint
}

#[deriving(Decodable, Encodable, Show)]
pub struct Create {
	pub id  : uint,
	pub kind: ~str
}

#[deriving(Decodable, Encodable, Eq, Show)]
pub struct Update {
	pub id  : uint,
	pub body: Body
}

#[deriving(Decodable, Encodable, Show)]
pub struct Remove {
	pub id: uint
}

#[deriving(Decodable, Encodable, Show)]
pub struct Command {
	pub attitude: Radians
}
