use collections::HashMap;

use common::ecs::EntityId;


pub type Components<T> = HashMap<EntityId, T>;


pub struct Player {
	pub missile_index: u64
}
