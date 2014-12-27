use std::io::BufReader;

use super::{
	Message,
	Part,
};


pub fn decode<M: Message<H, P>, H: Part, P: Part>(
	message: &[u8],
	parts  : &mut Vec<P>
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

	let header = match Part::read(header) {
		Ok(header) => header,
		Err(error) => return Err(format!("Error decoding header: {}", error)),
	};

	for line in lines.into_iter() {
		if line.len() == 0 {
			continue;
		}

		match Part::read(line) {
			Ok(part) =>
				parts.push(part),
			Err(error) =>
				return Err(error),

		}
	}

	Ok(header)
}
