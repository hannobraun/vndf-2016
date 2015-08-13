use std::char;
use std::collections::HashMap;
use std::marker::PhantomData;

use gfx;
use gfx_device_gl as gl;
use nalgebra::{
	Iso3,
	Mat4,
	Vec2,
	Vec3,
	Vec4,
	ToHomogeneous,
};

use client::font::{
	self,
	Font,
};
use client::render::base::{
	Batch,
	Graphics,
	Texture,
	color,
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
	uniform vec4 o_color;

	void main() {
		gl_FragColor = o_color * texture2D(color, v_tex_coord);
	}
";


gfx_vertex!(Vertex {
	pos      @ pos      : [f32; 2],
	tex_coord@ tex_coord: [f32; 2],
});


gfx_parameters!(Params {
	transform@ transform: [[f32; 4]; 4],
	size     @ size     : [f32; 2],
	color    @ color    : gfx::shade::TextureParam<R>,
	o_color  @ o_color  : [f32;4],
});


pub struct GlyphDrawer {
	textures : HashMap<char, (font::Glyph, Texture)>,
	batch    : Batch<Params<gl::Resources>>,
	transform: Mat4<f32>,
}

impl GlyphDrawer {
	pub fn new(graphics: &mut Graphics, transform: Mat4<f32>) -> GlyphDrawer {
		let batch = Batch::new(
			graphics,
			VERTEX_SRC, FRAGMENT_SRC,
			&[
				Vertex { pos: [ -0.5,  0.5 ], tex_coord: [ 0.0, 0.0 ] },
				Vertex { pos: [ -0.5, -0.5 ], tex_coord: [ 0.0, 1.0 ] },
				Vertex { pos: [  0.5,  0.5 ], tex_coord: [ 1.0, 0.0 ] },
				Vertex { pos: [  0.5, -0.5 ], tex_coord: [ 1.0, 1.0 ] },
			]
		);

		let     font     = Font::load(18);
		let mut textures = HashMap::new();

		// Iterate over all valid values of char
		for i in (0 .. 0xd7ff + 1).chain((0xe000 .. 0x10ffff + 1)) {
			let c = char::from_u32(i).unwrap_or_else(||
				panic!("Failed to convert u32 to char: {:x}", i)
			);

			let glyph = match font.glyph(c) {
				Some(glyph) => glyph,
				None        => continue,
			};
			match Texture::from_glyph(&glyph, &mut graphics.factory) {
				Some(texture) => { textures.insert(c, (glyph, texture)); },
				None          => continue,
			}
		}

		GlyphDrawer {
			textures : textures,
			batch    : batch,
			transform: transform,
		}
	}

	pub fn set_transform(&mut self, transform: Mat4<f32>) {
		self.transform = transform;
	}

	pub fn draw_at(
		&mut self,
		x       : f64,
		y       : f64,
		c       : char,
		color 	: color::Color,
		graphics: &mut Graphics,
		) {
		let &(ref glyph, ref texture) = match self.textures.get(&c) {
			Some(result) => result,
			None         => return,
		};
		let position =
			Vec2::new(0.0,0.0) +
			(glyph.size * 0.5) +
			glyph.offset +
			Vec2::new(x as f32, y as f32);

		let translation = Iso3::new(
			Vec3::new(position.x, position.y, 0.0),
			Vec3::new(0.0, 0.0, 0.0),
		);
		let transform = self.transform * translation.to_homogeneous();

		let params = Params {
			transform: *transform.as_array(),
			size     : *glyph.size.as_array(),
			color    : texture.to_param(),
			o_color  : *Vec4::new(color[0],color[1],color[2],1.0).as_array(),
			_r       : PhantomData,
		};

		graphics.draw(
			&self.batch,
			&params,
		);
	}
}
