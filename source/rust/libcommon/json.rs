use serialize::{
	Encodable,
	Decodable
};
use serialize::json;
use serialize::json::{
	Encoder,
	Decoder,
	DecoderError
};
use std::io::{
	IoError,
	MemWriter
};
use std::str;


pub fn from_json<T: Decodable<Decoder, DecoderError>>(s: &str) -> Result<T, String> {
	let json_object = match json::from_str(s) {
		Ok(object) => object,
		Err(error) =>
			return Err(
				format!("Error decoding JSON object from \"{}\": {}", s, error))
	};

	let mut decoder = Decoder::new(json_object);

	match Decodable::decode(&mut decoder) {
		Ok(t) => Ok(t),

		Err(error) =>
			Err(format!("Error decoding JSON object ({}): {}", s, error))
	}
}

pub fn to_json<'a, T: Encodable<Encoder<'a>, IoError>>(object: T) -> String {
	let mut m = MemWriter::new();
		{
			let mut encoder = Encoder::new(&mut m as &mut Writer);
			match object.encode(&mut encoder) {
				Ok(()) => (),
				Err(e) => fail!("JSON encoding error: {}", e)
			};
		}

		str::from_utf8(m.get_ref())
			.expect("expected UTF-8 string")
			.to_owned()
}
