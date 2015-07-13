use std::vec::Drain;

use game::entities::Entities;
use shared::game::{
	Broadcast,
	EntityId,
	Ship,
};


#[derive(Debug)]
pub struct GameState {
	// TODO: Make entities private
	pub entities: Entities,

	export_buffer: Vec<(EntityId, (Ship, Option<Broadcast>))>,
}

impl GameState {
	pub fn new() -> GameState {
		GameState {
			entities     : Entities::new(),
			export_buffer: Vec::new(),
		}
	}

	pub fn on_leave(&mut self, ship_id: &EntityId) {
		self.entities.destroy_entity(ship_id);
	}

	pub fn on_update(&mut self) {
		for (_, ship) in &mut self.entities.ships {
			// TODO(E7GyYwQy): Take passed time since last iteration into
			//                 account.
			ship.position = ship.position + ship.velocity;
		}
	}

	pub fn export_entities(&mut self)
		-> Drain<(EntityId, (Ship, Option<Broadcast>))>
	{
		for (id, ship) in &self.entities.ships {
			let broadcast = self.entities.broadcasts
				.get(id)
				.map(|broadcast|
					broadcast.clone()
				);

			self.export_buffer.push((*id, (*ship, broadcast)))
		}

		self.export_buffer.drain(..)
	}
}
