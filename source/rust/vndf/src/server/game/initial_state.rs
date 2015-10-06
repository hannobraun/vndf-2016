use nalgebra::{
    Vec1,
    Vec2,
    Rot2,
    Rotate,
};
use rand::thread_rng;
use rand::distributions::{
    Range,
    Sample,
};

use server::game::state::GameState;
use shared::color::Colors;
use shared::game::{
    Body,
    Planet,
};


pub struct InitialState;

impl InitialState {
    pub fn apply(&self, game_state: &mut GameState) {
        let entities = &mut game_state.entities;

        let mut rng = thread_rng();

        let planet_id = entities.create_entity()
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

            let rotation = Rot2::new(Vec1::new(
                Range::new(0.0, 360.0).sample(&mut rng),
            ));
            let position = rotation.rotate(&Vec2::new(current_distance, 0.0));

            entities.create_entity()
                .with_body(Body {
                    position: position,
                    velocity: Vec2::new(0.0, 0.0),
                    mass    : 1.0, // not used anywhere at the moment
                })
                .with_planet(Planet {
                    color: Colors::random(),
                    size : Range::new(500.0, 2000.0).sample(&mut rng),
                })
                .return_id();
        }

        game_state.spawn_position = {
            let body   = entities.bodies[&planet_id];
            let planet = entities.planets[&planet_id];

            body.position + Vec2::new(0.0, planet.size + 500.0)
        };
    }
}
