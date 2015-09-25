use shared::game::{EntityId};

use server::game::state::Entity;

#[derive(Clone, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Event {
    Heartbeat(f64),
    ShipId(EntityId),
    UpdateEntity(Entity),
    RemoveEntity(EntityId),
    Collision(EntityId,EntityId),
}
