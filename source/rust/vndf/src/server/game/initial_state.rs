use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
use rustc_serialize::json;

use server::game::data::Spawner;
use server::game::state::GameState;
use shared::color::Colors;
use shared::game::{
    Body,
    Planet,
};


#[derive(Debug, RustcDecodable, RustcEncodable)]
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

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let mut file = match File::open(path) {
            Ok(file)   => file,
            Err(error) => panic!("Error opening initial state file: {}", error),
        };

        let mut initial_state_data = String::new();
        if let Err(error) = file.read_to_string(&mut initial_state_data) {
            panic!("Error reading initial state: {}", error);
        }

        match json::decode(&initial_state_data) {
            Ok(initial_state) =>
                initial_state,
            Err(error) =>
                panic!("Error decoding initial state: {}", error),
        }
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        let initial_state_data = match json::encode(self) {
            Ok(data)   => data,
            Err(error) => panic!("Error encoding initial state: {}", error),
        };

        let mut file = match File::create(path) {
            Ok(file)   => file,
            Err(error) => panic!("Error creating file: {}", error),
        };

        if let Err(error) = file.write_all(initial_state_data.as_bytes()) {
            panic!("Error writing initial state: {}", error);
        }
    }

    pub fn with_celestial(mut self, celestial: Celestial) -> Self {
        self.celestials.push(celestial);
        self
    }

    pub fn with_spawn_position(mut self, position: Vec2<f64>) -> Self {
        self.spawn_position = position;
        self
    }

    pub fn apply(&self, game_state: &mut GameState) {
        let entities = &mut game_state.entities;

        for celestial in &self.celestials {
            entities.create_entity()
                .with_body(Body {
                    position: celestial.position,
                    velocity: Vec2::new(0.0, 0.0),
                    force   : Vec2::new(0.0, 0.0),
                    mass    : 1.0, // not used anywhere at the moment
                })
                .with_planet(Planet {
                    color: Colors::random(),
                    size : celestial.size,
                });
        }

        game_state.spawner = Spawner {
            position: self.spawn_position,
        };
    }
}


#[derive(Clone, Copy, Debug, RustcDecodable, RustcEncodable)]
pub struct Celestial {
    pub position: Vec2<f64>,
    pub size    : f64,
}
