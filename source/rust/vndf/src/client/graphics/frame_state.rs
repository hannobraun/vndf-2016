use nalgebra::Vec2;

use client::interface::Frame;
use client::graphics::base::Graphics;
use client::graphics::camera::Camera;
use client::graphics::transforms::Transforms;
use client::window::Window;


pub struct FrameState {
    pub graphics   : Graphics,
    pub window_size: Vec2<f32>,
    pub transforms : Transforms,
}

impl FrameState {
    pub fn new(window: &Window, frame: &Frame, camera: &mut Camera) -> Option<FrameState> {
        let window_size = {
            let size = match window.get_size().ok() {
                Some(size) => size,
                None       => return None,
            };

            if size == (0, 0) {
                //skip render frame if minimized!
                return None;
            }

            Vec2::new(size.0 as f32, size.1 as f32)
        };

        Some(FrameState {
            graphics   : window.create_graphics(),
            window_size: window_size,
            transforms : Transforms::new(frame, camera, window_size),
        })
    }
}
