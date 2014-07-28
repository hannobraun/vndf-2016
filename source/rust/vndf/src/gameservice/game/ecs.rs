use rustecs::EntityId;

use common::ecs::World;
use net::ConnId;


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
