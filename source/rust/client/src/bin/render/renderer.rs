use std::collections::HashMap;
use std::marker::PhantomData;

use gfx;
use gfx::traits::*;
use gfx_device_gl::{
	GlDevice,
	GlResources,
};
use nalgebra::{
	Iso3,
	Mat4,
	Ortho3,
	ToHomogeneous,
	Vec2,
	Vec3,
};

use font::{
	Font,
	Glyph,
};
use texture::Texture;


const HEIGHT: f32 = 20.0;


#[vertex_format]
#[derive(Copy)]
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

	_marker: PhantomData<R>,
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
	graphics: gfx::Graphics<GlDevice>,
	frame   : gfx::Frame<GlResources>,
	program : gfx::ProgramHandle<GlResources>,
	mesh    : gfx::Mesh<GlResources>,

	transform: Mat4<f32>,
	textures : HashMap<char, (Glyph, Texture)>,
}

impl Renderer {
	pub fn new(device: GlDevice, width: u32, height: u32) -> Renderer {
		let mut graphics = device.into_graphics();
		let     frame    = gfx::Frame::new(width as u16, height as u16);

		let program = graphics.device
			.link_program(VERTEX_SRC, FRAGMENT_SRC)
			.unwrap_or_else(|e| panic!("Error linking program: {:?}", e));

		let mesh = graphics.device.create_mesh(&[
			Vertex { pos: [ -0.5,  0.5 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { pos: [ -0.5, -0.5 ], tex_coord: [ 0.0, 1.0 ] },
			Vertex { pos: [  0.5,  0.5 ], tex_coord: [ 1.0, 0.0 ] },
			Vertex { pos: [  0.5, -0.5 ], tex_coord: [ 1.0, 1.0 ] },
		]);

		let transform =
			Ortho3::new(
				width as f32, height as f32,
				-1.0, 1.0,
			)
			.to_mat();

		let font  = Font::load();

		let mut textures = HashMap::new();
		for i in range(33, 127) {
			let c       = ::std::char::from_u32(i).unwrap();
			let glyph   = font.glyph(c, HEIGHT as u32 * 2);
			let texture = Texture::from_glyph(&glyph, &mut graphics.device);

			textures.insert(c, (glyph, texture));
		}

		Renderer {
			graphics: graphics,
			frame   : frame,
			program : program,
			mesh    : mesh,

			transform: transform,

			textures: textures,
		}
	}

	pub fn render(&mut self) {
		self.graphics.clear(
			gfx::ClearData {
				color  : [0.0, 0.0, 0.25, 1.0],
				depth  : 1.0,
				stencil: 0,
			},
			gfx::COLOR,
			&self.frame,
		);

		let mut position = Vec2::new(-490.0, 300.0);

		let text = "This is Von Neumann Defense Force.";

		for c in text.chars() {
			if c == ' ' {
				position = position + Vec2::new(15.0, 0.0);
				continue;
			}

			let (ref glyph, ref texture) = self.textures[c];

			let glyph_position = position + (glyph.size * 0.5) + glyph.offset;
			let translation = Iso3::new(
				Vec3::new(glyph_position.x, glyph_position.y, 0.0),
				Vec3::new(0.0, 0.0, 0.0),
			);

			position = position + glyph.advance;

			let params = Params {
				transform: *(self.transform * translation.to_homogeneous()).as_array(),

				width : glyph.size.x,
				height: glyph.size.x,

				color: texture.to_param(),

				_marker: PhantomData,
			};

			let batch = self.graphics
				.make_batch(
					&self.program,
					params,
					&self.mesh,
					self.mesh.to_slice(gfx::PrimitiveType::TriangleStrip),
					&gfx::DrawState::new().blend(gfx::BlendPreset::Alpha),
				)
				.unwrap_or_else(|e| panic!("Error making batch: {:?}", e));

			self.graphics
				.draw(&batch, &self.frame)
				.unwrap_or_else(|e| panic!("Error drawing graphics: {:?}", e));
		}

		self.graphics.end_frame();
	}
}
