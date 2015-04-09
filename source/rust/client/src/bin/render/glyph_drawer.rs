use std::collections::HashMap;

use gfx;
use gfx_device_gl as gl;
use nalgebra::{
	Iso3,
	Mat4,
	Vec2,
	Vec3,
	ToHomogeneous,
};

use font::Glyph;
use render::{
	Graphics,
	Params,
	Texture,
};


pub struct GlyphDrawer {
	pub textures: HashMap<char, (Glyph, Texture)>,
}

impl GlyphDrawer {
	pub fn draw(
		&mut self,
		x        : u16,
		y        : u16,
		c        : char,
		transform: &Mat4<f32>,
		graphics : &mut Graphics,
		batch    : &gfx::batch::CoreBatch<Params<gl::Resources>>,
		slice    : &gfx::Slice<gl::Resources>,
	) {
		let offset = Vec2::new(-390.0, 270.0);

		let &(ref glyph, ref texture) = match self.textures.get(&c) {
			Some(result) => result,
			None         => return,
		};

		let position =
			offset +
			(glyph.size * 0.5) +
			glyph.offset +
			Vec2::new(9.0 * x as f32, 18.0 * -(y as f32));
		let translation = Iso3::new(
			Vec3::new(position.x, position.y, 0.0),
			Vec3::new(0.0, 0.0, 0.0),
		);
		let transform = *transform * translation.to_homogeneous();

		let params = Params {
			transform: *transform.as_array(),

			width : glyph.size.x,
			height: glyph.size.y,

			color: texture.to_param(),
		};

		graphics.draw(
			batch,
			slice,
			&params,
		);
	}
}
