use nalgebra::Vec2;

use server::game::state::{
	GameEvent,
	GameState,
};
use shared::game::{
	Body,
	EntityId,
	Ship,
};


pub struct Enter;

impl GameEvent for Enter {
	type Output = EntityId;

	fn execute(self, game_state: &mut GameState) -> EntityId {
		game_state.entities.create_entity()
            .with_body(Body {
                position: game_state.spawner.position,
                velocity: game_state.spawner.velocity,
                force   : Vec2::new(0.0, 0.0),
                mass    : 1.0,
            })
            .with_ship(Ship)
            .return_id()
	}
}


pub struct Leave {
	pub ship_id: EntityId,
}

impl GameEvent for Leave {
	type Output = ();

	fn execute(self, game_state: &mut GameState) {
		game_state.to_destroy.push(self.ship_id);
	}
}
