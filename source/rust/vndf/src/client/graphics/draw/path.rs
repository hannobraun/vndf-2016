use nalgebra::{
    cast,
    Iso3,
    Norm,
    ToHomogeneous,
    Vec3,
};

use client::graphics::base::Graphics;
use client::graphics::draw::ShapeDrawer;
use client::graphics::transforms::Transforms;
use client::interface::Frame;
use shared::color::Colors;
use shared::util::angle_of;


pub struct PathDrawer {
    scaling_factor: f32,
    line_drawer   : ShapeDrawer,
}

impl PathDrawer {
    pub fn new(graphics: &mut Graphics, scaling_factor: f32) -> Self {
        PathDrawer {
            line_drawer   : ShapeDrawer::line(graphics),
            scaling_factor: scaling_factor,
        }
    }

    pub fn draw(
        &mut self,
        frame     : &Frame,
        transforms: &Transforms,
        graphics  : &mut Graphics,
    ) {
        let ship = match frame.ship_id {
            Some(id) => frame.ships[&id],
            None     => return,
        };

        let transform = transforms.symbol_to_screen(cast(ship.position));

        let line_rotation = Iso3::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(
                0.0,
                0.0,
                angle_of(cast(ship.velocity)),
            ),
        );

        self.line_drawer.draw(
            ship.velocity.norm() as f32 * self.scaling_factor * 50.0,
            Colors::red(),
            transform * line_rotation.to_homogeneous(),
            graphics,
        );
    }
}
