use rand::{
    thread_rng,
    sample,
    random,
};
use nalgebra::Vec2;

use shared::color::{
    Color,
    Colors,
};
use shared::game::Body;


pub const MAX_SIZE: i32 = 1000;
pub const MIN_SIZE: i32 = 100;

pub const MAX_MASS: i32 = 1000;
pub const MIN_MASS: i32 = 100;


#[derive(Clone, Copy, Debug, RustcDecodable, RustcEncodable, PartialEq)]
pub struct Planet {
    pub body: Body,
    pub attr: PlanetAttr,
}

#[derive(Clone, Copy, Debug, RustcDecodable, RustcEncodable, PartialEq)]
pub struct PlanetAttr {
    pub color: Color,
    pub size: f32,
}

impl Planet {
    pub fn new (pos: Vec2<f64>, size: f32, mass: f32, color: Color) -> Planet {
        let size = { if size < MIN_SIZE as f32 { MIN_SIZE as f32 }
                     else if size > MAX_SIZE as f32 { MAX_SIZE as f32 }
                     else { size }};
        let mass = { if mass < MIN_MASS as f32 { MIN_MASS as f32 }
                     else if mass > MAX_MASS as f32 { MAX_MASS as f32 }
                     else { mass }};
        
        Planet { body: Body { position: pos,
                              velocity: Vec2::new(0.0,0.0),
                              mass: mass, },
                 attr: PlanetAttr { color: color,
				    size: size, } }
    }
}

pub struct PlanetBuilder {
    planet: Planet,
}

impl PlanetBuilder {
    pub fn default () -> PlanetBuilder {
        // random placement and size
        let mut rng = thread_rng();
        let pos = sample(&mut rng, 1..10000, 2);
        let size = sample(&mut rng, MIN_SIZE..MAX_SIZE, 1);
        let mass = sample(&mut rng, MIN_MASS..MAX_MASS, 1);

	let x = pos[0] * (if random::<bool>() { 1 }
			else { -1 } );
	let y = pos[1] * (if random::<bool>() { 1 }
			else { -1 } );
	
        PlanetBuilder { planet: Planet::new(Vec2::new((x*2) as f64,y as f64),
                                            size[0] as f32,
                                            mass[0] as f32,
                                            Colors::random()) }
    }

    pub fn position (mut self, pos: Vec2<f64>) -> PlanetBuilder {
        self.planet.body.position = pos;
        self
    }
    pub fn size (mut self, size: f32) -> PlanetBuilder {
        self.planet.attr.size = size;
        self
    }
    pub fn mass (mut self, mass: f32) -> PlanetBuilder {
        self.planet.body.mass = mass;
        self
    }
    pub fn color (mut self, color: Color) -> PlanetBuilder {
        self.planet.attr.color = color;
        self
    }

    pub fn build (self) -> Planet {
        self.planet
    }
}
