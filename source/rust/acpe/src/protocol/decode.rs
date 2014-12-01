use std::io::BufReader;

use super::{
	MessagePart,
	Seq,
};


pub fn decode<P: MessagePart>(
	message: &[u8],
	parts  : &mut Vec<P>
) -> Result<Seq, String> {
	let mut reader = BufReader::new(message);

	let message = match reader.read_to_string() {
		Ok(message) =>
			message,
		Err(error) => {
			return Err(
				format!("Error converting message to string: {}\n", error)
			);
		},
	};

	let mut lines: Vec<&str> = message.split('\n').collect();

	let header = match lines.remove(0) {
		Some(header) =>
			header,
		None => {
			return Err(format!("Header line is missing\n"));
		},
	};

	let confirmed_seq = match from_str(header) {
		Some(confirmed_seq) =>
			confirmed_seq,
		None => {
			return Err(format!("Header is not a number\n"));
		},
	};

	for line in lines.into_iter() {
		if line.len() == 0 {
			continue;
		}

		match MessagePart::read(line) {
			Ok(part) =>
				parts.push(part),
			Err(error) =>
				return Err(error),

		}
	}

	Ok(confirmed_seq)
}
