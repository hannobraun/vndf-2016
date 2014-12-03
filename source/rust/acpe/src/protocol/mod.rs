use std::io::IoResult;


pub use self::action::{
	Action,
	ActionHeader,
};
pub use self::decode::decode;
pub use self::encode::{
	Encoder,
	MessageEncoder,
};
pub use self::perception::Perception;


mod action;
mod decode;
mod encode;
mod perception;


pub type Seq = u64;


pub trait Message<H: Header, P: Part> {}


pub trait Part {
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()>;
	fn read(line: &str) -> Result<Self, String>;
}


pub trait Header {
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()>;
	fn read(line: &str) -> Result<Self, String>;
}

impl Header for Seq {
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()> {
		write!(writer, "{}\n", self)
	}

	fn read(line: &str) -> Result<Seq, String> {
		match from_str(line) {
			Some(seq) => Ok(seq),
			None      => Err(format!("Header is not a number\n")),
		}
	}
}
