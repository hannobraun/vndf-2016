use nalgebra::{cast};

use client::graphics::base::{Graphics};
use client::graphics::draw::{ShapeDrawer};
use client::graphics::transforms::Transforms;

use client::interface::Frame;


pub struct PlanetDrawer {
    scaling_factor: f32,
    symbol_drawer: ShapeDrawer,
}

impl PlanetDrawer {
    pub fn new(graphics: &mut Graphics,
               scaling_factor: f32,)
               -> PlanetDrawer {
        PlanetDrawer { scaling_factor: scaling_factor,
                       symbol_drawer: ShapeDrawer::planet(graphics), }
    }

    pub fn draw(&mut self,
                frame: &Frame,
                transforms: &Transforms,
                graphics: &mut Graphics,) {
        for (_id,planet) in &frame.planets {
            let transform = transforms.symbol_to_screen(cast(planet.body.position));
            self.symbol_drawer.draw(
                planet.attr.size * self.scaling_factor,
                planet.attr.color,
                transform,
                graphics,
                );
        }
    }
    
}
