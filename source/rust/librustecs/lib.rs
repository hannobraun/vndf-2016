#![crate_id   = "rustecs"]
#![crate_type = "rlib"]


use std::collections::HashMap;


pub type EntityId      = u32;
pub type Components<T> = HashMap<EntityId, T>;


pub trait Entity {
	fn id(&self) -> EntityId;
}

pub trait World<E: Entity> {
	fn new() -> Self;

	fn from_entities(entities: Vec<E>) -> Self;
	fn to_entities(&self) -> Vec<E>;

	fn import_entity(&mut self, entity: E) -> bool;
	fn destroy_entity(&mut self, id: EntityId) -> bool;
}


pub fn components<T>() -> Components<T> {
	HashMap::new()
}
