use std::fmt::Write;

use nalgebra::{
    cast,
    Iso3,
    Vec2,
    Vec3,
    ToHomogeneous,
    Norm,
};

use client::console::Console;
use client::graphics::frame_state::FrameState;
use client::graphics::base::color;
use client::graphics::draw::{
    GlyphDrawer,
    ShapeDrawer,
};
use client::graphics::camera::{Camera};
use client::interface::Frame;
use client::window::Window;
use shared::util::angle_of;

const SHIP_SIZE: f32 = 30.0;

pub struct Renderer {
    glyph_drawer: GlyphDrawer,
    ship_drawer : ShapeDrawer,
    line_drawer : ShapeDrawer,

    pub camera: Camera,

    line_height   : f32,
    ship_size     : f32,
    scaling_factor: f32,
}

impl Renderer {
    pub fn new(window: &Window, scaling_factor: f32) -> Renderer {
        let mut graphics = window.create_graphics();

        let font_height = 18.0 * scaling_factor;
        
        let glyph_drawer = GlyphDrawer::new(&mut graphics, font_height as u32);
        let ship_drawer  = ShapeDrawer::ship(&mut graphics);
        let line_drawer  = ShapeDrawer::line(&mut graphics);

        Renderer {
            glyph_drawer  : glyph_drawer,
            ship_drawer   : ship_drawer,
            line_drawer   : line_drawer,
            camera        : Camera::new(),
            line_height   : font_height,
            ship_size     : SHIP_SIZE * scaling_factor,
            scaling_factor: scaling_factor,
        }
    }

    pub fn render(
        &mut self,
        frame  : &Frame,
        console: &Console,
        window : &Window,
    ) {
        let mut frame_state =
            match FrameState::new(window, frame, &mut self.camera) {
                Some(frame_state) => frame_state,
                None              => return,
            };

        frame_state.graphics.clear();

        self.render_console(console, &mut frame_state);
        self.render_ships(frame, &mut frame_state);

        frame_state.graphics.flush();
    }

    fn render_console(&mut self, console: &Console, frame_state: &mut FrameState) {
        let advance_x   = self.glyph_drawer.advance_x;
        let line_height = self.line_height;

        for (y, line) in console.output.iter().enumerate() {
            self.glyph_drawer.draw(
                &line,
                position_cli(0, y, advance_x, line_height, frame_state.window_size),
                color::Colors::white(),
                false,
                frame_state.transforms.camera_to_screen,
                &mut frame_state.graphics,
            );
        }

        let mut command_line = String::new();
        let prompt_ypos = 23;

        write!(&mut command_line, "> {}", console.input)
            .unwrap_or_else(|e| panic!("Error writing to String: {}", e));


        self.glyph_drawer.draw(
            &command_line,
            position_cli(0, prompt_ypos, advance_x, line_height, frame_state.window_size),
            color::Colors::white(),
            false,
            frame_state.transforms.camera_to_screen,
            &mut frame_state.graphics,
        );

         //draw cursor position in prompt
        self.glyph_drawer.draw(
            &"_".to_string(),
            position_cli(console.prompt_index + 2, prompt_ypos, advance_x, line_height, frame_state.window_size),
            color::Colors::white(),
            false,
            frame_state.transforms.camera_to_screen,
            &mut frame_state.graphics,
        );
    }

    fn render_ships(&mut self, frame: &Frame, frame_state: &mut FrameState) {
        for (ship_id, ship) in &frame.ships {
            let pos_offset    = Vec2::new(0.7, 0.3) * self.ship_size;
            let line_advance  = Vec2::new(0.0, -self.line_height);

            let ship_velocity: Vec2<f32> = cast(ship.velocity);

            let transform = frame_state.transforms.symbol_to_screen(cast(ship.position));

            // draw ship velocity line
            let line_rotation = Iso3::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(
                    0.0,
                    0.0,
                    angle_of(ship_velocity),
                ),
            );
            self.line_drawer.draw(
                ship_velocity.norm() * self.scaling_factor * 50.0,
                color::Colors::red(),
                transform * line_rotation.to_homogeneous(),
                &mut frame_state.graphics,
            );

            if frame.select_ids.contains(ship_id) {
                self.ship_drawer.draw(
                    self.ship_size * 1.25,
                    color::Colors::white(),
                    transform,
                    &mut frame_state.graphics,
                );
            }

            let mut color = color::Colors::blue();
            if let Some(sid) = frame.ship_id {
                if *ship_id == sid  { color = color::Colors::green_spring(); }
            }

            self.ship_drawer.draw(
                self.ship_size,
                color,
                transform,
                &mut frame_state.graphics,
            );

            // draw ship id
            self.glyph_drawer.draw(
                &ship_id.to_string(),
                Vec2::new(0.0, self.ship_size * 0.6),
                color::Colors::white(),
                true,
                transform,
                &mut frame_state.graphics,
            );

            // draw ship broadcast
            if let Some(ship_comm) = frame.broadcasts.get(&ship_id) {
                self.glyph_drawer.draw(
                    ship_comm,
                    -Vec2::new(0.0, self.ship_size),
                    color::Colors::white(),
                    true,
                    transform,
                    &mut frame_state.graphics,
                );
            }

            // draw ship position
            let pos = format!("pos: ({:.2}, {:.2})", ship.position[0], ship.position[1]);
            self.glyph_drawer.draw(
                &pos,
                pos_offset,
                color::Colors::white(),
                false,
                transform,
                &mut frame_state.graphics,
            );

            // draw ship velocity
            let vel = format!("vel: ({:.2}, {:.2})", ship.velocity[0], ship.velocity[1]);
            self.glyph_drawer.draw(
                &vel,
                pos_offset + line_advance,
                color::Colors::white(),
                false,
                transform,
                &mut frame_state.graphics,
            );
        }
    }
}


/// This is used to position CLI text
/// It takes in to account the window sizing
fn position_cli(
    x          : usize,
    y          : usize,
    advance_x  : f32,
    line_height: f32,
    window_size: Vec2<f32>,
) -> Vec2<f32> {
    let pad_x = 10.0;
    let pad_y = 30.0;

    Vec2::new(
        (-1.0 * ((window_size.x / 2.0) - pad_x)) + advance_x * x as f32,
        ((window_size.y / 2.0) - pad_y) + line_height * (y as f32 * -1.0),
    )
}
