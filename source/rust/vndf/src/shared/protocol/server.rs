use shared::game::{
	Body,
	Broadcast,
	EntityId,
};

use server::game::state::EntityState;

#[derive(Clone, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Event {
	Heartbeat(f64),
	ShipId(EntityId),
	UpdateEntity(EntityState),
	RemoveEntity(EntityId),
}
