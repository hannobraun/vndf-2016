use cgmath::FixedArray;
use gfx::{
	mod,
	DeviceHelper,
	Frame,
	ToSlice,
};

use super::{
	shaders,
	Graphics,
	Transform,
	Vertex,
};
use super::texture::Texture;


#[shader_param(Batch)]
struct Params {
	size     : [f32, ..3],
	transform: [[f32, ..4], ..4],
	tex      : gfx::shade::TextureParam,
}


pub struct Icon {
	pub batch  : Batch,
	pub texture: Texture,
}

impl Icon {
	pub fn new(
		graphics  : &mut Graphics,
		draw_state: &gfx::DrawState,
		texture   : Texture,
	) -> Icon {
		let vertices = [
			Vertex::new([ -0.5, -0.5, 0.0 ], [ 0.0, 1.0 ]),
			Vertex::new([  0.5, -0.5, 0.0 ], [ 1.0, 1.0 ]),
			Vertex::new([ -0.5,  0.5, 0.0 ], [ 0.0, 0.0 ]),
			Vertex::new([  0.5,  0.5, 0.0 ], [ 1.0, 0.0 ]),
		];

		let mesh  = graphics.device.create_mesh(vertices);
		let slice = mesh.to_slice(gfx::TriangleStrip);

		let program = graphics.device
			.link_program(
				shaders::vertex::SIMPLE.clone(),
				shaders::fragment::TEXTURE.clone()
			)
			.unwrap_or_else(|error| fail!("error linking program: {}", error));

		let batch = graphics
			.make_batch(
				&program,
				&mesh,
				slice,
				draw_state,
			)
			.unwrap();

		Icon {
			batch  : batch,
			texture: texture,
		}
	}

	pub fn draw(
		&self,
		graphics : &mut Graphics,
		frame    : &Frame,
		transform: &Transform,
	) {
		let params = Params {
			size     : self.texture.size.extend(0.0).into_fixed(),
			transform: transform.into_fixed(),
			tex      : self.texture.param,
		};

		graphics.draw(
			&self.batch,
			&params,
			frame
		);
	}
}
