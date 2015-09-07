use client::console::Console;
use client::graphics::frame_state::FrameState;
use client::graphics::draw::{
    ConsoleDrawer,
    GlyphDrawer,
    ShapeDrawer,
    ShipDrawer,
};
use client::graphics::camera::{Camera};
use client::interface::Frame;
use client::window::Window;

const SHIP_SIZE: f32 = 30.0;

pub struct Renderer {
    glyph_drawer  : GlyphDrawer,
    line_drawer   : ShapeDrawer,
    console_drawer: ConsoleDrawer,
    ship_drawer   : ShipDrawer,

    pub camera: Camera,

    line_height   : f32,
    ship_size     : f32,
    scaling_factor: f32,
}

impl Renderer {
    pub fn new(window: &Window, scaling_factor: f32) -> Renderer {
        let mut graphics = window.create_graphics();

        let font_height = 18.0 * scaling_factor;
        let ship_size   = SHIP_SIZE * scaling_factor;
        
        let glyph_drawer   = GlyphDrawer::new(&mut graphics, font_height as u32);
        let ship_drawer    = ShapeDrawer::ship(&mut graphics);
        let line_drawer    = ShapeDrawer::line(&mut graphics);
        let console_drawer = ConsoleDrawer::new(&mut graphics, font_height);

        let ship_drawer = ShipDrawer::new(
            &mut graphics,
            ship_size,
            font_height,
            scaling_factor,
        );

        Renderer {
            glyph_drawer  : glyph_drawer,
            ship_drawer   : ship_drawer,
            line_drawer   : line_drawer,
            console_drawer: console_drawer,
            camera        : Camera::new(),
            line_height   : font_height,
            ship_size     : ship_size,
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

        self.console_drawer.draw(console, &mut frame_state);
        self.ship_drawer.draw(frame, &mut frame_state);

        frame_state.graphics.flush();
    }
}
