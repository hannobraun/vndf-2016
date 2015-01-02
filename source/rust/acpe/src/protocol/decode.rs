use std::io::BufReader;

use rustc_serialize::{
	json,
	Decodable,
};

use root::UPDATE;

use super::Message;


pub fn decode<H, I, E>(
	source: &[u8],
	target: &mut Message<H, I, E>
) -> Result<(), String>
	where
		H: Decode,
		E: Decode,
{
	let mut reader = BufReader::new(source);

	let message = match reader.read_to_string() {
		Ok(message) =>
			message,
		Err(error) => {
			return Err(
				format!("Error converting message to string: {}", error)
			);
		},
	};

	let mut lines = message.split('\n');

	let header = match lines.next() {
		Some(header) =>
			header,
		None => {
			return Err(format!("Header line is missing"));
		},
	};

	target.header = match Decode::do_decode(header) {
		Ok(header) => header,
		Err(error) => return Err(format!("Error decoding header: {}", error)),
	};

	for line in lines {
		if line.len() == 0 {
			continue;
		}

		let mut splits = line.splitn(1, ' ');

		let directive = match splits.next() {
			Some(directive) =>
				directive,
			None =>
				return Err(format!("Invalid message line: Missing directive")),
		};

		let entity = match splits.next() {
			Some(entity) =>
				entity,
			None =>
				return Err(format!("Invalid message line: Missing entity")),
		};

		match directive {
			UPDATE => match Decode::do_decode(entity) {
				Ok(entity) =>
					target.update.push(entity),
				Err(error) =>
					return Err(format!("Error decoding entity: {}", error)),
			},

			_ => return Err(format!("Unknown directive: {}", directive)),
		}
	}

	Ok(())
}


pub trait Decode {
	fn do_decode(s: &str) -> Result<Self, String>;
}

impl<T> Decode for T where T: Decodable<json::Decoder, json::DecoderError> {
	fn do_decode(s: &str) -> Result<Self, String> {
		json::decode(s)
			.map_err(|error|
				format!("JSON decoding error: {}", error)
			)
	}
}
