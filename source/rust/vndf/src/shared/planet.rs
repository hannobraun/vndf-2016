use rand::{thread_rng, sample};
use nalgebra::Vec2;

use shared::game::{Body};
use client::graphics::base::{Color,Colors};

#[derive(Clone, Debug, RustcDecodable, RustcEncodable, PartialEq)]
pub struct Planet {
    pub body: Body,
    pub color: Color,
    pub size: f32,
}

impl Planet {
    pub fn new (pos: Vec2<f64>, size: f32, mass: f32, color: Color) -> Planet {
        Planet { body: Body { position: pos,
                              velocity: Vec2::new(0.0,0.0),
                              mass: mass, },
                 color: color,
                 size: size, }
    }
}

pub struct PlanetBuilder {
    planet: Planet,
}

impl PlanetBuilder {
    pub fn default () -> PlanetBuilder {
        // random placement and size
        let mut rng = thread_rng();
        let a = sample(&mut rng, 1..1000, 4);
        
        PlanetBuilder { planet: Planet::new(Vec2::new(a[0] as f64,a[1] as f64),
                                            a[2] as f32,
                                            a[3] as f32,
                                            Colors::gold()) }
    }

    pub fn position (mut self, pos: Vec2<f64>) -> PlanetBuilder {
        self.planet.body.position = pos;
        self
    }
    pub fn size (mut self, size: f32) -> PlanetBuilder {
        self.planet.size = size;
        self
    }
    pub fn mass (mut self, mass: f32) -> PlanetBuilder {
        self.planet.body.mass = mass;
        self
    }
    pub fn color (mut self, color: Color) -> PlanetBuilder {
        self.planet.color = color;
        self
    }

    pub fn build (self) -> Planet {
        self.planet
    }
}
