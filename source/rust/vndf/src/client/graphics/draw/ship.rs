use nalgebra::{
    cast,

    Norm,
    ToHomogeneous,

    Iso3,
    Mat4,
    Vec2,
    Vec3,
};

use client::graphics::base::{
    color,

    Graphics,
};
use client::graphics::draw::{
    GlyphDrawer,
    ShapeDrawer,
};
use client::graphics::transforms::Transforms;
use client::interface::Frame;
use shared::game::{
    Body,
    EntityId,
};
use shared::util::angle_of;


pub struct ShipDrawer {
    ship_size     : f32,
    line_height   : f32,
    scaling_factor: f32,

    symbol_drawer: ShapeDrawer,
    glyph_drawer : GlyphDrawer,
    line_drawer  : ShapeDrawer,
}

impl ShipDrawer {
    pub fn new(
        graphics      : &mut Graphics,
        ship_size     : f32,
        font_size     : f32,
        scaling_factor: f32,
    ) -> ShipDrawer {
        ShipDrawer {
            ship_size     : ship_size,
            line_height   : font_size,
            scaling_factor: scaling_factor,

            symbol_drawer: ShapeDrawer::ship(graphics),
            glyph_drawer : GlyphDrawer::new(graphics, font_size as u32),
            line_drawer  : ShapeDrawer::line(graphics),
        }
    }

    pub fn draw(
        &mut self,
        frame     : &Frame,
        transforms: &Transforms,
        graphics  : &mut Graphics,
    ) {
        for (ship_id, ship) in &frame.ships {
            let transform = transforms.symbol_to_screen(cast(ship.position));

            self.draw_velocity_line(
                cast(ship.velocity),
                transform,
                graphics,
            );

            if frame.select_ids.contains(ship_id) {
                self.draw_selection(
                    transform,
                    graphics,
                );
            }

            self.draw_symbol(
                frame,
                *ship_id,
                transform,
                graphics,
            );

            self.draw_name(
                *ship_id,
                transform,
                graphics,
            );

            if let Some(broadcast) = frame.broadcasts.get(&ship_id) {
                self.draw_broadcast(
                    broadcast,
                    transform,
                    graphics,
                );
            }

            self.draw_info(
                ship,
                transform,
                graphics,
            );
        }
    }

    fn draw_velocity_line(
        &mut self,
        velocity : Vec2<f32>,
        transform: Mat4<f32>,
        graphics : &mut Graphics,
    ) {
        // draw ship velocity line
        let line_rotation = Iso3::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(
                0.0,
                0.0,
                angle_of(velocity),
            ),
        );
        self.line_drawer.draw(
            velocity.norm() * self.scaling_factor * 50.0,
            color::Colors::red(),
            transform * line_rotation.to_homogeneous(),
            graphics,
        );
    }

    fn draw_selection(
        &mut self,
        transform: Mat4<f32>,
        graphics : &mut Graphics,
    ) {
        self.symbol_drawer.draw(
            self.ship_size * 1.25,
            color::Colors::white(),
            transform,
            graphics,
        );
    }

    fn draw_symbol(
        &mut self,
        frame    : &Frame,
        ship_id  : EntityId,
        transform: Mat4<f32>,
        graphics : &mut Graphics,
    ) {
        let mut color = color::Colors::blue();
        if let Some(sid) = frame.ship_id {
            if ship_id == sid  { color = color::Colors::green_spring(); }
        }

        self.symbol_drawer.draw(
            self.ship_size,
            color,
            transform,
            graphics,
        );
    }

    fn draw_name(
        &mut self,
        ship_id  : EntityId,
        transform: Mat4<f32>,
        graphics : &mut Graphics,
    ) {
        // draw ship id
        self.glyph_drawer.draw(
            &ship_id.to_string(),
            Vec2::new(0.0, self.ship_size * 0.6),
            color::Colors::white(),
            true,
            transform,
            graphics,
        );
    }

    fn draw_broadcast(
        &mut self,
        broadcast: &str,
        transform: Mat4<f32>,
        graphics : &mut Graphics,
    ) {
        self.glyph_drawer.draw(
            broadcast,
            -Vec2::new(0.0, self.ship_size),
            color::Colors::white(),
            true,
            transform,
            graphics,
        );
    }

    fn draw_info(
        &mut self,
        ship     : &Body,
        transform: Mat4<f32>,
        graphics : &mut Graphics,
    ) {
        let offset      = Vec2::new(0.7, 0.3) * self.ship_size;
        let line_offset = Vec2::new(0.0, -self.line_height);

        let position = format!("pos: ({:.2}, {:.2})",
            ship.position.x,
            ship.position.y,
        );
        let velocity = format!("vel: ({:.2}, {:.2})",
            ship.velocity.x,
            ship.velocity.y,
        );

        let mut advance = Vec2::new(0.0, 0.0);

        let mut draw = |text| {
            self.glyph_drawer.draw(
                text,
                offset + advance,
                color::Colors::white(),
                false,
                transform,
                graphics,
            );

            advance = advance + line_offset;
        };

        draw(&position);
        draw(&velocity);
    }
}
