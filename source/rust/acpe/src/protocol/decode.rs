use std::io::BufReader;

use rustc_serialize::{
	json,
	Decodable,
};

use constants::{
	DESTROY,
	UPDATE,
};


pub fn decode<Header, Id, Entity>(
	source : &[u8],
	header : &mut Header,
	update : &mut Vec<(Id, Entity)>,
	destroy: &mut Vec<Id>,
) -> Result<(), String>
	where
		Header: Decode,
		Id    : Decode,
		Entity: Decode,
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

	let header_line = match lines.next() {
		Some(header_line) =>
			header_line,
		None => {
			return Err(format!("Header line is missing"));
		},
	};

	*header = match Decode::do_decode(header_line) {
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

		let item = match splits.next() {
			Some(item) =>
				item,
			None =>
				return Err(format!("Invalid message line: Missing entity")),
		};

		match directive {
			UPDATE => {
				// TODO(85374308): This assumes the id contains no spaces.
				let mut splits = item.splitn(1, ' ');

				let id = match splits.next() {
					Some(id) => match Decode::do_decode(id) {
						Ok(id) =>
							id,
						Err(error) =>
							return Err(
								format!(
									"Error decoding id: {}; Id: {}",
									error, id,
								)
							),
					},
					None =>
						return Err(format!("Invalid update: No id")),
				};

				let entity = match splits.next() {
					Some(entity) => match Decode::do_decode(entity) {
						Ok(entity) =>
							entity,
						Err(error) =>
							return Err(
								format!("Error decoding entity: {}", error)
							),
					},
					None =>
						return Err(format!("Invalid update: No entity")),
				};

				update.push((id, entity));
			},

			DESTROY => {
				match Decode::do_decode(item) {
					Ok(id) =>
						destroy.push(id),
					Err(error) =>
						return Err(format!("Error decoding id: {}", error)),
				}
			},

			_ => return Err(format!("Unknown directive: {}", directive)),
		}
	}

	Ok(())
}


pub trait Decode {
	fn do_decode(s: &str) -> Result<Self, String>;
}

impl<T> Decode for T where T: Decodable {
	fn do_decode(s: &str) -> Result<Self, String> {
		json::decode(s)
			.map_err(|error|
				format!("JSON decoding error: {:?}", error)
			)
	}
}
