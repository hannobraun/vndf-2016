use client::console::Console;
use client::graphics::SHIP_SIZE;
use client::graphics::frame_state::FrameState;
use client::graphics::draw::{
    ConsoleDrawer,
    ShipDrawer,
    PlanetDrawer,
    GridDrawer,
    PathDrawer,
};
use client::graphics::camera::{Camera};
use client::interface::Frame;
use client::window::Window;


pub struct Renderer {
    console_drawer: ConsoleDrawer,
    ship_drawer   : ShipDrawer,
    planet_drawer : PlanetDrawer,
    grid_drawer   : GridDrawer,
    path_drawer   : PathDrawer,
    
    pub camera: Camera,
}

impl Renderer {
    pub fn new(window: &Window, scaling_factor: f32) -> Renderer {
        let mut graphics = window.create_graphics();

        let font_height = 18.0 * scaling_factor;
        let ship_size   = SHIP_SIZE * scaling_factor;
        
        let console_drawer = ConsoleDrawer::new(&mut graphics, font_height);
        let ship_drawer = ShipDrawer::new(
            &mut graphics,
            ship_size,
            font_height,
        );

        let planet_drawer = PlanetDrawer::new(
            &mut graphics,
            font_height,
            );

        let grid_drawer =  GridDrawer::new(
            &mut graphics,
            scaling_factor,
            );

        let path_drawer = PathDrawer::new(&mut graphics, scaling_factor);

        Renderer {
            console_drawer: console_drawer,
            ship_drawer   : ship_drawer,
            planet_drawer : planet_drawer,
            grid_drawer   : grid_drawer,
            path_drawer   : path_drawer,

            camera: Camera::new(),
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

        self.grid_drawer.draw(
            self.camera.zoom,
            &frame_state.window_size,
            &frame_state.transforms,
            &mut frame_state.graphics,
        );

        self.path_drawer.draw(
            frame,
            &frame_state.transforms,
            &mut frame_state.graphics,
        );

        self.planet_drawer.draw(frame,
            self.camera.zoom,
            &frame_state.transforms,
            &mut frame_state.graphics,
        );

        self.ship_drawer.draw(
            frame,
            &frame_state.transforms,
            &mut frame_state.graphics,
        );

        self.console_drawer.draw(console, &mut frame_state);

        frame_state.graphics.flush();
    }
}
