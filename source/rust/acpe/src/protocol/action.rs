use std::io::IoResult;

use rustc_serialize::{
	json,
	Encodable,
};

use super::{
	Message,
	Part,
	Seq,
};


pub type Action<T> = Message<ActionHeader, T>;


#[deriving(Clone, Copy, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub struct ActionHeader {
	pub id: Seq,
}

impl Part for ActionHeader {
	fn assemble<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		try!(self.encode(&mut json::Encoder::new(writer)));
		try!(writer.write_char('\n'));

		Ok(())
	}

	fn disassemble(line: &str) -> Result<ActionHeader, String> {
		json::decode(line)
			.map_err(|error|
				format!("Error decoding action header: {}", error)
			)
	}
}
