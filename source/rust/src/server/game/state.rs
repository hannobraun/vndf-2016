use std::vec::Drain;

use nalgebra::{
	Rot2,
	Rotate,
	Vec1,
	Vec2,
};

use server::game::data::Maneuver;
use server::game::entities::Entities;
use shared::game::{
	Body,
	Broadcast,
	EntityId,
};


#[derive(Debug)]
pub struct GameState {
	entities     : Entities,
	export_buffer: Vec<(EntityId, (Body, Option<Broadcast>))>,
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
			.with_body(Body {
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

	pub fn on_schedule_maneuver(
		&mut self,
		ship_id    : EntityId,
		delay_s    : f64,
		_duration_s: f64,
		angle      : f64,
		now_s      : f64,
	) {
		self.entities.create_entity()
			.with_maneuver(Maneuver {
				ship_id: ship_id,
				start_s: now_s + delay_s,
				angle  : angle,
			});
	}

	pub fn on_update(&mut self, now_s: f64) {
		for (_, body) in &mut self.entities.bodies {
			// TODO(E7GyYwQy): Take passed time since last iteration into
			//                 account.
			body.position = body.position + body.velocity;
		}


		let mut to_destroy = Vec::new();
		for (&id, maneuver) in &mut self.entities.maneuvers {
			if now_s >= maneuver.start_s {
				to_destroy.push(id);

				let rotation = Rot2::new(Vec1::new(maneuver.angle));
				let new_velocity = rotation.rotate(&Vec2::new(1.0, 0.0));

				match self.entities.bodies.get_mut(&maneuver.ship_id) {
					Some(body) => body.velocity = new_velocity,

					// The ship might not exist due to timing issues (it could
					// have been destroyed while the message was in flight). If
					// this happens too often, it might also be the symptom of a
					// bug.
					None => debug!("Ship not found: {}", maneuver.ship_id),
				}
			}
		}

		for id in to_destroy {
			self.entities.destroy_entity(&id);
		}
	}

	pub fn export_entities(&mut self)
		-> Drain<(EntityId, (Body, Option<Broadcast>))>
	{
		for (id, body) in &self.entities.bodies {
			let broadcast = self.entities.broadcasts
				.get(id)
				.map(|broadcast|
					broadcast.clone()
				);

			self.export_buffer.push((*id, (*body, broadcast)))
		}

		self.export_buffer.drain(..)
	}
}
