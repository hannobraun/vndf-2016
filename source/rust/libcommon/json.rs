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
use std::io::IoError;


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
	json::encode(&object)
}
