use collections::HashMap;

pub type EntityId      = u32;
pub type Components<T> = HashMap<EntityId, T>;
