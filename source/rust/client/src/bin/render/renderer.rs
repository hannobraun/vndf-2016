use std::marker::PhantomData;

use gfx::{
	self,
	Device,
	DeviceExt,
	ToSlice,
};
use gfx_device_gl::{
	GlDevice,
	GlResources,
};
use nalgebra::{
	Mat4,
	Ortho3,
};

use font;


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
	batch   : gfx::batch::RefBatch<Params<GlResources>>,

	transform: Mat4<f32>,

	texture: gfx::device::Handle<u32, gfx::device::tex::TextureInfo>,
	sampler: gfx::device::Handle<u32, gfx::device::tex::SamplerInfo>,
}

impl Renderer {
	pub fn new(mut device: GlDevice, width: u32, height: u32) -> Renderer {
		let     font  = font::load();
		let ref glyph = font['A'];

		let program = device
			.link_program(VERTEX_SRC, FRAGMENT_SRC)
			.unwrap_or_else(|e| panic!("Error linking program: {:?}", e));

		let mesh = device.create_mesh(&[
			Vertex { pos: [ -0.5,  0.5 ], tex_coord: [ 0.0, 1.0 ] },
			Vertex { pos: [ -0.5, -0.5 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { pos: [  0.5,  0.5 ], tex_coord: [ 1.0, 1.0 ] },
			Vertex { pos: [  0.5, -0.5 ], tex_coord: [ 1.0, 0.0 ] },
		]);

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

		let mut graphics = gfx::Graphics::new(device);
		let     frame    = gfx::Frame::new(width as u16, height as u16);

		let slice = mesh.to_slice(gfx::PrimitiveType::TriangleStrip);

		let batch = graphics
			.make_batch(
				&program,
				&mesh,
				slice,
				&gfx::DrawState::new(),
			)
			.unwrap_or_else(|e| panic!("Error making batch: {:?}", e));

		let transform =
			Ortho3::new(
				width as f32, height as f32,
				-1.0, 1.0,
			)
			.to_mat();

		Renderer {
			graphics: graphics,
			frame   : frame,
			batch   : batch,

			transform: transform,

			texture: texture,
			sampler: sampler,
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

		let params = Params {
			transform: *self.transform.as_array(),

			width : 20.0,
			height: 40.0,

			color: (self.texture, Some(self.sampler)),

			_marker: PhantomData,
		};

		self.graphics
			.draw(&self.batch, &params, &self.frame)
			.unwrap_or_else(|e| panic!("Error drawing graphics: {:?}", e));

		self.graphics.end_frame();
	}
}
