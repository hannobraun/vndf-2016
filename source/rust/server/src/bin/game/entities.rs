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

	broadcasts: Components<Broadcast>,
	ships     : Components<Ship>,

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

	pub fn get_entity<F>(&mut self, id: EntityId, f: F)
		where F: FnOnce(Entity) -> Entity
	{
		let handle = f(Entity::new());

		macro_rules! handle_component {
			($component:ident, $components:expr) => {
				match handle.$component {
					Component::Add(component) => {
						$components.insert(id, component);
					},
					Component::Remove => {
						$components.remove(&id);
					},
					Component::NoChange => (),
				}
			}
		}

		handle_component!(broadcast, self.broadcasts);
		handle_component!(ship     , self.ships     );
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


pub struct EntityBuilder<'c> {
	id: EntityId,

	broadcasts: &'c mut Components<Broadcast>,
	ships     : &'c mut Components<Ship>,
}

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

	pub fn add_broadcast(mut self, broadcast: Broadcast) -> Entity {
		self.broadcast = Component::Add(broadcast);
		self
	}

	pub fn remove_broadcast(mut self) -> Entity {
		self.broadcast = Component::Remove;
		self
	}
}


enum Component<T> {
	Add(T),
	Remove,
	NoChange,
}
