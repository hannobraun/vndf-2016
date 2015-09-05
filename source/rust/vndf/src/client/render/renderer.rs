use std::fmt::Write;

use nalgebra::{
    cast,
    Mat4,
    Ortho3,
    Iso3,
    Vec2,
    Vec3,
    ToHomogeneous,
    Norm,
};

use client::console::Console;
use client::interface::Frame;
use client::window::Window;
use client::render::base::{
    color,
    Graphics,
};
use client::render::draw::{
    GlyphDrawer,
    ShipDrawer,
};
use client::render::camera::{Camera};
use shared::util::angle_of;

const SHIP_SIZE: f32 = 30.0;

pub struct Renderer {
    glyph_drawer  : GlyphDrawer,
    triangle      : ShipDrawer,
    line          : ShipDrawer,
    pub camera    : Camera,
    line_height   : f32,
    scaling_factor: f32,
}

impl Renderer {
    pub fn new(window: &Window, scaling_factor: f32) -> Renderer {
        let mut graphics = window.create_graphics();

        let font_height = 18.0 * scaling_factor;
        
        let glyph_drawer = GlyphDrawer::new(&mut graphics, font_height as u32);
        let triangle     = ShipDrawer::triangle(&mut graphics);
        let line         = ShipDrawer::line(&mut graphics);

        Renderer {
            glyph_drawer  : glyph_drawer,
            triangle      : triangle,
            line          : line,
            camera        : Camera::new(),
            line_height   : font_height,
            scaling_factor: scaling_factor,
        }
    }

    pub fn render(
        &mut self,
        frame  : &Frame,
        console: &Console,
        window : &Window,
    ) {
        let mut graphics = window.create_graphics();

        let window_size = {
            let size = window.get_size();
            Vec2::new(size.0 as f32, size.1 as f32)
        };

        let camera_position    = self.camera.update(&frame);
        let camera_translation = translation(cast(camera_position));

        let camera_to_screen = ortho(window_size);
        let world_to_pixel   = camera_to_screen * camera_translation;

        let world_to_camera = ortho(window_size * self.camera.zoom) * camera_translation;

        let scale_factor = self.scaling_factor * (self.camera.zoom);

        graphics.clear();
        
        self.render_console(console, window_size, camera_to_screen, &mut graphics);
        self.render_selections(frame, world_to_camera, scale_factor, &mut graphics);
        self.render_ships(frame, scale_factor, world_to_camera, world_to_pixel, &mut graphics);

        graphics.flush();
    }

    fn render_console(&mut self, console: &Console, window_size: Vec2<f32>, transform: Mat4<f32>, graphics: &mut Graphics) {
        let advance_x   = self.glyph_drawer.advance_x;
        let line_height = self.line_height;

        for (y, line) in console.output.iter().enumerate() {
            self.glyph_drawer.draw(
                &line,
                position_cli(0, y, advance_x, line_height, window_size),
                color::Colors::white(),
                false,
                transform,
                graphics,
            );
        }

        let mut command_line = String::new();
        let prompt_ypos = 23;

        write!(&mut command_line, "> {}", console.input)
            .unwrap_or_else(|e| panic!("Error writing to String: {}", e));


        self.glyph_drawer.draw(
            &command_line,
            position_cli(0, prompt_ypos, advance_x, line_height, window_size),
            color::Colors::white(),
            false,
            transform,
            graphics,
        );

         //draw cursor position in prompt
        self.glyph_drawer.draw(
            &"_".to_string(),
            position_cli(console.prompt_index + 2, prompt_ypos, advance_x, line_height, window_size),
            color::Colors::white(),
            false,
            transform,
            graphics,
        );
    }

    fn render_selections(&mut self, frame: &Frame, world_trans: Mat4<f32>, scale_factor: f32, graphics: &mut Graphics) {
        for id in frame.select_ids.iter() {
            if let Some(ship) = frame.ships.get(&id) {
                let position = ship.position + Vec2::new(0.0, 2.0 * self.camera.zoom as f64);

                let translation = Iso3::new(
                    Vec3::new(position.x as f32, position.y as f32, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                );
                let transform = world_trans * translation.to_homogeneous();

                self.triangle.draw(
                    scale_factor * SHIP_SIZE * 1.25,
                    color::Colors::white(),
                    transform,
                    graphics,
                );
            }
        }
    }

    fn render_ships(&mut self, frame: &Frame, scale_factor: f32, world_trans: Mat4<f32>, screen_trans: Mat4<f32>, graphics: &mut Graphics) {
        for (ship_id, ship) in &frame.ships {
            let pos_offset    = Vec2::new(SHIP_SIZE, 10.0);
            let line_advance  = Vec2::new(0.0, -self.line_height);

            let ship_position: Vec2<f32> = cast(ship.position);
            let ship_velocity: Vec2<f32> = cast(ship.velocity);

            // draw ship velocity line
            let transform = Iso3::new(
                Vec3::new(ship.position.x as f32, ship.position.y as f32, 0.0),
                Vec3::new(
                    0.0,
                    0.0,
                    angle_of(ship_velocity),
                ),
            );
            self.line.draw(
                scale_factor * ship_velocity.norm() * 50.0,
                color::Colors::red(),
                world_trans * transform.to_homogeneous(),
                graphics,
            );

            let mut color = color::Colors::blue();
            if let Some(sid) = frame.ship_id {
                if *ship_id == sid  { color = color::Colors::green_spring(); }
            }

            let translation = Iso3::new(
                Vec3::new(ship_position.x, ship_position.y, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
            );
            let transform = world_trans * translation.to_homogeneous();

            self.triangle.draw(
                scale_factor * SHIP_SIZE,
                color,
                transform,
                graphics,
            );

            // draw ship id
            self.glyph_drawer.draw(
                &ship_id.to_string(),
                ship_position - line_advance + Vec2::new(0.0,5.0),
                color::Colors::white(),
                true,
                screen_trans,
                graphics,
            );

            // draw ship broadcast
            if let Some(ship_comm) = frame.broadcasts.get(&ship_id) {
                self.glyph_drawer.draw(
                    ship_comm,
                    ship_position + line_advance - Vec2::new(0.0, SHIP_SIZE),
                    color::Colors::white(),
                    true,
                    screen_trans,
                    graphics,
                );
            }

            // draw ship position
            let pos = format!("pos: ({:.2}, {:.2})", ship.position[0], ship.position[1]);
            self.glyph_drawer.draw(
                &pos,
                ship_position + pos_offset,
                color::Colors::white(),
                false,
                screen_trans,
                graphics,
            );

            // draw ship velocity
            let vel = format!("vel: ({:.2}, {:.2})", ship.velocity[0], ship.velocity[1]);
            self.glyph_drawer.draw(
                &vel,
                ship_position + pos_offset + line_advance,
                color::Colors::white(),
                false,
                screen_trans,
                graphics,
            );
        }
    }
}


/// get new ortho transform matrix based on window size specified
fn ortho(size: Vec2<f32>) -> Mat4<f32> {
    let ortho = Ortho3::new(
        size.x, size.y,
        -1.0, 1.0
    );

    ortho.to_mat()
}

fn translation(v: Vec2<f32>) -> Mat4<f32> {
    let translation = Iso3::new(
        Vec3::new(v.x, v.y, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

    translation.to_homogeneous()
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
