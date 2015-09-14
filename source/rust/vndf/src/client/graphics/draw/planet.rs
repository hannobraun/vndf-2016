use nalgebra::{cast,Mat4,Vec2};

use shared::game::{EntityId};

use client::graphics::base::{Graphics,color};
use client::graphics::draw::{ShapeDrawer, GlyphDrawer};
use client::graphics::transforms::Transforms;

use client::interface::Frame;


pub struct PlanetDrawer {
    scaling_factor: f32,
    symbol_drawer: ShapeDrawer,
    glyph_drawer: GlyphDrawer,
}

impl PlanetDrawer {
    pub fn new(graphics: &mut Graphics,
               font_size     : f32,
               scaling_factor: f32,)
               -> PlanetDrawer {
        PlanetDrawer {
            scaling_factor: scaling_factor,
                symbol_drawer: ShapeDrawer::planet(graphics),
            glyph_drawer: GlyphDrawer::new(graphics, font_size as u32),
        }
    }

    pub fn draw(&mut self,
                frame: &Frame,
                zoom: f32,
                transforms: &Transforms,
                graphics: &mut Graphics,) {
        for (id,planet) in &frame.planets {
            let transform = transforms.symbol_to_screen(cast(planet.body.position));
            let scale = planet.attr.size * self.scaling_factor;

            // draw selection behind planet
            if frame.select_ids.contains(id) {
                self.symbol_drawer.draw(
                    (scale/zoom) + 3.0, //this needs tweaking
                    color::Colors::white(),
                    transform,
                    graphics,
                );
            }

            // draw planet
            self.symbol_drawer.draw(
                scale/zoom,
                planet.attr.color,
                transform,
                graphics,
            );

            self.draw_name(
                *id,
                scale/zoom,
                transform,
                graphics,
            );
        }
    }

    fn draw_name(
        &mut self,
        id  : EntityId,
        size: f32,
        transform: Mat4<f32>,
        graphics : &mut Graphics,
    ) {
        let above = Vec2::new(0.0, size/2.0 + 5.0); // this needs tweaking
        let _center = Vec2::new(0.0, 0.0); // TODO: perhaps draw at center instead?

        self.glyph_drawer.draw(
            &id.to_string(),
            above,
            color::Colors::white(),
            true,
            transform,
            graphics,
            );
    }
}
