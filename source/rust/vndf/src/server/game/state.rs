use std::vec::Drain;

use nalgebra::{
    Rot2,
    Rotate,
    Vec1,
    Vec2,
};
use rand::thread_rng;
use rand::distributions::{
    Range,
    Sample,
};

use server::game::data::Maneuver;
use server::game::entities::Entities;
use shared::color::Colors;
use shared::game::{
    Body,
    Broadcast,
    EntityId,
    ManeuverData,
    Planet,
    Ship,
};
use shared::protocol::server::Entity;

use shared::physics::SphereCollider;

use client::graphics::SHIP_SIZE; // TODO: move this to top client module


#[derive(Debug)]
pub struct GameState {
    entities     : Entities,
    export_buffer: Vec<Entity>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            entities     : Entities::new(),
            export_buffer: Vec::new(),
        }
    }

    /// currently loads a random state
    pub fn generate_planets (&mut self) -> Vec<EntityId> {
        let mut planets = Vec::new();
        let mut rng     = thread_rng();


        let planet_id = self.entities.create_entity()
            .with_body(Body {
                position: Vec2::new(0.0, 0.0),
                velocity: Vec2::new(0.0, 0.0),
                mass    : 1.0, // not used anywhere at the moment
            })
            .with_planet(Planet {
                color: Colors::random(),
                size : Range::new(5000.0, 10000.0).sample(&mut rng),
            })
            .return_id();

        let mut current_distance = 0.0;

        for _ in 0 .. 5 {
            current_distance += Range::new(15000.0, 100000.0).sample(&mut rng);

            let moon_id = self.entities.create_entity()
                .with_body(Body {
                    position: Vec2::new(current_distance, 0.0),
                    velocity: Vec2::new(0.0, 0.0),
                    mass    : 1.0, // not used anywhere at the moment
                })
                .with_planet(Planet {
                    color: Colors::random(),
                    size : Range::new(500.0, 2000.0).sample(&mut rng),
                })
                .return_id();

            planets.push(moon_id);
        }

        planets.push(planet_id);

        planets
    }
    
    pub fn on_enter(&mut self) -> EntityId {
        self.entities.create_entity()
            .with_body(Body {
                position: Vec2::new(0.0, 0.0),
                velocity: Vec2::new(1.0, 0.0),
                mass: 0.0f32,
            })
            .with_ship(Ship)
            .with_collider(SphereCollider::new_from_oval(SHIP_SIZE))
            .return_id()
    }

    pub fn on_leave(&mut self, ship_id: &EntityId) {
        self.entities.destroy_entity(ship_id);
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

    pub fn on_update(&mut self, now_s: f64) {
        for (_, body) in &mut self.entities.bodies {
            // TODO(E7GyYwQy): Take passed time since last iteration into
            //                 account.
            body.position = body.position + body.velocity;
        }


        let mut to_destroy = Vec::new();
        for (&id, maneuver) in &mut self.entities.maneuvers {
            if now_s >= maneuver.data.start_s {
                let rotation     = Rot2::new(Vec1::new(maneuver.data.angle));
                let acceleration = rotation.rotate(&Vec2::new(1.0, 0.0));

                match self.entities.bodies.get_mut(&maneuver.ship_id) {
                    Some(body) =>
                        // TODO(E7GyYwQy): Take passed time since last iteration
                        //                 into account.
                        body.velocity = body.velocity + acceleration,

                    // The ship might not exist due to timing issues (it could
                    // have been destroyed while the message was in flight). If
                    // this happens too often, it might also be the symptom of a
                    // bug.
                    None => debug!("Ship not found: {}", maneuver.ship_id),
                }
            }

            if now_s >= maneuver.data.start_s + maneuver.data.duration_s {
                to_destroy.push(id);
            }
        }

        for id in to_destroy {
            self.entities.destroy_entity(&id);
        }
    }

    pub fn export_entities(&mut self) -> Drain<Entity> {
        for (id, body) in &self.entities.bodies {
            let broadcast = self.entities.broadcasts
                .get(id)
                .map(|broadcast|
                     broadcast.clone()
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

                body: *body,

                broadcast : broadcast,
                planet    : planet,
                ship      : ship,
            });
        }

        self.export_buffer.drain(..)
    }

    pub fn get_entities(&self) -> &Entities {
        &self.entities
    }
}
