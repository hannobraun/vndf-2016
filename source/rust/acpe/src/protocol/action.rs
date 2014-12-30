use super::{
	Message,
	Seq,
};


pub type Action<T> = Message<ActionHeader, T>;


#[deriving(Clone, Copy, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub struct ActionHeader {
	pub id: Seq,
}
