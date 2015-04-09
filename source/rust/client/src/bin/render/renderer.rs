use std::char;
use std::collections::HashMap;
use std::fmt::Write;

use gfx;
use gfx::traits::*;
use gfx_device_gl as gl;
use nalgebra::{
	Iso3,
	Mat4,
	Ortho3,
	ToHomogeneous,
	Vec2,
	Vec3,
};

use font::Font;
use render::{
	GlyphDrawer,
	Graphics,
	Texture,
};


#[vertex_format]
#[derive(Clone, Copy)]
struct Vertex {
	pos      : [f32; 2],
	tex_coord: [f32; 2],
}


#[shader_param]
struct Params<R: gfx::Resources> {
	transform: [[f32; 4]; 4],

	width : f32,
	height: f32,

	color: gfx::shade::TextureParam<R>,
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
	frame   : gfx::Frame<gl::Resources>,
	batch   : gfx::batch::CoreBatch<Params<gl::Resources>>,
	slice   : gfx::Slice<gl::Resources>,

	transform: Mat4<f32>,

	glyph_drawer: GlyphDrawer,
}

impl Renderer {
	pub fn new(mut graphics: Graphics, width: u32, height: u32) -> Renderer {
		let frame = gfx::Frame::new(width as u16, height as u16);

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
				width as f32, height as f32,
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
			frame   : frame,
			batch   : batch,
			slice   : slice,

			transform: transform,

			glyph_drawer: GlyphDrawer { textures: textures },
		}
	}

	pub fn render(&mut self, output: &[String], command: &str) {
		self.graphics.graphics.clear(
			gfx::ClearData {
				color  : [0.0, 0.0, 0.25, 1.0],
				depth  : 1.0,
				stencil: 0,
			},
			gfx::COLOR,
			&self.frame,
		);

		for (y, line) in output.iter().enumerate() {
			for (x, c) in line.chars().enumerate() {
				self.draw(x as u16, y as u16, c);
			}
		}

		let mut command_line = String::new();

		write!(&mut command_line, "> {}_", command)
			.unwrap_or_else(|e| panic!("Error writing to String: {}", e));

		for (x, c) in command_line.chars().enumerate() {
			self.draw(x as u16, 23, c);
		}

		self.graphics.graphics.end_frame();
	}

	fn draw(&mut self, x: u16, y: u16, c: char) {
		let offset = Vec2::new(-390.0, 270.0);

		let &(ref glyph, ref texture) = match self.glyph_drawer.textures.get(&c) {
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
		let transform = self.transform * translation.to_homogeneous();

		let params = Params {
			transform: *transform.as_array(),

			width : glyph.size.x,
			height: glyph.size.y,

			color: texture.to_param(),
		};

		self.graphics.graphics
			.draw_core(
				&self.batch,
				&self.slice,
				&params,
				&self.frame,
			)
			.unwrap_or_else(|e| panic!("Error drawing graphics: {:?}", e));
	}
}
