use cgmath::{
	Quaternion,
	Vector,
	Vector3,
};

use game::ecs::Entity as SharedEntity;
use game::ecs::{
	Planet,
	ShowAsMissile,
	ShowAsShip,
	Visual,
};
use game::physics::Body;
use net::ConnId;
use rustecs::EntityId;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Player {
	pub client_id    : ConnId,
	pub ship_id      : Option<EntityId>,
	pub missile_index: u64,
	pub last_snapshot: Vec<SharedEntity>
}


world!(
	entity_constructor missile(
		position: Vector3<f64>,
		attitude: Quaternion<f64>,
	) -> (Body, Visual) {
		let mut body = Body::default();
		body.position = position;
		body.velocity = Vector3::new(600.0, 0.0, 0.0);
		body.attitude = attitude;

		(body, ShowAsMissile)
	}

	entity_constructor ship() -> (Body, Visual) {
		let mut body = Body::default();
		body.position = Vector3::new(3000.0, 0.0, 0.0);
		body.velocity = Vector3::new(-50.0, 0.0, 0.0);

		(body, ShowAsShip)
	}

	entity_constructor planet(
		position: Vector3<f64>,
		radius  : f64,
		color   : Vector3<f32>,
	) -> (Planet) {
		(
			Planet {
				position: position,
				radius  : radius,
				color   : color,
			},
		)
	}

	entity_constructor player(conn_id: ConnId, ship_id: EntityId) -> (Player) {
		(
			Player {
				client_id    : conn_id,
				ship_id      : Some(ship_id),
				missile_index: 0,
				last_snapshot: Vec::new(),
			},
		)
	}
)


// Systems. Should be integrated with Rustecs at some point.
pub fn integrate(delta_time_in_s: f64, body: &mut Body) {
	body.velocity = body.velocity + body.force.mul_s(delta_time_in_s);
	body.position = body.position + body.velocity.mul_s(delta_time_in_s);
	body.force    = Vector3::zero();
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
