use nalgebra::{
    cast,

    ToHomogeneous,
    Inv,
    
    Iso3,
    Mat4,
    Ortho3,
    Vec2,
    Vec3,
    Vec4,
};

use client::graphics::camera::Camera;
use client::interface::Frame;


pub struct Transforms {
    pub camera_to_screen: Mat4<f32>,
    pub world_to_camera : Mat4<f32>,
}

impl Transforms {
    pub fn new(frame: &Frame, camera: &mut Camera, window_size: Vec2<f32>) -> Transforms {
        let camera_position    = camera.update(frame);
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
	
	let world_to_camera = {
	    if let Some(cam_zoom) = camera_zoom.inv() {
		cam_zoom * camera_translation
	    } // maybe we should warn! or panic! otherwise?
	    else { camera_translation }}; // probably not necessary

	
        Transforms {
            camera_to_screen: camera_to_screen,
            world_to_camera : world_to_camera,
        }
    }

    pub fn symbol_to_screen(&self, world_position: Vec2<f32>) -> Mat4<f32> {
        let world_position = Vec4::new(
            world_position.x,
            world_position.y,
            0.0,
            1.0,
        );

        let camera_position = self.world_to_camera * world_position;

        let camera_translation =
            Iso3::new(
                Vec3::new(camera_position.x, camera_position.y, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
            )
            .to_homogeneous();

        self.camera_to_screen * camera_translation
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
