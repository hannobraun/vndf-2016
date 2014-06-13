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

use json::{
	from_json,
	to_json
};
use physics::Radians;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Perception<Id, T> {
	pub self_id: Id,
	pub updated: Vec<T>,
}

impl<
	'a,
	Id:
		Decodable<Decoder, DecoderError> +
		Encodable<Encoder<'a>, IoError>,
	T:
		Decodable<Decoder, DecoderError> +
		Encodable<Encoder<'a>, IoError>
> Perception<Id, T> {
	pub fn new(
		_      : |T| -> Id,
		self_id: Id,
		_      : Vec<T>,
		current: Vec<T>
	) -> Perception<Id, T> {
		Perception {
			self_id: self_id,
			updated: current
		}
	}

	pub fn from_str(s: &str) -> Result<Perception<Id, T>, String> {
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
