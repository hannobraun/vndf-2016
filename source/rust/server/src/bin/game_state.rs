use std::collections::HashMap;
use std::collections::hash_map::Iter;

use shared::game::{
	Broadcast,
	EntityId,
	Ship,
};


#[derive(Debug)]
pub struct GameState {
	next_id: u64,

	broadcasts: HashMap<EntityId, Broadcast>,
	ships     : HashMap<EntityId, Ship>,
}

impl GameState {
	pub fn new() -> GameState {
		GameState {
			next_id   : 0,
			broadcasts: HashMap::new(),
			ships     : HashMap::new(),
		}
	}

	pub fn create_ship(&mut self, ship: Ship) -> EntityId {
		let id = self.next_id;
		self.next_id += 1;

		self.ships.insert(id, ship);

		id
	}

	pub fn ship(&mut self, id: &EntityId) -> &mut Ship {
		self.ships.get_mut(id)
			.unwrap_or_else(|| panic!("Ship not found: {}", id))
	}

	pub fn ships(&self) -> Iter<EntityId, Ship> {
		self.ships.iter()
	}

	pub fn add_broadcast(&mut self, id: EntityId, broadcast: Broadcast) {
		self.broadcasts.insert(id, broadcast);
	}

	pub fn remove_broadcast(&mut self, id: &EntityId) {
		self.broadcasts.remove(id);
	}

	pub fn destroy_entity(&mut self, id: &EntityId) {
		self.ships.remove(id);
		self.broadcasts.remove(id);
	}

	pub fn export_entity(&self, id: &EntityId) -> (Ship, Option<Broadcast>) {
		(
			self.ships[id],
			self.broadcasts.get(id).map(|broadcast| broadcast.clone()),
		)
	}

	pub fn update(&mut self) {
		for (_, ship) in &mut self.ships {
			// TODO(E7GyYwQy): Take passed time since last iteration into
			//                 account.
			ship.position = ship.position + ship.velocity;
		}
	}
}
