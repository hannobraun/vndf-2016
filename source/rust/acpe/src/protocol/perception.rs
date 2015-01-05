use super::{
	Message,
	Seq,
};


pub type Perception<Id, Entity> = Message<PerceptionHeader<Id>, Id, Entity>;


#[derive(Clone, Default, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub struct PerceptionHeader<Id> {
	pub confirm_action: Seq,
	pub self_id       : Option<Id>,
}
