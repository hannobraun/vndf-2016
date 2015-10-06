use std::vec::Drain;

use nalgebra::{
    Rot2,
    Rotate,
    Vec1,
    Vec2,
};

use server::game::entities::Entities;
use shared::game::{
    Body,
    Broadcast,
    EntityId,
    Maneuver,
    ManeuverData,
    Ship,
};
use shared::protocol::server::Entity;

use shared::physics::SphereCollider;


#[derive(Debug)]
pub struct GameState {
    pub entities      : Entities,
    pub spawn_position: Vec2<f64>,

    export_buffer     : Vec<Entity>,
    destroyed_entities: Vec<EntityId>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            entities     : Entities::new(),
            export_buffer: Vec::new(),

            destroyed_entities: Vec::new(),

            spawn_position: Vec2::new(0.0, 0.0),
        }
    }
    
    pub fn on_enter(&mut self) -> EntityId {
        self.entities.create_entity()
            .with_body(Body {
                position: self.spawn_position,
                velocity: Vec2::new(1.0, 0.0),
                mass: 0.0f32,
            })
            .with_ship(Ship)
            .with_collider(SphereCollider::new_from_oval(1.0))
            .return_id()
    }

    pub fn on_leave(&mut self, ship_id: &EntityId) {
        self.entities.destroy_entity(ship_id);
        self.destroyed_entities.push(*ship_id);
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
        self.integrate();


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


    fn integrate(&mut self) {
         for (_, body) in &mut self.entities.bodies {
            // TODO(E7GyYwQy): Take passed time since last iteration into
            //                 account.
            body.position = body.position + body.velocity;
        }
    }
}
