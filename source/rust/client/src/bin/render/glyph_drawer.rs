use std::char;
use std::collections::HashMap;

use gfx;
use gfx::traits::*;
use gfx_device_gl as gl;
use nalgebra::{
	Iso3,
	Mat4,
	Vec2,
	Vec3,
	ToHomogeneous,
};

use font::{
	Font,
	Glyph,
};
use render::{
	Graphics,
	Texture,
};


static VERTEX_SRC: &'static [u8] = b"
	#version 120

	attribute vec2 pos;
	attribute vec2 tex_coord;

	uniform mat4 transform;

	uniform vec2 size;

	varying vec2 v_tex_coord;

	void main() {
		gl_Position = transform * vec4(pos * size, 0.0, 1.0);
		v_tex_coord = tex_coord;
	}
";

static FRAGMENT_SRC: &'static [u8] = b"
	#version 120

	varying vec2 v_tex_coord;

	uniform sampler2D color;

	void main() {
		gl_FragColor = texture2D(color, v_tex_coord);
	}
";


#[vertex_format]
#[derive(Clone, Copy)]
struct Vertex {
	pos      : [f32; 2],
	tex_coord: [f32; 2],
}


#[shader_param]
pub struct Params<R: gfx::Resources> {
	pub transform: [[f32; 4]; 4],
	pub size     : [f32; 2],
	pub color    : gfx::shade::TextureParam<R>,
}


pub struct GlyphDrawer {
	textures: HashMap<char, (Glyph, Texture)>,
	batch   : gfx::batch::CoreBatch<Params<gl::Resources>>,
	slice   : gfx::Slice<gl::Resources>,
}

impl GlyphDrawer {
	pub fn new(graphics: &mut Graphics) -> GlyphDrawer {
		let program = graphics.graphics.factory
			.link_program(VERTEX_SRC, FRAGMENT_SRC)
			.unwrap_or_else(|e| panic!("Error linking program: {:?}", e));

		let mesh = graphics.graphics.factory.create_mesh(&[
			Vertex { pos: [ -0.5,  0.5 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { pos: [ -0.5, -0.5 ], tex_coord: [ 0.0, 1.0 ] },
			Vertex { pos: [  0.5,  0.5 ], tex_coord: [ 1.0, 0.0 ] },
			Vertex { pos: [  0.5, -0.5 ], tex_coord: [ 1.0, 1.0 ] },
		]);

		let batch = graphics.graphics
			.make_core(
				&program,
				&mesh,
				&gfx::DrawState::new().blend(gfx::BlendPreset::Alpha),
			)
			.unwrap_or_else(|e| panic!("Error making batch: {:?}", e));

		let slice = mesh.to_slice(gfx::PrimitiveType::TriangleStrip);

		let     font     = Font::load(18);
		let mut textures = HashMap::new();

		// Iterator over all valid values of char.
		for i in (0 .. 0xd7ff + 1).chain((0xe000 .. 0x10ffff + 1)) {
			let c = char::from_u32(i).unwrap_or_else(||
				panic!("Failed to convert u32 to char: {:x}", i)
			);

			let glyph = match font.glyph(c) {
				Some(glyph) => glyph,
				None        => continue,
			};
			match Texture::from_glyph(&glyph, &mut graphics.graphics.factory) {
				Some(texture) => { textures.insert(c, (glyph, texture)); },
				None          => continue,
			}
		}

		GlyphDrawer {
			textures: textures,
			batch   : batch,
			slice   : slice,
		}
	}

	pub fn draw(
		&mut self,
		x        : u16,
		y        : u16,
		c        : char,
		transform: &Mat4<f32>,
		graphics : &mut Graphics,
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
			size     : *glyph.size.as_array(),
			color    : texture.to_param(),
		};

		graphics.draw(
			&self.batch,
			&self.slice,
			&params,
		);
	}
}
