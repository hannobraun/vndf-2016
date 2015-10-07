use std::vec::Drain;

use server::game::data::Spawner;
use server::game::entities::Entities;
use server::game::systems;
use shared::game::EntityId;
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
    pub destroyed : Vec<EntityId>,

    export_buffer: Vec<Entity>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            entities: Entities::new(),
            spawner : Spawner::new(),

            export_buffer     : Vec::new(),
            to_destroy        : Vec::new(),
            destroyed         : Vec::new(),
        }
    }

    pub fn handle_event<E: GameEvent>(&mut self, event: E) -> E::Output {
        event.execute(self)
    }

    pub fn on_update(&mut self, now_s: f64) {
        systems::apply_maneuvers(self, now_s);
        systems::apply_gravity(self);
        systems::integrate(self);
        systems::check_collisions(self);

        for id in self.to_destroy.drain(..) {
            self.entities.destroy_entity(&id);
            self.destroyed.push(id);
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
        self.destroyed.drain(..)
    }
}
