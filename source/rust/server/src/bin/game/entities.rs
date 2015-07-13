use std::collections::HashMap;

use game::Maneuver;
use shared::game::{
	Broadcast,
	EntityId,
	Ship,
};


pub type Components<T> = HashMap<EntityId, T>;


#[derive(Debug)]
pub struct Entities {
	next_id: u64,

	pub broadcasts: Components<Broadcast>,
	pub maneuvers : Components<Maneuver>,
	pub ships     : Components<Ship>,
}

impl Entities {
	pub fn new() -> Entities {
		Entities {
			next_id   : 0,
			broadcasts: HashMap::new(),
			maneuvers : HashMap::new(),
			ships     : HashMap::new(),
		}
	}

	pub fn create_entity(&mut self) -> EntityBuilder {
		let id = self.next_id;
		self.next_id += 1;

		EntityBuilder {
			id: id,

			broadcasts: &mut self.broadcasts,
			maneuvers : &mut self.maneuvers,
			ships     : &mut self.ships,
		}
	}

	pub fn update_entity(&mut self, id: EntityId) -> EntityUpdater {
		EntityUpdater {
			id: id,

			broadcasts: &mut self.broadcasts,
			maneuvers : &mut self.maneuvers,
			ships     : &mut self.ships,
		}
	}

	pub fn destroy_entity(&mut self, id: &EntityId) {
		self.broadcasts.remove(id);
		self.maneuvers.remove(id);
		self.ships.remove(id);
	}
}


// TODO: Remove #[allow(unused)], once this is moved to a library.
#[allow(unused)]
pub struct EntityBuilder<'c> {
	id: EntityId,

	broadcasts: &'c mut Components<Broadcast>,
	maneuvers : &'c mut Components<Maneuver>,
	ships     : &'c mut Components<Ship>,
}

// TODO: Remove #[allow(unused)], once this is moved to a library.
#[allow(unused)]
impl<'c> EntityBuilder<'c> {
	pub fn with_broadcast(mut self, broadcast: Broadcast) -> EntityBuilder<'c> {
		self.broadcasts.insert(self.id, broadcast);
		self
	}

	pub fn with_maneuver(mut self, maneuver: Maneuver) -> EntityBuilder<'c> {
		self.maneuvers.insert(self.id, maneuver);
		self
	}

	pub fn with_ship(mut self, ship: Ship) -> EntityBuilder<'c> {
		self.ships.insert(self.id, ship);
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
	ships     : &'c mut Components<Ship>,
}

// TODO: Remove #[allow(unused)], once this is moved to a library.
#[allow(unused)]
impl<'c> EntityUpdater<'c> {
	pub fn add_broadcast(mut self, broadcast: Broadcast) -> EntityUpdater<'c> {
		self.broadcasts.insert(self.id, broadcast);
		self
	}

	pub fn add_maneuver(mut self, maneuver: Maneuver) -> EntityUpdater<'c> {
		self.maneuvers.insert(self.id, maneuver);
		self
	}

	pub fn add_ship(mut self, ship: Ship) -> EntityUpdater<'c> {
		self.ships.insert(self.id, ship);
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
