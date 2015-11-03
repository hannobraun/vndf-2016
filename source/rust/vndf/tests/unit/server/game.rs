use nalgebra::Vec2;

use vndf::server::game::events;
use vndf::server::game::state::{
    GameEvent,
    GameState,
};
use vndf::shared::game::data::{
    Body,
    EntityId,
    ManeuverData,
};
use vndf::shared::util::angle_of;


#[test]
fn it_should_execute_multiple_maneuvers_after_each_other() {
    let mut game_state = GameState::new(0.0);

    let ship_id = game_state.handle_event(events::Enter).unwrap();

    let maneuver_a = ManeuverData {
        start_s   : 0.5,
        duration_s: 0.05,
        angle     : 1.0,
        thrust    : 1.0,
    };
    let maneuver_b = ManeuverData {
        start_s   : 1.0,
        duration_s: 0.05,
        angle     : -1.0,
        thrust    : 1.0,
    };

    game_state
        .handle_event(events::ScheduleManeuver {
            ship_id: ship_id,
            data   : maneuver_a,
        })
        .unwrap();
    game_state
        .handle_event(events::ScheduleManeuver {
            ship_id: ship_id,
            data   : maneuver_b,
        })
        .unwrap();

    let before = get_body(ship_id, &mut game_state);
    game_state
        .handle_event(events::Update { now_s: maneuver_a.start_s + 0.1 })
        .unwrap();
    let after = get_body(ship_id, &mut game_state);

    assert!(angle_has_decreased(maneuver_a.angle, before, after));

    let before = get_body(ship_id, &mut game_state);
    game_state
        .handle_event(events::Update { now_s: maneuver_b.start_s + 0.1 })
        .unwrap();
    let after = get_body(ship_id, &mut game_state);

    assert!(angle_has_decreased(maneuver_b.angle, before, after));
}

#[test]
fn maneuvers_should_apply_thrust_over_time() {
    let mut game_state = GameState::new(0.0);

    let ship_id = game_state.handle_event(events::Enter).unwrap();

    let maneuver = ManeuverData {
        start_s   : 0.5,
        duration_s: 0.2,
        angle     : 1.0,
        thrust    : 1.0,
    };

    game_state
        .handle_event(events::ScheduleManeuver {
            ship_id: ship_id,
            data   : maneuver,
        })
        .unwrap();

    let before = get_body(ship_id, &mut game_state);
    game_state
        .handle_event(events::Update {
            now_s: maneuver.start_s + maneuver.duration_s / 2.0
        })
        .unwrap();
    let after = get_body(ship_id, &mut game_state);

    assert!(angle_has_decreased(maneuver.angle, before, after));

    let before = get_body(ship_id, &mut game_state);
    game_state
        .handle_event(events::Update {
            now_s: maneuver.start_s + maneuver.duration_s
        })
        .unwrap();
    let after = get_body(ship_id, &mut game_state);

    assert!(angle_has_decreased(maneuver.angle, before, after));
}


#[test]
fn maneuver_thrust_should_be_configurable() {
    let mut game_state = GameState::new(0.0);

    let ship_id_a = game_state.handle_event(events::Enter).unwrap();
    let ship_id_b = game_state.handle_event(events::Enter).unwrap();

    let start_s    = 0.5;
    let duration_s = 1.0;
    let angle      = 0.0;

    let maneuver_a = ManeuverData {
        start_s   : start_s,
        duration_s: duration_s,
        angle     : angle,
        thrust    : 1.0,
    };
    let maneuver_b = ManeuverData {
        start_s   : start_s,
        duration_s: duration_s,
        angle     : angle,
        thrust    : 0.5,
    };

    game_state
        .handle_event(events::ScheduleManeuver {
            ship_id: ship_id_a,
            data   : maneuver_a,
        })
        .unwrap();
    game_state
        .handle_event(events::ScheduleManeuver {
            ship_id: ship_id_b,
            data   : maneuver_b,
        })
        .unwrap();
    game_state
        .handle_event(events::Update { now_s: start_s + duration_s })
        .unwrap();

    let body_a = get_body(ship_id_a, &mut game_state);
    let body_b = get_body(ship_id_b, &mut game_state);

    assert!(body_a.velocity.x > body_b.velocity.x);
}

#[test]
fn game_state_should_reject_invalid_events() {
    struct InvalidEvent;

    impl GameEvent for InvalidEvent {
        type Output = ();

        fn validate(&self, _: &GameState) -> bool {
            false
        }
        fn execute(self, game_state: &mut GameState) {
            game_state.entities
                .create_entity()
                    .with_body(Body {
                        position: Vec2::new(0.0, 0.0),
                        velocity: Vec2::new(0.0, 0.0),
                        force   : Vec2::new(0.0, 0.0),
                        mass    : 0.0,
                    });
        }
    }

    let mut game_state = GameState::new(0.0);

    let result = game_state.handle_event(InvalidEvent);

    assert!(result.is_err());
    assert_eq!(game_state.entities.bodies.len(), 0);
}

#[test]
fn maneuver_thrust_should_be_validated() {
    let thrust_above_max = events::ScheduleManeuver {
        ship_id: 0,

        data: ManeuverData {
            start_s   : 0.0,
            duration_s: 1.0,
            angle     : 0.0,
            thrust    : 1.01,
        }
    };
    let thrust_below_min = events::ScheduleManeuver {
        ship_id: 0,

        data: ManeuverData {
            start_s   : 0.0,
            duration_s: 1.0,
            angle     : 0.0,
            thrust    : -0.01,
        }
    };

    let game_state = GameState::new(0.0);

    assert_eq!(thrust_above_max.validate(&game_state), false);
    assert_eq!(thrust_below_min.validate(&game_state), false);
}

#[test]
fn players_should_only_be_able_to_cancel_their_own_maneuvers() {
    let mut game_state = GameState::new(0.0);

    let ship_id_a = game_state.handle_event(events::Enter).unwrap();
    let ship_id_b = game_state.handle_event(events::Enter).unwrap();

    let maneuver = ManeuverData {
        start_s   : 0.5,
        duration_s: 1.0,
        angle     : 0.0,
        thrust    : 1.0,
    };

    game_state
        .handle_event(events::ScheduleManeuver {
            ship_id: ship_id_a,
            data   : maneuver,
        })
        .unwrap();
    game_state
        .handle_event(events::ScheduleManeuver {
            ship_id: ship_id_b,
            data   : maneuver,
        })
        .unwrap();

    assert_eq!(game_state.entities.maneuvers.len(), 2);

    let maneuver_id_a = get_maneuver_id(ship_id_a, &mut game_state);
    let result = game_state.handle_event(events::CancelManeuver {
        ship_id    : ship_id_b,
        maneuver_id: maneuver_id_a,
    });
    game_state.handle_event(events::Update { now_s: 0.0 }).unwrap();

    assert!(result.is_err());
    assert_eq!(game_state.entities.maneuvers.len(), 2);
}

#[test]
fn updates_should_update_the_game_time() {
    let mut game_state = GameState::new(0.0);

    let now_s = 10.0;
    game_state.handle_event(events::Update { now_s: now_s }).unwrap();
    assert_eq!(game_state.time_s, now_s);
}


fn get_body(body_id: EntityId, game_state: &mut GameState) -> Body {
    for entity in game_state.export_entities() {
        if entity.id == body_id {
            return entity.body.unwrap();
        }
    }

    unreachable!();
}

fn get_maneuver_id(ship_id: EntityId, game_state: &mut GameState) -> EntityId {
    for (id, maneuver) in &game_state.entities.maneuvers {
        if ship_id == maneuver.ship_id {
            return *id;
        }
    }

    panic!("Maneuver not found");
}

fn angle_has_decreased(direction: f64, before: Body, after: Body) -> bool {
    let old_difference = (direction - angle_of(before.velocity)).abs();
    let new_difference = (direction - angle_of(after.velocity )).abs();

    new_difference < old_difference
}
