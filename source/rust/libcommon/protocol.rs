use rustecs::{
	components,
	Components,
	EntityId,
};

use ecs::components::Visual;
use json::{
	from_json,
	to_json
};
use physics::{
	Body,
	Radians
};


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Perception {
	pub self_id: EntityId,
	pub updated: Snapshot,
}

impl Perception {
	pub fn new(
		self_id: EntityId,
		_      : Option<Snapshot>,
		current: Snapshot) -> Perception {

		Perception {
			self_id: self_id,
			updated: current
		}
	}

	pub fn from_str(s: &str) -> Result<Perception, String> {
		from_json(s)
	}

	pub fn to_str(&self) -> String {
		to_json(self)
	}
}


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Snapshot {
	pub bodies  : Components<Body>,
	pub visuals : Components<Visual>,
}

impl Snapshot {
	pub fn new() -> Snapshot {
		Snapshot {
			bodies  : components(),
			visuals : components(),
		}
	}
}


#[deriving(Decodable, Encodable, PartialEq, Show)]
pub struct Action {
	pub attitude: Radians,
	pub missile : u64
}

impl Action {
	pub fn from_str(s: &str) -> Result<Action, String> {
		from_json(s)
	}

	pub fn to_str(&self) -> String {
		to_json(self)
	}
}
