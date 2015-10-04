use shared::game::{
	Body,
	Broadcast,
	EntityId,
	ManeuverData,
	Planet,
	Ship,
};


#[derive(Clone, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Entity {
	pub id: EntityId,

	pub body      : Option<Body>,
	pub broadcast : Option<Broadcast>,
	pub maneuver  : Option<ManeuverData>,
	pub planet    : Option<Planet>,
	pub ship      : Option<Ship>,
}


#[derive(Clone, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Event {
    Heartbeat(f64),
    ShipId(EntityId),
    UpdateEntity(Entity),
    RemoveEntity(EntityId),
    Collision(EntityId,EntityId),
}
