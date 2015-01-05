use super::{
	Message,
	Seq,
};


pub type Perception<Id, Entity> = Message<PerceptionHeader<Id>, Id, Entity>;


#[derive(Clone, Default, PartialEq, RustcDecodable, RustcEncodable, Show)]
pub struct PerceptionHeader<Id> {
	// TODO: Move field to Message
	pub confirm_action: Seq,
	pub self_id       : Option<Id>,
}
