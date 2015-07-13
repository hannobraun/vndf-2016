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
