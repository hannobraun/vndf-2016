use physics::{
	Body,
	Vec2
};
use vndf::client::game::receiver::receive;
use vndf::game::ecs::{
	ClientWorld,
	SharedWorldEntity,
	ShowAsMissile,
};
use vndf::protocol::Perception;


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

#[test]
fn it_should_destroy_removed_entities() {
	let entity_id = 5;

	let mut world = ClientWorld::new();
	world.import_cliententity(entity_id, Body::default(), ShowAsMissile, 0);

	let entity = SharedWorldEntity {
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
