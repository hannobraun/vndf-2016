use std::io::IoResult;

use super::{
	Part,
	Seq,
};


#[deriving(Clone, Copy, PartialEq, Show)]
pub struct ActionHeader {
	pub id: Seq,
}

impl Part for ActionHeader {
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		write!(writer, "{}\n", self.id)
	}

	fn read(line: &str) -> Result<ActionHeader, String> {
		match line.parse() {
			Some(id) => Ok(ActionHeader { id: id }),
			None     => Err(format!("Header is not a number")),
		}
	}
}
