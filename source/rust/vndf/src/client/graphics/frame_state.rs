use nalgebra::{
    cast,

    ToHomogeneous,

    Iso3,
    Mat4,
    Ortho3,
    Vec2,
    Vec3,
};

use client::interface::Frame;
use client::graphics::base::Graphics;
use client::graphics::camera::Camera;
use client::window::Window;


pub struct FrameState {
    pub graphics   : Graphics,
    pub window_size: Vec2<f32>,

    pub camera_to_screen: Mat4<f32>,
    pub world_to_camera : Mat4<f32>,
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

        let camera_position    = camera.update(&frame);
        let camera_translation = translation(cast(camera_position));

        let camera_zoom = Mat4::new(
            camera.zoom,         0.0,         0.0, 0.0,
                    0.0, camera.zoom,         0.0, 0.0,
                    0.0,         0.0, camera.zoom, 0.0,
                    0.0,         0.0,         0.0, 1.0,
        );

        // The following transformation matrices are named based on the
        // following nomenclature:
        // - screen space: The representation used by OpenGL. After the shaders
        //                 are done with it, point will be transformed to that
        //                 space.
        // - camera space: The coordinates from the view of the camera.
        //                 Corresponds to the pixel coordinates relative to the
        //                 window.
        // - world space:  The only space relevant, as far as the game logic is
        //                 concerned.
        let camera_to_screen = ortho(window_size);
        let world_to_camera  = camera_zoom * camera_translation;

        Some(FrameState {
            graphics   : window.create_graphics(),
            window_size: window_size,

            camera_to_screen: camera_to_screen,
            world_to_camera : world_to_camera,
        })
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
