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


world! {
	components Body, Visual, Planet, Player;

	derived_traits Clone;
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

pub fn apply_gravity(
	bodies : &mut Components<Body>,
	planets: &Components<Planet>
) {
	// If you think the exponent should be -11, please consider that we're using
	// km instead of m, so the constant has to be adjusted for that.
	let gravitational_constant = 6.673e-17;
	for (_, body) in bodies.iter_mut() {
		for (_, planet) in planets.iter() {
			let body_to_planet = planet.position - body.position;
			let force =
				gravitational_constant
				* planet.mass
				/ body_to_planet.length2();

			body.force =
				body.force + body_to_planet.normalize().mul_s(force);
		}
	}
}


// Utility functions
pub fn entity_id_from_conn_id(
	world    : &Entities,
	client_id: ConnId
) -> Option<EntityId> {
	for (&id, player) in world.players.iter() {
		if player.client_id == client_id {
			return Some(id);
		}
	}

	None
}
