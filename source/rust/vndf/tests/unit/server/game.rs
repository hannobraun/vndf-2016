use vndf::server::game::state::{GameState};
use vndf::shared::game::{
	Body,
	EntityId,
	ManeuverData,
};
use vndf::shared::util::angle_of;


#[test]
fn it_should_execute_multiple_maneuvers_after_each_other() {
	let mut game_state = GameState::new();

	let ship_id = game_state.on_enter();

	let maneuver_a = ManeuverData {
		start_s   : 0.5,
		duration_s: 0.05,
		angle     : 1.0,
	};
	let maneuver_b = ManeuverData {
		start_s   : 1.0,
		duration_s: 0.05,
		angle     : -1.0,
	};

	game_state.on_schedule_maneuver(ship_id, maneuver_a);
	game_state.on_schedule_maneuver(ship_id, maneuver_b);

	let before = get_body(ship_id, &mut game_state);
	game_state.on_update(maneuver_a.start_s + 0.1);
	let after = get_body(ship_id, &mut game_state);

	assert!(angle_has_decreased(maneuver_a.angle, before, after));

	let before = get_body(ship_id, &mut game_state);
	game_state.on_update(maneuver_b.start_s + 0.1);
	let after = get_body(ship_id, &mut game_state);

	assert!(angle_has_decreased(maneuver_b.angle, before, after));
}

#[test]
fn maneuvers_should_apply_thrust_over_time() {
	let mut game_state = GameState::new();

	let ship_id = game_state.on_enter();

	let maneuver = ManeuverData {
		start_s   : 0.5,
		duration_s: 0.2,
		angle     : 1.0,
	};

	game_state.on_schedule_maneuver(ship_id, maneuver);

	let before = get_body(ship_id, &mut game_state);
	game_state.on_update(maneuver.start_s + maneuver.duration_s / 2.0);
	let after = get_body(ship_id, &mut game_state);

	assert!(angle_has_decreased(maneuver.angle, before, after));

	let before = get_body(ship_id, &mut game_state);
	game_state.on_update(maneuver.start_s + maneuver.duration_s);
	let after = get_body(ship_id, &mut game_state);

	assert!(angle_has_decreased(maneuver.angle, before, after));
}


fn get_body(body_id: EntityId, game_state: &mut GameState) -> Body {
	for (id, (body, _, _)) in game_state.export_entities() {
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
