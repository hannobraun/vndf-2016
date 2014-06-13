use serialize::{
	Decodable,
	Encodable,
};
use serialize::json::{
	Decoder,
	DecoderError,
	Encoder,
};
use std::io::IoError;

use rustecs::EntityId;

use json::{
	from_json,
	to_json
};
use physics::Radians;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Perception<T> {
	pub self_id: EntityId,
	pub updated: Vec<T>,
}

impl<'a, T:
	Decodable<Decoder, DecoderError> +
	Encodable<Encoder<'a>, IoError>
> Perception<T> {
	pub fn new(
		self_id: EntityId,
		_      : Option<Vec<T>>,
		current: Vec<T>
	) -> Perception<T> {
		Perception {
			self_id: self_id,
			updated: current
		}
	}

	pub fn from_str(s: &str) -> Result<Perception<T>, String> {
		from_json(s)
	}

	pub fn to_str(&self) -> String {
		to_json(self)
	}
}


#[deriving(Decodable, Encodable, PartialEq, Show)]
pub struct Action {
	pub attitude: Radians,
	pub missile : u64
}

impl Action {
	pub fn from_str(s: &str) -> Result<Action, String> {
		from_json(s)
	}

	pub fn to_str(&self) -> String {
		to_json(self)
	}
}
