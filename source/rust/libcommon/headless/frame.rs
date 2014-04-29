use serialize::{
	Decodable,
	Encodable
};
use serialize::json;
use serialize::json::{
	Decoder,
	Encoder
};
use std::io::MemWriter;
use std::str;

use physics::Body;


#[deriving(Decodable, Encodable)]
pub struct Frame {
	pub ships: ~[Body]
}

impl Frame {
	pub fn from_json(s: &str) -> Frame {
		let json_object = match json::from_str(s) {
			Ok(object) => object,
			Err(error) =>
				fail!("Error decoding JSON object from \"{}\": {}", s, error)
		};

		let mut decoder = Decoder::new(json_object);

		match Decodable::decode(&mut decoder) {
			Ok(frame)  => frame,
			Err(error) => fail!("error decoding JSON object ({})", error)
		}
	}

	pub fn to_json(&self) -> ~str {
		let mut m = MemWriter::new();
		{
			let mut encoder = Encoder::new(&mut m as &mut Writer);
			match self.encode(&mut encoder) {
				Ok(()) => (),
				Err(e) => fail!("JSON encoding error: {}", e)
			};
		}

		str::from_utf8(m.get_ref())
			.expect("expected UTF-8 string")
			.to_owned()
	}
}
