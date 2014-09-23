use cgmath::{
	Matrix4,
	Point3,
	Vector3,
};
use gfx;

use platform::Camera;


pub mod renderer;

mod grid;
mod icon;


type Graphics  = gfx::Graphics<gfx::GlDevice, gfx::GlCommandBuffer>;
type Transform = Matrix4<f32>;


#[vertex_format]
pub struct Vertex {
	position : [f32, ..3],
	tex_coord: [f32, ..2],
}

impl Vertex {
	fn without_tex(position: [f32, ..3]) -> Vertex {
		Vertex {
			position : position,
			tex_coord: [0.0, 0.0],
		}
	}

	fn for_icon(position: [f32, ..3], tex_coord: [f32, ..2]) -> Vertex {
		Vertex {
			position : position,
			tex_coord: tex_coord,
		}
	}
}


fn camera_to_transform(camera: &Camera) -> Transform {
	let (phi, theta) = camera.perspective;

	let x = camera.distance * theta.s.sin() * phi.s.cos();
	let y = camera.distance * theta.s.sin() * phi.s.sin();
	let z = camera.distance * theta.s.cos();

	Matrix4::look_at(
		&Point3::new(
			(camera.center[0] + x) as f32,
			(camera.center[1] + y) as f32,
			z as f32,
		),
		&Point3::new(
			camera.center[0] as f32,
			camera.center[1] as f32,
			camera.center[2] as f32,
		),
		&Vector3::new(0.0, 0.0, 1.0),
	)
}
