use gfx::{
	self,
	Device,
};
use gfx_device_gl::GlDevice;

use font::Glyph;


pub type TextureHandle = gfx::device::Handle<u32, gfx::device::tex::TextureInfo>;
pub type SamplerHandle = gfx::device::Handle<u32, gfx::device::tex::SamplerInfo>;


pub struct Texture {
	pub texture: TextureHandle,
	pub sampler: SamplerHandle,
}

impl Texture {
	pub fn from_glyph(glyph: &Glyph, device: &mut GlDevice) -> Texture {
		let format = gfx::tex::Format::Unsigned(
			gfx::tex::Components::R,
			8,
			gfx::attrib::IntSubType::Normalized,
		);
		let texture_info = gfx::tex::TextureInfo {
			width : glyph.size.x as u16,
			height: glyph.size.y as u16,
			depth : 1,
			levels: 1,
			kind  : gfx::tex::TextureKind::Texture2D,
			format: format,
		};
		let image_info = texture_info.to_image_info();

		let texture = device
			.create_texture(texture_info)
			.unwrap_or_else(|e| panic!("Error creating texture: {:?}", e));
		device
			.update_texture(
				&texture,
				&image_info,
				glyph.data.as_slice(),
			)
			.unwrap_or_else(|e| panic!("Error updating texture: {:?}", e));

		let sampler = device.create_sampler(
			gfx::tex::SamplerInfo::new(
				gfx::tex::FilterMethod::Bilinear,
				gfx::tex::WrapMode::Clamp,
			),
		);

		Texture {
			texture: texture,
			sampler: sampler,
		}
	}

	pub fn to_param(&self) -> (TextureHandle, Option<SamplerHandle>) {
		(self.texture, Some(self.sampler))
	}
}
