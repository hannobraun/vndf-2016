use cgmath::Matrix4;
use gfx;


pub mod renderer;

mod grid;
mod icon;
mod planet;
mod shaders {
	pub mod vertex;
}


type Graphics  = gfx::Graphics<gfx::GlDevice, gfx::GlCommandBuffer>;
type Transform = Matrix4<f32>;


#[vertex_format]
pub struct Vertex {
	vertex   : [f32, ..3],
	tex_coord: [f32, ..2],
}

impl Vertex {
	fn new(position: [f32, ..3], tex_coord: [f32, ..2]) -> Vertex {
		Vertex {
			vertex   : position,
			tex_coord: tex_coord,
		}
	}

	fn without_tex(position: [f32, ..3]) -> Vertex {
		Vertex {
			vertex   : position,
			tex_coord: [0.0, 0.0],
		}
	}
}
