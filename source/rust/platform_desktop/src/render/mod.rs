use cgmath::Matrix4;
use gfx;


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
	fn for_grid(position: [f32, ..3]) -> Vertex {
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
