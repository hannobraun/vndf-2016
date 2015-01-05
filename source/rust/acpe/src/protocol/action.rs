use super::{
	Message,
	Seq,
};


pub type Action<Id, Entity> = Message<ActionHeader, Id, Entity>;


#[derive(Clone, Copy, Default, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub struct ActionHeader {
	pub id: Seq,
}
