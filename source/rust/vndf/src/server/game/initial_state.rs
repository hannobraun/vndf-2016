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


pub struct InitialState {
    celestials    : Vec<Celestial>,
    spawn_position: Vec2<f64>,
}

impl InitialState {
    pub fn new() -> Self {
        InitialState {
            celestials    : Vec::new(),
            spawn_position: Vec2::new(0.0, 0.0),
        }
    }

    pub fn random() -> InitialState {
        let mut rng = thread_rng();

        let mut celestials = Vec::new();

        let planet = Celestial {
            position: Vec2::new(0.0, 0.0),
            size    : Range::new(5000.0, 10000.0).sample(&mut rng),
        };
        celestials.push(planet);

        let mut current_distance = 0.0;

        for _ in 0 .. 5 {
            current_distance += Range::new(15000.0, 100000.0).sample(&mut rng);

            let rotation = Rot2::new(Vec1::new(
                Range::new(0.0, 360.0).sample(&mut rng),
            ));
            let position = rotation.rotate(&Vec2::new(current_distance, 0.0));

            celestials.push(Celestial {
                position: position,
                size    : Range::new(500.0, 2000.0).sample(&mut rng),
            });
        }

        let spawn_position =
            planet.position + Vec2::new(0.0, planet.size + 500.0);

        InitialState {
            celestials    : celestials,
            spawn_position: spawn_position,
        }
    }

    pub fn apply(&self, game_state: &mut GameState) {
        let entities = &mut game_state.entities;

        for celestial in &self.celestials {
            entities.create_entity()
                .with_body(Body {
                    position: celestial.position,
                    velocity: Vec2::new(0.0, 0.0),
                    mass    : 1.0, // not used anywhere at the moment
                })
                .with_planet(Planet {
                    color: Colors::random(),
                    size : celestial.size,
                });
        }

        game_state.spawn_position = self.spawn_position;
    }
}


#[derive(Clone, Copy)]
pub struct Celestial {
    pub position: Vec2<f64>,
    pub size    : f64,
}
