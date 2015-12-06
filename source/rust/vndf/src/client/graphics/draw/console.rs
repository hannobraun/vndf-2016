use std::fmt::Write;

use nalgebra::Vec2;

use client::console::Console;
use client::graphics::base::Graphics;
use client::graphics::draw::GlyphDrawer;
use client::graphics::frame_state::FrameState;
use shared::color;


pub struct ConsoleDrawer {
    font_size   : f32,
    glyph_drawer: GlyphDrawer,
}

impl ConsoleDrawer {
    pub fn new(graphics: &mut Graphics, font_size: f32) -> ConsoleDrawer {
        ConsoleDrawer {
            font_size   : font_size,
            glyph_drawer: GlyphDrawer::new(graphics, font_size as u32),
        }
    }

    pub fn draw(&mut self, console: &Console, frame_state: &mut FrameState) {
        let advance_x = self.glyph_drawer.advance_x;

        for (y, line) in console.output.iter().enumerate() {
            self.glyph_drawer.draw(
                &line,
                position_cli(
                    0, y,
                    advance_x,
                    self.font_size,
                    frame_state.window_size,
                ),
                color::Colors::white(),
                false,
                frame_state.transforms.camera_to_screen,
                &mut frame_state.graphics,
            );
        }

        let mut command_line = String::new();
        let prompt_pos_y = 23;

        write!(&mut command_line, "> {}", console.input)
            .expect("Writing to String should not fail");


        self.glyph_drawer.draw(
            &command_line,
            position_cli(
                0, prompt_pos_y,
                advance_x,
                self.font_size,
                frame_state.window_size,
            ),
            color::Colors::white(),
            false,
            frame_state.transforms.camera_to_screen,
            &mut frame_state.graphics,
        );

         //draw cursor position in prompt
        self.glyph_drawer.draw(
            "_",
            position_cli(
                console.prompt_index + 2, prompt_pos_y,
                advance_x,
                self.font_size,
                frame_state.window_size,
            ),
            color::Colors::white(),
            false,
            frame_state.transforms.camera_to_screen,
            &mut frame_state.graphics,
        );
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
