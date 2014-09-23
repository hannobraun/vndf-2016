#![feature(phase)]


extern crate serialize;

extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Position(f64, f64);

#[deriving(Clone, Decodable, Encodable, Eq, PartialEq, Show)]
pub enum Visual {
	RenderAsMissile,
	RenderAsShip,
}

type Score = u32;


world!(
	Missile(Position, Visual): |x: f64, y: f64| {
		(
			Position(x, y),
			RenderAsMissile
		)
	}
	Ship(Position, Visual, Score): |score: u32| {
		(
			Position(0.0, 0.0),
			RenderAsShip,
			score
		)
	}
)


#[test]
fn it_should_initialize_an_empty_world() {
	let world = World::new();

	assert_eq!(0, world.positions.len());
	assert_eq!(0, world.visuals.len());
	assert_eq!(0, world.scores.len());
}

#[test]
fn it_should_create_entities() {
	let mut world = World::new();

	let missile_id = world.create_missile(8.0, 12.0);

	assert_eq!(1, world.positions.len());
	assert_eq!(1, world.visuals.len());
	assert_eq!(0, world.scores.len());

	assert_eq!(&Position(8.0, 12.0), world.positions.get(&missile_id));
	assert_eq!(&RenderAsMissile    , world.visuals.get(&missile_id));

	let ship_id = world.create_ship(100);

	assert_eq!(2, world.positions.len());
	assert_eq!(2, world.visuals.len());
	assert_eq!(1, world.scores.len());

	assert_eq!(&Position(0.0, 0.0), world.positions.get(&ship_id));
	assert_eq!(&RenderAsShip      , world.visuals.get(&ship_id));
	assert_eq!(&100               , world.scores.get(&ship_id));
}

#[test]
fn it_should_destroy_entities() {
	let mut world = World::new();

	let id = world.create_ship(100);
	world.destroy_entity(id);

	assert_eq!(0, world.positions.len());
	assert_eq!(0, world.visuals.len());
	assert_eq!(0, world.scores.len());
}

#[test]
fn it_should_export_entities() {
	let mut world = World::new();

	let missile_id = world.create_missile(8.0, 12.0);
	let ship_id    = world.create_ship(100);

	let entities = world.to_entities();

	assert_eq!(2, entities.len());

	let missile = Entity {
		id      : missile_id,
		position: Some(Position(8.0, 12.0)),
		visual  : Some(RenderAsMissile),
		score   : None
	};
	let ship = Entity {
		id      : ship_id,
		position: Some(Position(0.0, 0.0)),
		visual  : Some(RenderAsShip),
		score   : Some(100),
	};

	for &entity in entities.iter() {
		if entity.id == missile_id {
			assert_eq!(missile, entity);
		}
		else if entity.id == ship_id {
			assert_eq!(ship, entity);
		}
		else {
			fail!("Unexpected id: {}", entity.id);
		}
	}
}

#[test]
fn it_should_import_entities() {
	let mut old_world = World::new();

	let missile_id = old_world.create_missile(8.0, 12.0);
	let ship_id    = old_world.create_ship(100);

	let world = World::from_entities(old_world.to_entities());

	assert_eq!(2, world.positions.len());
	assert_eq!(2, world.visuals.len());
	assert_eq!(1, world.scores.len());

	assert_eq!(&Position(8.0, 12.0), world.positions.get(&missile_id));
	assert_eq!(&RenderAsMissile    , world.visuals.get(&missile_id));

	assert_eq!(&Position(0.0, 0.0), world.positions.get(&ship_id));
	assert_eq!(&RenderAsShip      , world.visuals.get(&ship_id));
	assert_eq!(&100               , world.scores.get(&ship_id));
}

#[test]
fn it_should_import_single_entities() {
	let mut world = World::new();

	world.import_missile(5, 8.0, 12.0);

	assert_eq!(1, world.positions.len());
	assert_eq!(1, world.visuals.len());
	assert_eq!(0, world.scores.len());

	assert_eq!(&Position(8.0, 12.0), world.positions.get(&5));
	assert_eq!(&RenderAsMissile    , world.visuals.get(&5));

	world.import_ship(7, 100);

	assert_eq!(2, world.positions.len());
	assert_eq!(2, world.visuals.len());
	assert_eq!(1, world.scores.len());

	assert_eq!(&Position(0.0, 0.0), world.positions.get(&7));
	assert_eq!(&RenderAsShip      , world.visuals.get(&7));
	assert_eq!(&100               , world.scores.get(&7));
}
