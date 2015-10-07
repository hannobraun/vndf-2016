use std::vec::Drain;

use server::game::data::Spawner;
use server::game::entities::Entities;
use shared::game::EntityId;
use shared::protocol::server::Entity;


pub trait GameEvent {
    type Output;

    fn validate(&self, game_state: &GameState) -> bool;
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

            to_destroy: Vec::new(),
            destroyed : Vec::new(),

            export_buffer: Vec::new(),
        }
    }

    pub fn handle_event<E>(&mut self, event: E) -> Result<E::Output, ()>
        where E: GameEvent
    {
        Ok(event.execute(self))
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
