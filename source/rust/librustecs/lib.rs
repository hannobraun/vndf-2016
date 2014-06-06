#![crate_id   = "rustecs"]
#![crate_type = "rlib"]


extern crate collections;


use collections::HashMap;


pub type EntityId      = u32;
pub type Components<T> = HashMap<EntityId, T>;


pub fn components<T>() -> Components<T> {
	HashMap::new()
}
