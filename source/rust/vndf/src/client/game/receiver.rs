use time;

use rustecs::EntityId;

use common::ecs::{
	ClientWorld,
	SharedWorldEntity,
};
use protocol;



pub type Perception = protocol::Perception<EntityId, SharedWorldEntity>;


pub fn receive(world: &mut ClientWorld, perception: Perception) {
	let current_time = time::precise_time_ns();

	for entity in perception.added.iter() {
		world.import_cliententity(
			entity.id,
			entity.body.unwrap(),
			entity.visual.unwrap(),
			current_time
		);
	}

	for entity in perception.updated.iter() {
		*world.visuals.get_mut(&entity.id) = entity.visual.unwrap();

		world.interpolateds.get_mut(&entity.id).current      = entity.body;
		world.interpolateds.get_mut(&entity.id).current_time = current_time;
	}

	for entity in perception.removed.iter() {
		world.destroy_entity(entity.id);
	}
}
