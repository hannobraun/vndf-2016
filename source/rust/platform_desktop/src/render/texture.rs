use cgmath::Vector2;
use gfx::{
	mod,
	Device,
};

use super::Graphics;


pub struct Texture {
	pub param: gfx::shade::TextureParam,
}

impl Texture {
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
			param: (texture, Some(sampler)),
		}
	}
}
