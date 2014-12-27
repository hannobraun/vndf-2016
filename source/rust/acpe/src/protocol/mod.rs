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
pub use self::perception::{
	Perception,
	PerceptionHeader,
};


mod action;
mod decode;
mod encode;
mod perception;


pub type Seq = u64;


pub trait Message<H: Header, B: Part> {}


pub trait Part {
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()>;
	fn read(line: &str) -> Result<Self, String>;
}


pub trait Header {
	fn write<W: Writer>(&self, writer: &mut W) -> IoResult<()>;
	fn read(line: &str) -> Result<Self, String>;
}
