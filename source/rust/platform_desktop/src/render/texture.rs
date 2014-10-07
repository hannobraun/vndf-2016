use cgmath::Vector2;
use gfx::{
	mod,
	Device,
};

use font::Glyph;
use images::Image;

use super::Graphics;


#[deriving(Clone)]
pub struct Texture {
	pub size : Vector2<f32>,
	pub param: gfx::shade::TextureParam,
}

impl Texture {
	pub fn from_glyph(glyph: &Glyph, graphics: &mut Graphics) -> Texture {
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

		Texture::new(data.as_slice(), glyph.size, graphics)
	}

	pub fn from_image(image: &Image, graphics: &mut Graphics) -> Texture {
		Texture::new(
			image.data.as_slice(),
			Vector2::new(image.width as f32, image.height as f32),
			graphics
		)
	}

	pub fn new(
		data    : &[u8],
		size    : Vector2<f32>,
		graphics: &mut Graphics,
	) -> Texture {
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

		Texture {
			size : size,
			param: (texture, Some(sampler)),
		}
	}
}
