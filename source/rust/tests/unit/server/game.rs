use vndf::server::game::state::GameState;
use vndf::shared::game::EntityId;
use vndf::shared::util::{
	angle_from,
	roughly_equal,
};


#[test]
fn it_should_execute_multiple_maneuvers_after_each_other() {
	let mut game_state = GameState::new();

	let ship_id = game_state.on_enter();

	let delay_a = 0.5;
	let delay_b = 1.0;

	let direction_a = 1.0;
	let direction_b = 2.0;

	game_state.on_schedule_maneuver(ship_id, delay_a, direction_a, 0.0);
	game_state.on_schedule_maneuver(ship_id, delay_b, direction_b, 0.0);

	game_state.on_update(delay_a + 0.1);
	assert!(ship_has_direction(ship_id, direction_a, &mut game_state));

	game_state.on_update(delay_b + 0.1);
	assert!(ship_has_direction(ship_id, direction_b, &mut game_state));


	fn ship_has_direction(
		ship_id  : EntityId,
		direction: f64,
		game_state: &mut GameState,
	) -> bool {
		for (id, (body, _)) in game_state.export_entities() {
			print!("{} == {}", angle_from(body.velocity), direction);
			let direction_matches = roughly_equal(
				angle_from(body.velocity),
				direction,
				0.001,
			);

			if id == ship_id && direction_matches {
				return true;
			}
		}

		false
	}
}
