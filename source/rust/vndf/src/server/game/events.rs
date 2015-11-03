use nalgebra::Vec2;

use server::game::state::{
    GameEvent,
    GameState,
};
use server::game::systems;
use shared::game::data::{
    Body,
    Broadcast,
    EntityId,
    Maneuver,
    ManeuverData,
    Ship,
};
use shared::game::logic::{
    apply_gravity,
    integrate,
};


pub struct Enter;

impl GameEvent for Enter {
    type Output = EntityId;

    fn validate(&self, _: &GameState) -> bool {
        true
    }

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

    fn validate(&self, _: &GameState) -> bool {
        true
    }

    fn execute(self, game_state: &mut GameState) {
        game_state.to_destroy.push(self.ship_id);
    }
}


pub struct StartBroadcast {
    pub ship_id: EntityId,
    pub message: String,
}

impl GameEvent for StartBroadcast {
    type Output = ();

    fn validate(&self, _: &GameState) -> bool {
        true
    }

    fn execute(self, game_state: &mut GameState) {
        game_state.entities.update_entity(self.ship_id)
            .add_broadcast(Broadcast {
                sender : self.ship_id,
                message: self.message,
            });
    }
}


pub struct StopBroadcast {
    pub ship_id: EntityId,
}

impl GameEvent for StopBroadcast {
    type Output = ();

    fn validate(&self, _: &GameState) -> bool {
        true
    }

    fn execute(self, game_state: &mut GameState) {
        game_state.entities
            .update_entity(self.ship_id)
            .remove_broadcast();
    }
}


pub struct ScheduleManeuver {
    pub ship_id: EntityId,
    pub data   : ManeuverData,
}

impl GameEvent for ScheduleManeuver {
    type Output = ();

    fn validate(&self, _: &GameState) -> bool {
        self.data.thrust >= 0.0 && self.data.thrust <= 1.0
    }

    fn execute(self, game_state: &mut GameState) {
        game_state.entities.create_entity()
            .with_maneuver(Maneuver {
                ship_id: self.ship_id,
                data   : self.data,
            });
    }
}


pub struct CancelManeuver {
    pub ship_id    : EntityId,
    pub maneuver_id: EntityId,
}

impl GameEvent for CancelManeuver {
    type Output = ();

    fn validate(&self, game_state: &GameState) -> bool {
        let maneuver =
            match game_state.entities.maneuvers.get(&self.maneuver_id) {
                Some(maneuver) => maneuver,
                None           => return false,
            };

        self.ship_id == maneuver.ship_id
    }

    fn execute(self, game_state: &mut GameState) {
        game_state.to_destroy.push(self.maneuver_id);
    }
}


pub struct FtlJump {
    pub ship_id      : EntityId,
    pub target_time_s: f64,
}

impl GameEvent for FtlJump {
    type Output = ();

    fn validate(&self, _: &GameState) -> bool {
        true
    }

    fn execute(self, game_state: &mut GameState) {
        let mut ship = match game_state.entities.bodies.get_mut(&self.ship_id) {
            Some(ship) => ship,
            None       => return,
        };

        let mut jump_time_s = game_state.time_s;
        while jump_time_s < self.target_time_s {
            let delta_s = 100.0;

            for (_, planet) in &game_state.entities.planets {
                apply_gravity(planet, &mut ship);
            }
            integrate(&mut ship, delta_s);

            jump_time_s += delta_s;
        }
    }
}


pub struct Update {
    pub now_s: f64,
}

impl GameEvent for Update {
    type Output = ();

    fn validate(&self, _: &GameState) -> bool {
        true
    }

    fn execute(self, game_state: &mut GameState) {
        game_state.time_s = self.now_s;

        systems::apply_maneuvers(game_state, self.now_s);
        systems::apply_gravity(game_state);
        systems::move_bodies(game_state, 0.5);
        systems::check_collisions(game_state);

        for id in game_state.to_destroy.drain(..) {
            game_state.entities.destroy_entity(&id);
            game_state.destroyed.push(id);
        }
    }
}
