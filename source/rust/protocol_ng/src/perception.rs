use serialize::json::{
	mod,
	DecodeResult,
};
use std::io::{
	BufWriter,
	IoResult,
};


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Perception {
	pub last_action: u64,
	pub broadcasts : Vec<String>,
}

impl Perception {
	pub fn decode(json: &str) -> DecodeResult<Perception> {
		json::decode(json)
	}
}


pub struct PerceptionEnc {
	last_action: u64,
	broadcasts : Vec<String>,
}

impl PerceptionEnc {
	pub fn new(last_action: u64) -> PerceptionEnc {
		PerceptionEnc {
			last_action: last_action,
			broadcasts : Vec::new(),
		}
	}

	pub fn update(&mut self, broadcast: &str) -> bool {
		self.broadcasts.push(broadcast.to_string());
		true
	}

	pub fn encode(self, buffer: &mut [u8]) -> IoResult<&[u8]> {
		let len = {
			let perception = Perception {
				last_action: self.last_action,
				broadcasts : self.broadcasts,
			};
			let string = json::encode(&perception);
			let bytes  = string.as_bytes();

			let mut writer = BufWriter::new(buffer);

			match writer.write(bytes) {
				Ok(())     => bytes.len(),
				Err(error) => return Err(error),
			}
		};

		Ok(buffer[.. len])
	}
}
