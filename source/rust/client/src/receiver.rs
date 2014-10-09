use time;

use rustecs::{
	Entities,
	EntityId,
};

use game::ecs::Entity as SharedEntity;
use protocol;

use super::ecs::{
	Entity,
	Interpolated,
	World,
};



pub type Perception = protocol::Perception<EntityId, (EntityId, SharedEntity)>;


// This code should be generic and live with the protocol code. Before this can
// happen, more features have to be added to Rustecs. I've added some comments
// where relevant to explain the details.
pub fn receive(entities: &mut World, perception: Perception) {
	let current_time = time::precise_time_ns();

	for (id, entity) in perception.added.into_iter() {
		// This code is responsible initializing the Interpolated component.
		// This should happen in a system that runs on the import event.
		let interpolated = match entity.body {
			Some(body) => Some(Interpolated::new(current_time, Some(body))),
			None       => None
		};

		// Here, two separate things happen:
		// 1. world.import_entity is called
		// 2. The entity is converted from a shared into a client-side entity
		//
		// Item 2 (the conversion) can happen in a closure that is passed to
		// receive, which is pretty straight-forward.
		// Item 1 is more interesting, and I see basically two solutions:
		// 1. If protocol is allowed to know about Rustecs (which might be an
		//    advantage, since it allows for much more automation), then there
		//    should be a trait, World, that every generated world implements.
		// 2. If protocol is kept ignorant of Rustecs, the import call can be
		//    moved into the closure that does the conversion.
		entities.import(
			id,
			Entity {
				body        : entity.body,
				visual      : entity.visual,
				interpolated: interpolated,
				planet      : entity.planet,
			}
		);
	}

	for &(id, entity) in perception.updated.iter() {
		// This does a straight update of the new component data. There could be
		// a method update_entity akin to import_entity, or maybe import_entity
		// could be reused. If the world doesn't trigger events by itself, then
		// reuse can be possible. Just import_entity, then call trigger_import
		// or trigger_update_received as appropriate.
		match entity.visual {
			Some(visual) => *entities.visuals.get_mut(&id) = visual,
			None => (),
		}

		// This special-purpose code should live in a system that is triggered
		// on the update_received event.
		match entity.body {
			Some(body) => {
				entities.interpolateds.get_mut(&id).current      = Some(body);
				entities.interpolateds.get_mut(&id).current_time = current_time;
			},
			None => (),
		}
	}

	for &(id, _) in perception.removed.iter() {
		// This is already quite generic. The only thing that's required to make
		// it totally generic is trait World.
		entities.remove(id);
	}
}
