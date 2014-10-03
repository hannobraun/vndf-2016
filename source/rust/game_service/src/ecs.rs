use cgmath::{
	Quaternion,
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
	pub missile_index: u64,
	pub last_snapshot: Vec<SharedEntity>
}


world!(
	Missile(Body, Visual): (position: Vector3<f64>, attitude: Quaternion<f64>) {
		let mut body = Body::default();
		body.position = position;
		body.velocity = Vector3::new(600.0, 0.0, 0.0);
		body.attitude = attitude;

		(body, ShowAsMissile)
	}
	Ship(Body, Player, Visual): (client_id: ConnId) {
		let mut body = Body::default();
		body.position = Vector3::new(3000.0, 0.0, 0.0);
		body.velocity = Vector3::new(-50.0, 0.0, 0.0);

		let player = Player {
			client_id    : client_id,
			missile_index: 0,
			last_snapshot: Vec::new(),
		};

		(body, player, ShowAsShip)
	}
	Planet(Planet): (position: Vector3<f64>, radius: f64, color: Vector3<f32>) {
		(
			Planet {
				position: position,
				radius  : radius,
				color   : color,
			},
		)
	}
)


pub fn destroy_ship(world: &mut World, client_id: ConnId) {
	let id = match entity_id_from_client_id(world, client_id) {
		Some(id) => id,
		None     => return
	};

	world.destroy_entity(id);
}

pub fn entity_id_from_client_id(
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
