use cgmath::Vector3;
use rustecs::Entities;

use client::ecs::{
	Entity,
	Interpolated,
	World,
};
use client::receiver::receive;
use game::ecs::Entity as SharedEntity;
use game::ecs::ShowAsMissile;
use game::physics::Body;
use protocol::Perception;


#[test]
fn it_should_import_added_entities() {
	let mut world = World::new();

	let entity = SharedEntity {
		visual: Some(ShowAsMissile),
		body  : Some(Body::new()),
		planet: None,
	};

	let perception = Perception {
		self_id: Some(0),
		added  : vec!((5, entity)),
		removed: vec!(),
		updated: vec!(),
	};

	receive(&mut world, perception);

	let entities = world.export();

	assert_eq!(1, entities.len());
	assert!(entities[0].0 == 5);
}

#[test]
fn it_should_update_entities() {
	let entity_id = 5;

	let mut world = World::new();
	world.import(
		entity_id,
		Entity {
			body        : Some(Body::new()),
			visual      : Some(ShowAsMissile),
			interpolated: Some(Interpolated::new(0, Some(Body::new()))),
			planet      : None,
		}
	);

	let mut entity = SharedEntity {
		visual: Some(ShowAsMissile),
		body  : Some(Body::new()),
		planet: None,
	};
	entity.body.as_mut().unwrap().position = Vector3::new(5.0, 8.0, 13.0);

	let perception = Perception {
		self_id: Some(0),
		added  : vec!(),
		removed: vec!(),
		updated: vec!((entity_id, entity)),
	};

	receive(&mut world, perception);

	assert_eq!(
		entity.body.unwrap(),
		world.interpolateds[entity_id].current.unwrap()
	);
}

#[test]
fn it_should_destroy_removed_entities() {
	let entity_id = 5;

	let mut world = World::new();
	world.import(
		entity_id,
		Entity {
			body        : Some(Body::new()),
			visual      : Some(ShowAsMissile),
			interpolated: Some(Interpolated::new(0, Some(Body::new()))),
			planet      : None,
		}
	);

	let entity = SharedEntity {
		visual: Some(ShowAsMissile),
		body  : Some(Body::new()),
		planet: None,
	};

	let perception = Perception {
		self_id: Some(0),
		added  : vec!(),
		removed: vec!((entity_id, entity)),
		updated: vec!(),
	};

	receive(&mut world, perception);

	assert_eq!(0, world.bodies.len());
	assert_eq!(0, world.visuals.len());
	assert_eq!(0, world.interpolateds.len());
}
