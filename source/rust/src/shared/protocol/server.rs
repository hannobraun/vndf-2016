use shared::game::{
	Body,
	Broadcast,
	EntityId,
};


#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Event {
	Heartbeat,
	ShipId(EntityId),
	UpdateEntity(EntityId, (Body, Option<Broadcast>)),
	RemoveEntity(EntityId),
}
