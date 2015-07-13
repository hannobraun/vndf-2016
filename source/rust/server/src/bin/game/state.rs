use std::vec::Drain;

use nalgebra::{
	Rot2,
	Rotate,
	Vec1,
	Vec2,
};

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

	pub fn on_enter(&mut self) -> EntityId {
		self.entities.create_entity()
			.with_ship(Ship {
				position: Vec2::new(0.0, 0.0),
				velocity: Vec2::new(1.0, 0.0),
			})
			.return_id()
	}

	pub fn on_leave(&mut self, ship_id: &EntityId) {
		self.entities.destroy_entity(ship_id);
	}

	pub fn on_start_broadcast(&mut self, ship_id: EntityId, message: String) {
		self.entities.update_entity(ship_id)
			.add_broadcast(Broadcast {
				sender : ship_id,
				message: message,
			});
	}

	pub fn on_stop_broadcast(&mut self, ship_id: EntityId) {
		self.entities.update_entity(ship_id).remove_broadcast();
	}

	pub fn on_schedule_maneuver(&mut self, ship_id: EntityId, angle: f64) {
		let rotation = Rot2::new(Vec1::new(angle));
		let new_velocity = rotation.rotate(&Vec2::new(1.0, 0.0));

		match self.entities.ships.get_mut(&ship_id) {
			Some(ship) => ship.velocity = new_velocity,

			// The ship might not exist due to timing issues (it could have been
			// destroyed while the message was in flight). If this happens too
			// often, it might also be the symptom of a bug.
			None => debug!("Ship not found: {}", ship_id),
		}
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
