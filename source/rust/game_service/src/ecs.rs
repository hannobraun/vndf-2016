use cgmath::{
	EuclideanVector,
	Vector,
	Vector3,
};

use game::ecs::Entity as SharedEntity;
use game::ecs::{
	Planet,
	Visual,
};
use game::physics::Body;
use net::ConnId;
use rustecs::{
	Components,
	Control,
	EntityId,
};


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Player {
	pub client_id    : ConnId,
	pub ship_id      : Option<EntityId>,
	pub missile_index: u64,
	pub last_snapshot: Vec<(EntityId, SharedEntity)>
}

impl Player {
	pub fn new(conn_id: ConnId, ship_id: EntityId) -> Player {
		Player {
			client_id    : conn_id,
			ship_id      : Some(ship_id),
			missile_index: 0,
			last_snapshot: Vec::new(),
		}
	}
}


world! { World,
	components Body, Visual, Planet, Player;
}


// Systems. Should be integrated with Rustecs at some point.
pub fn integrate(delta_time_in_s: f64, bodies: &mut Components<Body>) {
	for (_, body) in bodies.iter_mut() {
		body.velocity = body.velocity + body.force.mul_s(delta_time_in_s);
		body.position = body.position + body.velocity.mul_s(delta_time_in_s);
		body.force    = Vector3::zero();
	}
}

pub fn kill_colliding_ships(
	bodies : &Components<Body>,
	planets: &Components<Planet>,
	control: &mut Control<Entity>,
) {
	for (&body_id, body) in bodies.iter() {
		for (_, planet) in planets.iter() {
			if (body.position - planet.position).length() <= planet.radius {
				control.remove(body_id);
			}
		}
	}
}


// Utility functions
pub fn entity_id_from_conn_id(
	world    : &World,
	client_id: ConnId
) -> Option<EntityId> {
	for (&id, player) in world.players.iter() {
		if player.client_id == client_id {
			return Some(id);
		}
	}

	None
}
