use serialize::Decodable;
use serialize::json;
use serialize::json::Decoder;

pub fn from_json<T: Decodable<Decoder, json::Error>>(s: &str) -> T {
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
