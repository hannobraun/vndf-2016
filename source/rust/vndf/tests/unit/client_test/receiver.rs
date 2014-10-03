use cgmath::Vector3;

use client::ecs::World;
use client::receiver::receive;
use game::ecs::{
	Entity,
	ShowAsMissile,
};
use game::physics::Body;
use protocol::Perception;


#[test]
fn it_should_import_added_entities() {
	let mut world = World::new();

	let entity = Entity {
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

	let entities = world.export_entities();

	assert_eq!(1, entities.len());
	assert!(entities[0].id == 5);
}

#[test]
fn it_should_update_entities() {
	let entity_id = 5;

	let mut world = World::new();
	world.import_craft(entity_id, Body::default(), ShowAsMissile, 0);

	let mut entity = Entity {
		id    : entity_id,
		visual: Some(ShowAsMissile),
		body  : Some(Body::default()),
	};
	entity.body.as_mut().unwrap().position = Vector3::new(5.0, 8.0, 13.0);

	let perception = Perception {
		self_id: 0,
		added  : vec!(),
		removed: vec!(),
		updated: vec!(entity),
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
	world.import_craft(entity_id, Body::default(), ShowAsMissile, 0);

	let entity = Entity {
		id    : entity_id,
		visual: Some(ShowAsMissile),
		body  : Some(Body::default()),
	};

	let perception = Perception {
		self_id: 0,
		added  : vec!(),
		removed: vec!(entity),
		updated: vec!(),
	};

	receive(&mut world, perception);

	assert_eq!(0, world.bodies.len());
	assert_eq!(0, world.visuals.len());
	assert_eq!(0, world.interpolateds.len());
}
