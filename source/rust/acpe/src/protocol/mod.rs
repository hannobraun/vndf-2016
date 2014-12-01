use std::io::{
	BufReader,
	IoResult,
};


pub use self::encode::{
	Encoder,
	MessageEncoder,
};


mod encode;


pub type Seq = u64;


// TODO: A decode method in an encoder module. Something has to change.
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


pub trait MessagePart {
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()>;

	// TODO: This interface doesn't allow for an allocation-free implementation,
	//       when the type contains a String, Vec, or similar.
	fn read(line: &str) -> Result<Self, String>;
}
