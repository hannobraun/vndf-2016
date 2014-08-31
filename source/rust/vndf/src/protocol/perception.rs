use serialize::{
	Decodable,
	Encodable,
};
use serialize::json;
use serialize::json::{
	Decoder,
	DecoderError,
	DecodeResult,
	Encoder,
};
use std::io::IoError;


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
				added.push(c.take().unwrap());

				c = current.next();

				continue;
			}
			if c.is_none() {
				removed.push(p.take().unwrap());

				p = current.next();

				continue;
			}

			let p_id = get_id(p.as_ref().unwrap());
			let c_id = get_id(c.as_ref().unwrap());

			if p_id == c_id {
				updated.push(c.take().unwrap());

				p = previous.next();
				c = current.next();

				continue;
			}
			if p_id < c_id {
				removed.push(p.take().unwrap());

				p = previous.next();

				continue;
			}
			if p_id > c_id {
				added.push(c.take().unwrap());

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

	pub fn from_string(s: &str) -> DecodeResult<Perception<Id, T>> {
		json::decode(s)
	}

	pub fn to_string(&self) -> String {
		json::encode(self)
	}
}
