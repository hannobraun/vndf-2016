use cgmath::{
	FixedArray,
	Matrix4,
	Vector,
	Vector2,
};
use gfx::{
	mod,
	Device,
	DeviceHelper,
	Frame,
	ToSlice,
};

use font::Glyph;
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
	pub fn from_glyph(
		graphics  : &mut Graphics,
		draw_state: &gfx::DrawState,
		glyph     : &Glyph
	) -> Icon {
		let data = Vec::from_fn(
			glyph.data.len() * 4,
			|i| {
				if (i + 1) % 4 == 0 {
					glyph.data[i / 4]
				}
				else {
					255
				}
			}
		);

		Icon::new(
			graphics,
			draw_state,
			glyph.size,
			data.as_slice(),
			false,
		)
	}

	pub fn from_image(
		graphics  : &mut Graphics,
		draw_state: &gfx::DrawState,
		image     : Image
	) -> Icon {
		Icon::new(
			graphics,
			draw_state,
			Vector2::new(image.width as f32, image.height as f32),
			image.data.as_slice(),
			true,
		)
	}

	fn new(
		graphics  : &mut Graphics,
		draw_state: &gfx::DrawState,
		size      : Vector2<f32>,
		data      : &[u8],
		center    : bool,
	) -> Icon {
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

		let texture_info = gfx::tex::TextureInfo {
			width : size.x as u16,
			height: size.y as u16,
			depth : 1,
			levels: -1,
			kind  : gfx::tex::Texture2D,
			format: gfx::tex::RGBA8,
		};

		let texture = graphics.device.create_texture(texture_info).unwrap();
		graphics.device.update_texture(
			&texture,
			&texture_info.to_image_info(),
			data
		)
		.unwrap();

		let sampler = graphics.device.create_sampler(
			gfx::tex::SamplerInfo::new(
				gfx::tex::Bilinear,
				gfx::tex::Clamp
			)
		);

		let batch = graphics
			.make_batch(
				&program,
				&mesh,
				slice,
				draw_state,
			)
			.unwrap();

		let texture = Texture {
			param: (texture, Some(sampler)),
		};

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
