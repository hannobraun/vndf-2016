use std::char;
use std::collections::HashMap;
use std::fmt::Write;

use gfx;
use gfx::traits::*;
use gfx_device_gl as gl;
use nalgebra::{
	Mat4,
	Ortho3,
};

use font::Font;
use render::{
	GlyphDrawer,
	Graphics,
	Params,
	Texture,
};


#[vertex_format]
#[derive(Clone, Copy)]
struct Vertex {
	pos      : [f32; 2],
	tex_coord: [f32; 2],
}


static VERTEX_SRC: &'static [u8] = b"
	#version 120

	attribute vec2 pos;
	attribute vec2 tex_coord;

	uniform mat4 transform;

	uniform float width;
	uniform float height;

	varying vec2 v_tex_coord;

	void main() {
		gl_Position = transform * vec4(pos.x * width, pos.y * height, 0.0, 1.0);
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


pub struct Renderer {
	graphics: Graphics,
	batch   : gfx::batch::CoreBatch<Params<gl::Resources>>,
	slice   : gfx::Slice<gl::Resources>,

	transform: Mat4<f32>,

	glyph_drawer: GlyphDrawer,
}

impl Renderer {
	pub fn new(mut graphics: Graphics, size: (f32, f32)) -> Renderer {
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

		let transform =
			Ortho3::new(
				size.0, size.1,
				-1.0, 1.0,
			)
			.to_mat();

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

		Renderer {
			graphics: graphics,
			batch   : batch,
			slice   : slice,

			transform: transform,

			glyph_drawer: GlyphDrawer { textures: textures },
		}
	}

	pub fn render(&mut self, output: &[String], command: &str) {
		self.graphics.clear();

		for (y, line) in output.iter().enumerate() {
			for (x, c) in line.chars().enumerate() {
				self.glyph_drawer.draw(
					x as u16,
					y as u16,
					c,
					&self.transform,
					&mut self.graphics,
					&self.batch,
					&self.slice,
				);
			}
		}

		let mut command_line = String::new();

		write!(&mut command_line, "> {}_", command)
			.unwrap_or_else(|e| panic!("Error writing to String: {}", e));

		for (x, c) in command_line.chars().enumerate() {
			self.glyph_drawer.draw(
				x as u16,
				23,
				c,
				&self.transform,
				&mut self.graphics,
				&self.batch,
				&self.slice,
			);
		}

		self.graphics.graphics.end_frame();
	}
}
