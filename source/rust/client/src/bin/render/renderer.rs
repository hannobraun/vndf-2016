use gfx::{
	self,
	DeviceExt,
	ToSlice,
};
use gfx_device_gl::{
	GlDevice,
	GlResources,
};


#[vertex_format]
#[derive(Copy)]
struct Vertex {
	pos: [f32; 2],
}


static VERTEX_SRC: &'static [u8] = b"
	#version 120

	attribute vec2 pos;

	void main() {
		gl_Position = vec4(pos, 0.0, 1.0);
	}
";

static FRAGMENT_SRC: &'static [u8] = b"
	#version 120

	void main() {
		gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
	}
";


pub struct Renderer {
	graphics: gfx::Graphics<GlDevice>,
	frame   : gfx::Frame<GlResources>,
	batch   : gfx::batch::RefBatch<Option<GlResources>>,
}

impl Renderer {
	pub fn new(mut device: GlDevice, width: u32, height: u32) -> Renderer {
		let program = match device.link_program(VERTEX_SRC, FRAGMENT_SRC) {
			Ok(program) => program,
			Err(error)  => panic!("Error linking program: {:?}", error),
		};

		let mesh = device.create_mesh(&[
			Vertex { pos: [ -0.5,  0.5 ] },
			Vertex { pos: [ -0.5, -0.5 ] },
			Vertex { pos: [  0.5,  0.5 ] },
			Vertex { pos: [  0.5, -0.5 ] },
		]);

		let mut graphics = gfx::Graphics::new(device);
		let     frame    = gfx::Frame::new(width as u16, height as u16);

		let slice = mesh.to_slice(gfx::PrimitiveType::TriangleStrip);

		let batch = graphics.make_batch(
			&program,
			&mesh,
			slice,
			&gfx::DrawState::new(),
		);
		let batch = match batch {
			Ok(batch)  => batch,
			Err(error) => panic!("Error making batch: {:?}", error),
		};

		Renderer {
			graphics: graphics,
			frame   : frame,
			batch   : batch,
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

		match self.graphics.draw(&self.batch, &None, &self.frame) {
			Ok(())     => (),
			Err(error) => panic!("Error drawing graphics: {:?}", error),
		}

		self.graphics.end_frame();
	}
}
