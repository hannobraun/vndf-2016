#![crate_name = "rustecs"]
#![crate_type = "rlib"]


use std::collections::HashMap;


pub type EntityId      = u32;
pub type Components<T> = HashMap<EntityId, T>;


pub trait Entity {
	fn id(&self) -> EntityId;
}


pub fn components<T>() -> Components<T> {
	HashMap::new()
}
