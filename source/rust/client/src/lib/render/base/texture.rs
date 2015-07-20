use gfx;
use gfx::traits::*;
use gfx_device_gl as gl;

use font::Glyph;


pub type TextureHandle = gfx::handle::Texture<gl::Resources>;
pub type SamplerHandle = gfx::handle::Sampler<gl::Resources>;


pub struct Texture {
	pub texture: TextureHandle,
	pub sampler: SamplerHandle,
}

impl Texture {
	pub fn from_glyph(glyph: &Glyph, factory: &mut gl::Factory) -> Option<Texture> {
		let width  = glyph.size.x as u16;
		let height = glyph.size.y as u16;

		if width == 0 || height == 0 {
			return None;
		}

		let data: Vec<u8> = (0..glyph.data.len() * 4)
			.map(|i|
				if (i + 1) % 4 == 0 {
					glyph.data[i / 4]
				}
				else {
					255
				}
			)
			.collect();

		let texture_info = gfx::tex::TextureInfo {
			width : width,
			height: height,
			depth : 1,
			levels: 1,
			kind  : gfx::tex::Kind::D2,
			format: gfx::tex::RGBA8,
		};
		let image_info = texture_info.into();

		let texture = factory
			.create_texture(texture_info)
			.unwrap_or_else(|e| panic!("Error creating texture: {:?}", e));
		factory
			.update_texture(
				&texture,
				&image_info,
				data.as_ref(),
				None,
			)
			.unwrap_or_else(|e| panic!("Error updating texture: {:?}", e));

		let sampler = factory.create_sampler(
			gfx::tex::SamplerInfo::new(
				gfx::tex::FilterMethod::Bilinear,
				gfx::tex::WrapMode::Clamp,
			),
		);

		Some(Texture {
			texture: texture,
			sampler: sampler,
		})
	}

	pub fn to_param(&self) -> (TextureHandle, Option<SamplerHandle>) {
		(self.texture.clone(), Some(self.sampler.clone()))
	}
}
