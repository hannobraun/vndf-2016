use cgmath::{
	FixedArray,
	Matrix4,
	Vector,
	Vector2,
};
use gfx::{
	mod,
	DeviceHelper,
	Frame,
	ToSlice,
};

use images::Image;

use super::{
	shaders,
	Graphics,
	Transform,
	Vertex,
};
use super::texture::Texture;


#[shader_param(Batch)]
struct Params {
	transform: [[f32, ..4], ..4],
	tex      : gfx::shade::TextureParam,
}


pub struct Icon {
	pub batch  : Batch,
	pub texture: Texture,
	pub size   : Vector2<f32>,
	pub offset : Vector2<f32>,
}

impl Icon {
	pub fn from_image(
		graphics  : &mut Graphics,
		draw_state: &gfx::DrawState,
		image     : &Image
	) -> Icon {
		let size    = Vector2::new(image.width as f32, image.height as f32);
		let texture = Texture::new(image.data.as_slice(), size, graphics);

		Icon::new(
			graphics,
			draw_state,
			texture,
			true,
		)
	}

	pub fn new(
		graphics  : &mut Graphics,
		draw_state: &gfx::DrawState,
		texture   : Texture,
		center    : bool,
	) -> Icon {
		let size = texture.size;

		let vertices = [
			Vertex::new([ -0.5 * size.x, -0.5 * size.y, 0.0 ], [ 0.0, 1.0 ]),
			Vertex::new([  0.5 * size.x, -0.5 * size.y, 0.0 ], [ 1.0, 1.0 ]),
			Vertex::new([ -0.5 * size.x,  0.5 * size.y, 0.0 ], [ 0.0, 0.0 ]),
			Vertex::new([  0.5 * size.x,  0.5 * size.y, 0.0 ], [ 1.0, 0.0 ]),
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

		let offset = if center { Vector2::zero() } else { size.mul_s(0.5) };

		Icon {
			batch  : batch,
			texture: texture,
			size   : size,
			offset : offset,
		}
	}

	pub fn draw(
		&self,
		graphics : &mut Graphics,
		frame    : &Frame,
		transform: &Transform,
	) {
		let params = Params {
			transform: transform.mul(&Matrix4::from_translation(&self.offset.extend(0.0))).into_fixed(),
			tex      : self.texture.param,
		};

		graphics.draw(
			&self.batch,
			&params,
			frame
		);
	}
}
