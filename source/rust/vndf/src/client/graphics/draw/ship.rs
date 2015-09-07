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
use client::graphics::frame_state::FrameState;
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

    pub fn draw(&mut self, frame: &Frame, frame_state: &mut FrameState) {
        for (ship_id, ship) in &frame.ships {
            let transform = frame_state.transforms.symbol_to_screen(cast(ship.position));

            self.draw_velocity_line(
                &mut frame_state.graphics,
                cast(ship.velocity),
                transform,
            );

            if frame.select_ids.contains(ship_id) {
                self.draw_selection(
                    &mut frame_state.graphics,
                    transform,
                );
            }

            self.draw_symbol(
                &mut frame_state.graphics,
                transform,
                frame,
                *ship_id,
            );

            self.draw_name(
                &mut frame_state.graphics,
                transform,
                *ship_id,
            );

            if let Some(broadcast) = frame.broadcasts.get(&ship_id) {
                self.draw_broadcast(
                    &mut frame_state.graphics,
                    transform,
                    broadcast,
                );
            }

            self.draw_info(
                &mut frame_state.graphics,
                transform,
                ship,
            );
        }
    }

    pub fn draw_velocity_line(
        &mut self,
        graphics : &mut Graphics,
        velocity : Vec2<f32>,
        transform: Mat4<f32>
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

    pub fn draw_selection(
        &mut self,
        graphics : &mut Graphics,
        transform: Mat4<f32>,
    ) {
        self.symbol_drawer.draw(
            self.ship_size * 1.25,
            color::Colors::white(),
            transform,
            graphics,
        );
    }

    pub fn draw_symbol(
        &mut self,
        graphics : &mut Graphics,
        transform: Mat4<f32>,
        frame    : &Frame,
        ship_id  : EntityId,
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

    pub fn draw_name(
        &mut self,
        graphics : &mut Graphics,
        transform: Mat4<f32>,
        ship_id  : EntityId,
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

    pub fn draw_broadcast(
        &mut self,
        graphics : &mut Graphics,
        transform: Mat4<f32>,
        broadcast: &str,
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

    pub fn draw_info(
        &mut self,
        graphics : &mut Graphics,
        transform: Mat4<f32>,
        ship     : &Body,
    ) {
        let offset       = Vec2::new(0.7, 0.3) * self.ship_size;
        let line_advance = Vec2::new(0.0, -self.line_height);

        let position = format!("pos: ({:.2}, {:.2})",
            ship.position.x,
            ship.position.y,
        );
        let velocity = format!("vel: ({:.2}, {:.2})",
            ship.velocity.x,
            ship.velocity.y,
        );

        // draw ship position
        self.glyph_drawer.draw(
            &position,
            offset,
            color::Colors::white(),
            false,
            transform,
            graphics,
        );

        // draw ship velocity
        self.glyph_drawer.draw(
            &velocity,
            offset + line_advance,
            color::Colors::white(),
            false,
            transform,
            graphics,
        );
    }
}
