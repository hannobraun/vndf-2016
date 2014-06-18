use time;

use rustecs::EntityId;

use common::ecs::{
	ClientWorld,
	SharedWorldEntity,
};
use common::protocol;



type Perception = protocol::Perception<EntityId, SharedWorldEntity>;


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
}


#[cfg(test)]
mod test {
	use common::ecs::{
		ClientWorld,
		SharedWorldEntity,
		ShowAsMissile,
	};
	use common::physics::Body;
	use common::protocol::Perception;

	use super::receive;

	#[test]
	fn it_should_import_added_entities() {
		let mut world = ClientWorld::new();

		let entity = SharedWorldEntity {
			id    : 5,
			visual: Some(ShowAsMissile),
			body  : Some(Body::default()),
		};

		let perception = Perception {
			self_id: 0,
			added  : vec!(entity),
			removed: vec!(),
			updated: vec!(),
		};

		receive(&mut world, perception);

		let entities = world.to_entities();

		assert_eq!(1, entities.len());
		assert!(entities.get(0).id == 5);
	}
}
