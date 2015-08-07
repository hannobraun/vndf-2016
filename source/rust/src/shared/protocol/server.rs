use shared::game::{
	Body,
	Broadcast,
	EntityId,
};


#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Event {
	Heartbeat(f64),
	ShipId(EntityId),
	UpdateEntity(EntityId, (Body, Option<Broadcast>)),
	RemoveEntity(EntityId),
}
