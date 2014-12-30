use std::io::BufReader;

use root::UPDATE;

use super::Part;


pub fn decode<H: Part, P: Part>(
	message: &[u8],
	update : &mut Vec<P>,
) -> Result<H, String> {
	let mut reader = BufReader::new(message);

	let message = match reader.read_to_string() {
		Ok(message) =>
			message,
		Err(error) => {
			return Err(
				format!("Error converting message to string: {}", error)
			);
		},
	};

	let mut lines: Vec<&str> = message.split('\n').collect();

	let header = match lines.remove(0) {
		Some(header) =>
			header,
		None => {
			return Err(format!("Header line is missing"));
		},
	};

	let header = match Part::disassemble(header) {
		Ok(header) => header,
		Err(error) => return Err(format!("Error decoding header: {}", error)),
	};

	for line in lines.into_iter() {
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
			UPDATE => match Part::disassemble(entity) {
				Ok(entity) => update.push(entity),
				Err(error) => return Err(error),
			},

			_ => return Err(format!("Unknown directive: {}", directive)),
		}
	}

	Ok(header)
}
