use collections::HashMap;

use ecs::EntityId;
use json::{
	from_json,
	to_json
};
use physics::{
	Body,
	Radians
};


#[deriving(Clone, Decodable, Encodable, Eq, Show)]
pub struct Perception {
	pub self_id : EntityId,
	pub ships   : HashMap<EntityId, Body>,
	pub missiles: HashMap<EntityId, Body>
}

impl Perception {
	pub fn from_str(s: &str) -> Result<Perception, ~str> {
		from_json(s)
	}

	pub fn to_str(&self) -> ~str {
		to_json(self)
	}
}


#[deriving(Decodable, Encodable, Eq, Show)]
pub struct Action {
	pub attitude: Radians,
	pub missile : u64
}

impl Action {
	pub fn from_str(s: &str) -> Result<Action, ~str> {
		from_json(s)
	}

	pub fn to_str(&self) -> ~str {
		to_json(self)
	}
}
