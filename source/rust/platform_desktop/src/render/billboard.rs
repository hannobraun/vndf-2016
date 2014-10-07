use cgmath::{
	FixedArray,
	Vector2,
	Vector3,
};
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
	position   : [f32, ..3],
	transform  : [[f32, ..4], ..4],
	size       : [f32, ..2],
	screen_size: [f32, ..2],
	tex        : gfx::shade::TextureParam,
}


pub struct Billboard {
	pub batch  : Batch,
	pub texture: Texture,
	pub size   : Vector2<f32>,
}

impl Billboard {
	pub fn new(
		graphics  : &mut Graphics,
		draw_state: &gfx::DrawState,
		texture   : Texture,
	) -> Billboard {
		let size = texture.size;

		let vertices = [
			Vertex::new([ -1.0, -1.0, 0.0 ], [ 0.0, 1.0 ]),
			Vertex::new([  1.0, -1.0, 0.0 ], [ 1.0, 1.0 ]),
			Vertex::new([ -1.0,  1.0, 0.0 ], [ 0.0, 0.0 ]),
			Vertex::new([  1.0,  1.0, 0.0 ], [ 1.0, 0.0 ]),
		];

		let mesh  = graphics.device.create_mesh(vertices);
		let slice = mesh.to_slice(gfx::TriangleStrip);

		let program = graphics.device
			.link_program(
				shaders::vertex::FIXED_SIZE_BILLBOARD.clone(),
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

		Billboard {
			batch  : batch,
			texture: texture,
			size   : size,
		}
	}

	pub fn draw(
		&self,
		graphics   : &mut Graphics,
		frame      : &Frame,
		position   : &Vector3<f32>,
		texture    : &Texture,
		transform  : &Transform,
		screen_size: &Vector2<f32>,
	) {
		let params = Params {
			position   : position.into_fixed(),
			transform  : transform.into_fixed(),
			size       : texture.size.into_fixed(),
			screen_size: screen_size.into_fixed(),
			tex        : texture.param,
		};

		graphics.draw(
			&self.batch,
			&params,
			frame
		);
	}
}
