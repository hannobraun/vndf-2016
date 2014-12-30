use super::{
	Message,
	Seq,
};


pub type Perception<T> = Message<PerceptionHeader, T>;


#[deriving(Clone, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub struct PerceptionHeader {
	pub confirm_action: Seq,
	pub self_id       : Option<String>,
}
