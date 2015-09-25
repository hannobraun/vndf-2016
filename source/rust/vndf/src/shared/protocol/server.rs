use shared::game::{
	Attributes,
	Body,
	Broadcast,
	EntityId,
	Planet,
	Ship,
};


#[derive(Clone, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Entity {
	pub id: EntityId,

	pub body: Body,

	pub attributes: Option<Attributes>,
	pub broadcast : Option<Broadcast>,
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
