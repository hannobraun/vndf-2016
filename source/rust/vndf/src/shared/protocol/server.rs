use shared::game::{
	Attributes,
	Body,
	Broadcast,
	EntityId,
	Ship,
};


pub type Entity = (
	EntityId,
	(
		Body,
		Option<Broadcast>,
		Option<Attributes>,
		Option<Ship>,
	)
);


#[derive(Clone, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Event {
    Heartbeat(f64),
    ShipId(EntityId),
    UpdateEntity(Entity),
    RemoveEntity(EntityId),
    Collision(EntityId,EntityId),
}
