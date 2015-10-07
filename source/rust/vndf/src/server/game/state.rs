use std::vec::Drain;

use nalgebra::{
    Norm,
    Rot2,
    Rotate,
    Vec1,
    Vec2,
};

use server::game::data::Spawner;
use server::game::entities::Entities;
use shared::game::{
    Broadcast,
    EntityId,
    Maneuver,
    ManeuverData,
};
use shared::protocol::server::Entity;


pub trait GameEvent {
    type Output;

    fn execute(self, game_state: &mut GameState) -> Self::Output;
}


#[derive(Debug)]
pub struct GameState {
    pub entities: Entities,
    pub spawner : Spawner,

    pub to_destroy: Vec<EntityId>,

    export_buffer     : Vec<Entity>,
    destroyed_entities: Vec<EntityId>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            entities: Entities::new(),
            spawner : Spawner::new(),

            export_buffer     : Vec::new(),
            to_destroy        : Vec::new(),
            destroyed_entities: Vec::new(),
        }
    }

    pub fn handle_event<E: GameEvent>(&mut self, event: E) -> E::Output {
        event.execute(self)
    }

    pub fn on_start_broadcast(&mut self, ship_id: EntityId, message: String) {
        self.entities.update_entity(ship_id)
            .add_broadcast(Broadcast {
                sender : ship_id,
                message: message,
            });
    }

    pub fn on_stop_broadcast(&mut self, ship_id: EntityId) {
        self.entities.update_entity(ship_id).remove_broadcast();
    }

    pub fn on_schedule_maneuver(
        &mut self,
        ship_id: EntityId,
        data   : ManeuverData,
    ) {
        self.entities.create_entity()
            .with_maneuver(Maneuver {
                ship_id: ship_id,
                data   : data,
            });
    }

    pub fn on_cancel_maneuver(
        &mut self,
        ship_id    : EntityId,
        maneuver_id: EntityId,
    ) {
        match self.entities.maneuvers.get(&maneuver_id) {
            Some(maneuver) => {
                if maneuver.ship_id == ship_id {
                    self.to_destroy.push(maneuver_id);
                }
                else {
                    // This could be a bug or malicious behavior.
                    debug!(
                        "{}. Ship: {}; Maneuver: {}",
                        "Player tried to cancel foreign maneuver",
                        ship_id,
                        maneuver_id,
                    );
                }
            },
            None =>
                // This could happen, if the maneuver was finished while the
                // cancel message was in flight. It might also be the symptom of
                // a bug.
                debug!("Could not find maneuver: {}", maneuver_id),
        }
    }

    pub fn on_update(&mut self, now_s: f64) {
        self.apply_maneuvers(now_s);
        self.apply_gravity();
        self.integrate();
        self.check_collisions();

        for id in self.to_destroy.drain(..) {
            self.entities.destroy_entity(&id);
            self.destroyed_entities.push(id);
        }
    }

    pub fn export_entities(&mut self) -> Drain<Entity> {
        for id in &self.entities.entities {
            let body = self.entities.bodies
                .get(id)
                .map(|body|
                     *body
                );
            let broadcast = self.entities.broadcasts
                .get(id)
                .map(|broadcast|
                     broadcast.clone()
                );
            let maneuver = self.entities.maneuvers
                .get(id)
                .map(|maneuver|
                    *maneuver
                );
            let planet = self.entities.planets
                .get(id)
                .map(|planet|
                    *planet
                );
            let ship = self.entities.ships
                .get(id)
                .map(|ship|
                    *ship
                );

            self.export_buffer.push(Entity {
                id: *id,

                body     : body,
                broadcast: broadcast,
                maneuver : maneuver,
                planet   : planet,
                ship     : ship,
            });
        }

        self.export_buffer.drain(..)
    }

    pub fn get_entities(&self) -> &Entities {
        &self.entities
    }

    pub fn destroyed_entities(&mut self) -> Drain<EntityId> {
        self.destroyed_entities.drain(..)
    }


    fn apply_maneuvers(&mut self, now_s: f64) {
        for (&id, maneuver) in &mut self.entities.maneuvers {
            if now_s >= maneuver.data.start_s {
                let thrust = match maneuver.data.thrust {
                    thrust if thrust > 1.0 => 1.0,
                    thrust if thrust < 0.0 => 0.0,

                    thrust => thrust,
                };

                let rotation = Rot2::new(Vec1::new(maneuver.data.angle));
                let force    = rotation.rotate(&Vec2::new(1.0, 0.0));
                let force    = force * thrust;

                match self.entities.bodies.get_mut(&maneuver.ship_id) {
                    Some(body) =>
                        body.force = body.force + force,

                    // The ship might not exist due to timing issues (it could
                    // have been destroyed while the message was in flight). If
                    // this happens too often, it might also be the symptom of a
                    // bug.
                    None => debug!("Ship not found: {}", maneuver.ship_id),
                }
            }

            if now_s >= maneuver.data.start_s + maneuver.data.duration_s {
                self.to_destroy.push(id);
            }
        }
    }

    fn apply_gravity(&mut self) {
        for (_, planet) in &self.entities.planets {
            for (_, body) in &mut self.entities.bodies {
                let g = 6.674e-11; // unit: N * m^2 / kg^2

                let body_to_planet = body.position - planet.position;
                let distance       = body_to_planet.norm();
                let direction      = body_to_planet / distance;

                let force =
                    direction * -g * (planet.mass * body.mass) / distance;

                body.force = body.force + force;
            }
        }
    }

    fn integrate(&mut self) {
         for (_, body) in &mut self.entities.bodies {
            // TODO(E7GyYwQy): Take passed time since last iteration into
            //                 account.
            body.velocity = body.velocity + body.force;
            body.position = body.position + body.velocity;

            body.force = Vec2::new(0.0, 0.0);
        }
    }

    fn check_collisions(&mut self) {
        for (&body_id, body) in &self.entities.bodies {
            for (_, planet) in &self.entities.planets {
                let squared_size = planet.size * planet.size;

                if (body.position - planet.position).sqnorm() < squared_size {
                    self.to_destroy.push(body_id);
                }
            }
        }
    }
}
