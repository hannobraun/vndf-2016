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
use shared::game::logic::{
    apply_gravity,
    integrate,
};
use shared::util::angle_of;


pub struct PathDrawer {
    line_drawer: ShapeDrawer,
}

impl PathDrawer {
    pub fn new(graphics: &mut Graphics) -> Self {
        PathDrawer {
            line_drawer: ShapeDrawer::line(graphics),
        }
    }

    pub fn draw(
        &mut self,
        frame     : &Frame,
        transforms: &Transforms,
        graphics  : &mut Graphics,
    ) {
        let mut ship = match frame.ship_id {
            Some(id) => frame.ships[&id],
            None     => return,
        };

        let mut previous_position = ship.position;
        for _ in 0 .. 500 {
            for (_, planet) in &frame.planets {
                apply_gravity(planet, &mut ship);
            }
            integrate(&mut ship, 5.0);

            let movement = previous_position - ship.position;
            previous_position = ship.position;

            let transform: Iso3<f32> = Iso3::new(
                Vec3::new(
                    ship.position.x as f32,
                    ship.position.y as f32,
                    0.0
                ),
                Vec3::new(
                    0.0,
                    0.0,
                    angle_of(cast(movement)),
                ),
            );

            self.line_drawer.draw(
                movement.norm() as f32,
                Colors::white(),
                transforms.camera_to_screen
                    * transforms.world_to_camera
                    * transform.to_homogeneous(),
                graphics,
            );
        }
    }
}
