use serialize::Encodable;
use serialize::json::Encoder;
use std::io::MemWriter;
use std::str;

use physics::Body;


#[deriving(Encodable)]
pub struct Frame {
	pub ships: ~[Body]
}

impl Frame {
	pub fn to_json(&self) -> ~str {
		let mut m = MemWriter::new();
		{
			let mut encoder = Encoder::new(&mut m as &mut Writer);
			match self.encode(&mut encoder) {
				Ok(()) => (),
				Err(e) => fail!("JSON encoding error: {}", e)
			};
		}

		str::from_utf8(m.get_ref())
			.expect("expected UTF-8 string")
			.to_owned()
	}
}
