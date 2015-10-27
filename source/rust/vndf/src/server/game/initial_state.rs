use std::f64::consts::PI;
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
use shared::game::data::Planet;


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct InitialState {
    celestials: Vec<Celestial>,
    spawner   : Spawner,
}

impl InitialState {
    pub fn new() -> Self {
        InitialState {
            celestials: Vec::new(),
            spawner   : Spawner::new(),
        }
    }

    pub fn random() -> InitialState {
        let mut rng = thread_rng();

        let mut celestials = Vec::new();

        let planet = Celestial {
            position: Vec2::new(0.0, 0.0),
            size    : Range::new(500.0, 1000.0).sample(&mut rng),
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
                size    : Range::new(50.0, 200.0).sample(&mut rng),
            });
        }

        let mut spawner = Spawner::new();
        spawner.position =
            planet.position + Vec2::new(0.0, planet.size + 500.0);

        InitialState {
            celestials: celestials,
            spawner   : spawner,
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

    pub fn with_spawner(mut self, spawner: Spawner) -> Self {
        self.spawner = spawner;
        self
    }

    pub fn apply(&self, game_state: &mut GameState) {
        let entities = &mut game_state.entities;

        for celestial in &self.celestials {
            // Let's say mass is just proportional to volume.
            let size_cubed = celestial.size * celestial.size * celestial.size;
            let mass       = size_cubed * 4.0 / 3.0 * PI * 1000.0;

            entities.create_entity()
                .with_planet(Planet {
                    position: celestial.position,
                    radius  : celestial.size,
                    mass    : mass,
                    color   : Colors::random(),
                });
        }

        game_state.spawner = self.spawner;
    }
}


#[derive(Clone, Copy, Debug, RustcDecodable, RustcEncodable)]
pub struct Celestial {
    pub position: Vec2<f64>,
    pub size    : f64,
}
