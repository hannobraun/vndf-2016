use gfx;
use gfx::traits::*;
use gfx_device_gl::{
	GlDevice,
	GlResources,
};

use font::Glyph;


pub type TextureHandle = gfx::TextureHandle<GlResources>;
pub type SamplerHandle = gfx::SamplerHandle<GlResources>;


pub struct Texture {
	pub texture: TextureHandle,
	pub sampler: SamplerHandle,
}

impl Texture {
	pub fn from_glyph(glyph: &Glyph, device: &mut GlDevice) -> Option<Texture> {
		let width  = glyph.size.x as u16;
		let height = glyph.size.y as u16;

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
			kind  : gfx::tex::TextureKind::Texture2D,
			format: gfx::tex::RGBA8,
		};
		let image_info = texture_info.to_image_info();

		let texture = device
			.create_texture(texture_info)
			.unwrap_or_else(|e| panic!("Error creating texture: {:?}", e));
		device
			.update_texture(
				&texture,
				&image_info,
				data.as_slice(),
			)
			.unwrap_or_else(|e| panic!("Error updating texture: {:?}", e));

		let sampler = device.create_sampler(
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
