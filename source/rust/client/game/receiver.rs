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

	for entity in perception.updated.iter() {
		*world.bodies.get_mut(&entity.id)  = entity.body.unwrap();
		*world.visuals.get_mut(&entity.id) = entity.visual.unwrap();

		world.interpolateds.get_mut(&entity.id).current      = entity.body;
		world.interpolateds.get_mut(&entity.id).current_time = current_time;
	}
}


#[cfg(test)]
mod test {
	use common::ecs::{
		ClientWorld,
		SharedWorldEntity,
		ShowAsMissile,
	};
	use common::physics::{
		Body,
		Vec2
	};
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

	#[test]
	fn it_should_update_entities() {
		let entity_id = 5;

		let mut world = ClientWorld::new();
		world.import_cliententity(entity_id, Body::default(), ShowAsMissile, 0);

		let mut entity = SharedWorldEntity {
			id    : entity_id,
			visual: Some(ShowAsMissile),
			body  : Some(Body::default()),
		};
		entity.body.get_mut_ref().position = Vec2(5.0, 8.0);

		let perception = Perception {
			self_id: 0,
			added  : vec!(),
			removed: vec!(),
			updated: vec!(entity),
		};

		receive(&mut world, perception);

		assert_eq!(
			entity.body.unwrap(),
			world.interpolateds.get(&entity_id).current.unwrap()
		);
	}
}
