use std::collections::HashMap;
use std::vec::Drain;

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
	pub ships     : Components<Ship>,

	export_buffer: Vec<(EntityId, (Ship, Option<Broadcast>))>,
}

impl Entities {
	pub fn new() -> Entities {
		Entities {
			next_id      : 0,
			broadcasts   : HashMap::new(),
			ships        : HashMap::new(),
			export_buffer: Vec::new(),
		}
	}

	pub fn create_entity(&mut self) -> EntityBuilder {
		let id = self.next_id;
		self.next_id += 1;

		EntityBuilder::new(
			id,
			&mut self.broadcasts,
			&mut self.ships,
		)
	}

	pub fn update_entity(&mut self, id: EntityId) -> EntityUpdater {
		EntityUpdater {
			id: id,

			broadcasts: &mut self.broadcasts,
			ships     : &mut self.ships,
		}
	}

	pub fn destroy_entity(&mut self, id: &EntityId) {
		self.ships.remove(id);
		self.broadcasts.remove(id);
	}

	// TODO: Remove as soon as possible
	pub fn ship(&mut self, id: &EntityId) -> &mut Ship {
		self.ships.get_mut(id)
			.unwrap_or_else(|| panic!("Ship not found: {}", id))
	}

	// TODO: Move to GameState
	pub fn export_entities(&mut self)
		-> Drain<(EntityId, (Ship, Option<Broadcast>))>
	{
		for (id, ship) in &self.ships {
			let broadcast =
				self.broadcasts.get(id).map(|broadcast| broadcast.clone());

			self.export_buffer.push((*id, (*ship, broadcast)))
		}

		self.export_buffer.drain(..)
	}

	// TODO: Move to GameState
	pub fn update(&mut self) {
		for (_, ship) in &mut self.ships {
			// TODO(E7GyYwQy): Take passed time since last iteration into
			//                 account.
			ship.position = ship.position + ship.velocity;
		}
	}
}


// TODO: Remove #[allow(unused)], once this is moved to a library.
#[allow(unused)]
pub struct EntityBuilder<'c> {
	id: EntityId,

	broadcasts: &'c mut Components<Broadcast>,
	ships     : &'c mut Components<Ship>,
}

// TODO: Remove #[allow(unused)], once this is moved to a library.
#[allow(unused)]
impl<'c> EntityBuilder<'c> {
	fn new(
		id        : EntityId,
		broadcasts: &'c mut Components<Broadcast>,
		ships     : &'c mut Components<Ship>
	)
		-> EntityBuilder<'c>
	{
		EntityBuilder {
			id: id,

			broadcasts: broadcasts,
			ships     : ships,
		}
	}

	pub fn with_ship(mut self, ship: Ship) -> EntityBuilder<'c> {
		self.ships.insert(self.id, ship);
		self
	}

	pub fn with_broadcast(mut self, broadcast: Broadcast) -> EntityBuilder<'c> {
		self.broadcasts.insert(self.id, broadcast);
		self
	}

	pub fn return_id(self) -> EntityId {
		self.id
	}
}


pub struct EntityUpdater<'c> {
	id: EntityId,

	broadcasts: &'c mut Components<Broadcast>,
	ships     : &'c mut Components<Ship>,
}

impl<'c> EntityUpdater<'c> {
	pub fn add_broadcast(mut self, broadcast: Broadcast) -> EntityUpdater<'c> {
		self.broadcasts.insert(self.id, broadcast);
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

	pub fn remove_ship(mut self) -> EntityUpdater<'c> {
		self.ships.remove(&self.id);
		self
	}
}
