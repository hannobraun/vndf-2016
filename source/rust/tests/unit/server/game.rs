use vndf::server::game::state::GameState;
use vndf::shared::game::{
	Body,
	EntityId,
};
use vndf::shared::util::angle_of;


#[test]
fn it_should_execute_multiple_maneuvers_after_each_other() {
	let mut game_state = GameState::new();

	let ship_id = game_state.on_enter();

	let delay_a = 0.5;
	let delay_b = 1.0;

	let direction_a =  1.0;
	let direction_b = -1.0;

	game_state.on_schedule_maneuver(ship_id, delay_a, direction_a, 0.0);
	game_state.on_schedule_maneuver(ship_id, delay_b, direction_b, 0.0);

	let before = get_body(ship_id, &mut game_state);
	game_state.on_update(delay_a + 0.1);
	let after = get_body(ship_id, &mut game_state);

	assert!(angle_has_decreased(direction_a, before, after));

	let before = get_body(ship_id, &mut game_state);
	game_state.on_update(delay_b + 0.1);
	let after = get_body(ship_id, &mut game_state);

	assert!(angle_has_decreased(direction_b, before, after));
}


fn get_body(body_id: EntityId, game_state: &mut GameState) -> Body {
	for (id, (body, _)) in game_state.export_entities() {
		if id == body_id {
			return body;
		}
	}

	unreachable!();
}

fn angle_has_decreased(direction: f64, before: Body, after: Body, ) -> bool {
	let old_difference = (direction - angle_of(before.velocity)).abs();
	let new_difference = (direction - angle_of(after.velocity )).abs();

	new_difference < old_difference
}
