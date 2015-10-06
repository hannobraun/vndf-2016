use shared::game::{
	Body,
	Broadcast,
	EntityId,
	Maneuver,
	Planet,
	Ship,
};


#[derive(Clone, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Entity {
	pub id: EntityId,

	pub body      : Option<Body>,
	pub broadcast : Option<Broadcast>,
	pub maneuver  : Option<Maneuver>,
	pub planet    : Option<Planet>,
	pub ship      : Option<Ship>,
}


#[derive(Clone, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Event {
    Heartbeat(f64),
    ShipId(EntityId),
    UpdateEntity(Entity),
    RemoveEntity(EntityId),
}
