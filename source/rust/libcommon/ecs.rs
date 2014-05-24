use collections::HashMap;

pub type EntityId      = uint;
pub type Components<T> = HashMap<EntityId, T>;
