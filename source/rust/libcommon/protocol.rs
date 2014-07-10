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
	pub added  : Vec<T>,
	pub removed: Vec<T>,
	pub updated: Vec<T>,
}

impl<
	'a,
	Id:
		Ord +
		Decodable<Decoder, DecoderError> +
		Encodable<Encoder<'a>, IoError>,
	T:
		Decodable<Decoder, DecoderError> +
		Encodable<Encoder<'a>, IoError>
> Perception<Id, T> {
	pub fn new(
		get_id      : |&T| -> Id,
		self_id     : Id,
		mut previous: Vec<T>,
		mut current : Vec<T>
	) -> Perception<Id, T> {
		previous.sort_by(|a, b| get_id(a).cmp(&get_id(b)));
		current.sort_by(|a, b| get_id(a).cmp(&get_id(b)));

		let mut previous = previous.move_iter();
		let mut current  = current.move_iter();

		let mut added   = Vec::new();
		let mut removed = Vec::new();
		let mut updated = Vec::new();

		let mut p = previous.next();
		let mut c = current.next();

		while !(p.is_none() && c.is_none()) {
			if p.is_none() {
				added.push(c.take_unwrap());

				c = current.next();

				continue;
			}
			if c.is_none() {
				removed.push(p.take_unwrap());

				p = current.next();

				continue;
			}

			let p_id = get_id(p.get_ref());
			let c_id = get_id(c.get_ref());

			if p_id == c_id {
				updated.push(c.take_unwrap());

				p = previous.next();
				c = current.next();

				continue;
			}
			if p_id < c_id {
				removed.push(p.take_unwrap());

				p = previous.next();

				continue;
			}
			if p_id > c_id {
				added.push(c.take_unwrap());

				c = current.next();

				continue;
			}
		}

		Perception {
			self_id: self_id,
			added  : added,
			removed: removed,
			updated: updated,
		}
	}

	pub fn from_string(s: &str) -> Result<Perception<Id, T>, String> {
		from_json(s)
	}

	pub fn to_string(&self) -> String {
		to_json(self)
	}
}


#[deriving(Decodable, Encodable, PartialEq, Show)]
pub struct Action {
	pub attitude: Radians,
	pub missile : u64
}

impl Action {
	pub fn from_string(s: &str) -> Result<Action, String> {
		from_json(s)
	}

	pub fn to_str(&self) -> String {
		to_json(self)
	}
}
