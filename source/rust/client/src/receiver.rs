use time;

use game::ecs::Entity as SharedEntity;
use protocol;
use rustecs::EntityId;

use super::ecs::{
	Entity,
	Interpolated,
	World,
};



pub type Perception = protocol::Perception<EntityId, SharedEntity>;


pub fn receive(world: &mut World, perception: Perception) {
	let current_time = time::precise_time_ns();

	for entity in perception.added.into_iter() {
		let interpolated = match entity.body {
			Some(body) => Some(Interpolated::new(current_time, Some(body))),
			None       => None
		};

		world.import_entity(Entity {
			id          : entity.id,
			body        : entity.body,
			visual      : entity.visual,
			interpolated: interpolated,
			planet      : None,
		});
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
