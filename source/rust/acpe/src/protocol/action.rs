use std::io::IoResult;

use super::{
	Message,
	Part,
	Seq,
};


pub type Action<T> = Message<ActionHeader, T>;


#[deriving(Clone, Copy, PartialEq, Show)]
pub struct ActionHeader {
	pub id: Seq,
}

impl Part for ActionHeader {
	fn assemble<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		write!(writer, "{}\n", self.id)
	}

	fn disassemble(line: &str) -> Result<ActionHeader, String> {
		match line.parse() {
			Some(id) => Ok(ActionHeader { id: id }),
			None     => Err(format!("Header is not a number")),
		}
	}
}
