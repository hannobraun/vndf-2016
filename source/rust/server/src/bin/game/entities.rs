use std::collections::HashMap;
use std::vec::Drain;

use shared::game::{
	Broadcast,
	EntityId,
	Ship,
};


#[derive(Debug)]
pub struct Entities {
	next_id: u64,

	broadcasts: HashMap<EntityId, Broadcast>,
	ships     : HashMap<EntityId, Ship>,

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

	pub fn new_entity<C>(&mut self, constructor: C) -> EntityId
		where C: FnOnce(Entity) -> Entity
	{
		let id = self.next_id;
		self.next_id += 1;

		let handle = constructor(Entity::new());

		if let Component::Add(broadcast) = handle.broadcast {
			self.broadcasts.insert(id, broadcast);
		}
		if let Component::Add(ship) = handle.ship {
			self.ships.insert(id, ship);
		}

		id
	}

	pub fn destroy_entity(&mut self, id: &EntityId) {
		self.ships.remove(id);
		self.broadcasts.remove(id);
	}

	pub fn ship(&mut self, id: &EntityId) -> &mut Ship {
		self.ships.get_mut(id)
			.unwrap_or_else(|| panic!("Ship not found: {}", id))
	}

	pub fn add_broadcast(&mut self, id: EntityId, broadcast: Broadcast) {
		self.broadcasts.insert(id, broadcast);
	}

	pub fn remove_broadcast(&mut self, id: &EntityId) {
		self.broadcasts.remove(id);
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


pub struct Entity {
	broadcast: Component<Broadcast>,
	ship     : Component<Ship>,
}

impl Entity {
	fn new() -> Entity {
		Entity {
			broadcast: Component::NoChange,
			ship     : Component::NoChange,
		}
	}

	pub fn add_ship(mut self, ship: Ship) -> Entity {
		self.ship = Component::Add(ship);
		self
	}

	// TODO: This should have an equivalent `with_broadcast` method, but it's
	//       not required right now.
}


enum Component<T> {
	Add(T),
	Remove(T),
	NoChange,
}
