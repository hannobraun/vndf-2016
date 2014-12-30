use super::{
	Message,
	Seq,
};


pub type Action<Id, Entity> = Message<ActionHeader, Id, Entity>;


#[deriving(Clone, Copy, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub struct ActionHeader {
	pub id: Seq,
}
