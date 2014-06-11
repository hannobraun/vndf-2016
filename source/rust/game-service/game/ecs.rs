use rustecs::EntityId;

use common::ecs::{
	MissileVisual,
	ShipVisual,
	Visual
};
use common::physics::{
	Body,
	Radians,
	Vec2
};

use game::data::Player;
use network::ConnId;


ecs!(
	component(Body, bodies): Body
	component(Visual, visuals): Visual
	component(Player, players): Player

	entity(Missile<Body, Visual>): |position: Vec2, attitude: Radians| {
		let body = Body {
			position: position,
			velocity: Vec2::zero(),
			attitude: attitude
		};

		(body, MissileVisual)
	}
	entity(Ship<Body, Player, Visual>): |client_id: ConnId| {
		let body = Body {
			position: Vec2::zero(),
			velocity: Vec2::zero(),
			attitude: Radians(0.0)
		};

		let player = Player {
			client_id    : client_id,
			missile_index: 0
		};

		(body, player, ShipVisual)
	}

	world(World<Missile, Ship>)
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
