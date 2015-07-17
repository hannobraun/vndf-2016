use std::collections::HashMap;

use game::Maneuver;
use shared::game::{
	Body,
	Broadcast,
	EntityId,
};


pub type Components<T> = HashMap<EntityId, T>;


#[derive(Debug)]
pub struct Entities {
	next_id: u64,

	pub bodies    : Components<Body>,
	pub broadcasts: Components<Broadcast>,
	pub maneuvers : Components<Maneuver>,
}

impl Entities {
	pub fn new() -> Entities {
		Entities {
			next_id   : 0,
			bodies    : HashMap::new(),
			broadcasts: HashMap::new(),
			maneuvers : HashMap::new(),
		}
	}

	pub fn create_entity(&mut self) -> EntityBuilder {
		let id = self.next_id;
		self.next_id += 1;

		EntityBuilder {
			id: id,

			bodies    : &mut self.bodies,
			broadcasts: &mut self.broadcasts,
			maneuvers : &mut self.maneuvers,
		}
	}

	pub fn update_entity(&mut self, id: EntityId) -> EntityUpdater {
		EntityUpdater {
			id: id,

			broadcasts: &mut self.broadcasts,
			maneuvers : &mut self.maneuvers,
			ships     : &mut self.bodies,
		}
	}

	pub fn destroy_entity(&mut self, id: &EntityId) {
		self.bodies.remove(id);
		self.broadcasts.remove(id);
		self.maneuvers.remove(id);
	}
}


// TODO: Remove #[allow(unused)], once this is moved to a library.
#[allow(unused)]
pub struct EntityBuilder<'c> {
	id: EntityId,

	bodies    : &'c mut Components<Body>,
	broadcasts: &'c mut Components<Broadcast>,
	maneuvers : &'c mut Components<Maneuver>,
}

// TODO: Remove #[allow(unused)], once this is moved to a library.
#[allow(unused)]
impl<'c> EntityBuilder<'c> {
	pub fn with_broadcast(mut self, component: Broadcast) -> EntityBuilder<'c> {
		self.broadcasts.insert(self.id, component);
		self
	}

	pub fn with_maneuver(mut self, component: Maneuver) -> EntityBuilder<'c> {
		self.maneuvers.insert(self.id, component);
		self
	}

	pub fn with_ship(mut self, component: Body) -> EntityBuilder<'c> {
		self.bodies.insert(self.id, component);
		self
	}

	pub fn return_id(self) -> EntityId {
		self.id
	}
}


// TODO: Remove #[allow(unused)], once this is moved to a library.
#[allow(unused)]
pub struct EntityUpdater<'c> {
	id: EntityId,

	broadcasts: &'c mut Components<Broadcast>,
	maneuvers : &'c mut Components<Maneuver>,
	ships     : &'c mut Components<Body>,
}

// TODO: Remove #[allow(unused)], once this is moved to a library.
#[allow(unused)]
impl<'c> EntityUpdater<'c> {
	pub fn add_broadcast(mut self, component: Broadcast) -> EntityUpdater<'c> {
		self.broadcasts.insert(self.id, component);
		self
	}

	pub fn add_maneuver(mut self, component: Maneuver) -> EntityUpdater<'c> {
		self.maneuvers.insert(self.id, component);
		self
	}

	pub fn add_ship(mut self, component: Body) -> EntityUpdater<'c> {
		self.ships.insert(self.id, component);
		self
	}

	pub fn remove_broadcast(mut self) -> EntityUpdater<'c> {
		self.broadcasts.remove(&self.id);
		self
	}

	pub fn remove_maneuver(mut self) -> EntityUpdater<'c> {
		self.maneuvers.remove(&self.id);
		self
	}

	pub fn remove_ship(mut self) -> EntityUpdater<'c> {
		self.ships.remove(&self.id);
		self
	}
}
