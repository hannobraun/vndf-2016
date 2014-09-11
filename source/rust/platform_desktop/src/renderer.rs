use std::collections::HashMap;
use std::rc::Rc;

use cgmath::{
	mod,
	Deg,
	FixedArray,
	Matrix4,
	Point3,
	Vector3,
};
use gfx::{
	mod,
	Device,
	DeviceHelper,
	ToSlice,
};
use gfx::tex::TextureInfo;

use font::{
	Font,
	Glyph,
};
use images::{
	Image,
	Images,
};
use physics::{
	Body,
	Radians,
	Vec2,
};
use platform::Frame;
use window::Window;


type Graphics = gfx::Graphics<gfx::GlDevice, gfx::GlCommandBuffer>;

#[vertex_format]
struct Vertex {
	position : [f32, ..3],
	tex_coord: [f32, ..2],
}

impl Vertex {
	fn for_grid(position: [f32, ..3]) -> Vertex {
		Vertex {
			position : position,
			tex_coord: [0.0, 0.0],
		}
	}

	fn for_texture(position: [f32, ..3], tex_coord: [f32, ..2]) -> Vertex {
		Vertex {
			position : position,
			tex_coord: tex_coord,
		}
	}
}


#[shader_param(GridBatch)]
struct GridParams {
	transform: [[f32, ..4], ..4],
}

#[shader_param(TextureBatch)]
struct TextureParams {
	screen_size: [f32, ..2],
	camera_pos : [f32, ..2],
	texture_pos: [f32, ..2],
	tex        : gfx::shade::TextureParam,
}


static GRID_VERTEX_SHADER: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform mat4 transform;

		in vec3 position;

		void main() {
			gl_Position = transform * vec4(position, 1.0);
		}
	"
};

static GRID_FRAGMENT_SHADER: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		out vec4 out_color;

		void main() {
			out_color = vec4(1.0, 1.0, 1.0, 1.0);
		}
	"
};

static TEXTURE_VERTEX_SHADER: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform vec2 screen_size;
		uniform vec2 camera_pos;
		uniform vec2 texture_pos;

		in vec3 position;
		in vec2 tex_coord;

		out vec2 tex_coord_f;

		void main()
		{
			mat4 m = mat4(
				2.0 / screen_size.x,                 0.0,  0.0 , 0.0,
				                0.0, 2.0 / screen_size.y,  0.0 , 0.0,
				                0.0,                 0.0, -0.01, 0.0,
				               -1.0,                -1.0,  0.0 , 1.0);

			vec2 camera_trans = screen_size * 0.5 - camera_pos;

			vec3 translated =
				position + vec3(texture_pos, 0.0) + vec3(camera_trans, 0.0);
			gl_Position = m * vec4(translated, 1.0);

			tex_coord_f = tex_coord;
		}
	"
};

static TEXTURE_FRAGMENT_SHADER: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform sampler2D tex;

		in vec2 tex_coord_f;

		out vec4 out_color;

		void main()
		{
			out_color = texture(tex, tex_coord_f);
		}
	"
};


pub struct Renderer {
	graphics: Graphics,
	window  : Rc<Window>,

	frame: gfx::Frame,

	grid    : Grid,
	textures: HashMap<String, Texture>,

	glyphs: HashMap<char, Glyph>,
}

impl Renderer {
	pub fn new(window: Rc<Window>, images: Images, font: Font) -> Renderer {
		let mut graphics = gfx::Graphics::new(window.new_device());

		let frame = gfx::Frame::new(window.width, window.height);

		let grid = Grid::new(&mut graphics);

		let mut glyphs   = HashMap::new();
		let mut textures = HashMap::new();
		for (path, image) in images.move_iter() {
			textures.insert(path, Texture::from_image(&mut graphics, image));
		}
		for (c, glyph) in font.move_iter() {
			if c != ' ' {
				textures.insert(
					c.to_string(),
					Texture::from_glyph(&mut graphics, &glyph)
				);
			}
			glyphs.insert(c, glyph);
		}

		Renderer {
			graphics: graphics,
			window  : window,

			frame: frame,

			grid    : grid,
			textures: textures,

			glyphs: glyphs,
		}
	}

	pub fn render(&mut self, frame: &Frame) {
		self.graphics.clear(
			gfx::ClearData {
				color  : [0.0, 0.0, 0.0, 1.0],
				depth  : 0.0,
				stencil: 0,
			},
			gfx::Color,
			&self.frame
		);

		self.draw_grid(&frame.camera.position);

		for body in frame.ships.iter() {
			self.draw_craft(
				body,
				&frame.camera.position,
				"images/spaceship.png"
			);
		}

		for body in frame.missiles.iter() {
			self.draw_craft(
				body,
				&frame.camera.position,
				"images/missile.png"
			);
		}

		self.draw_ui_overlay(frame.input.attitude);

		self.graphics.end_frame();
		self.window.swap_buffers();
	}

	fn draw_grid(&mut self, camera: &Vec2) {
		let camera_position = Vec2(
			camera.x() % 200.0,
			camera.y() % 200.0,
		);

		let projection = cgmath::perspective(
			Deg { s: 45.0f32 },
			self.window.width as f32 / self.window.height as f32,
			0.01, 100000.0,
		);
		let view: Matrix4<f32> = Matrix4::look_at(
			&Point3::new(
				camera_position.x() as f32 - 500.0,
				camera_position.y() as f32 - 500.0,
				500.0
			),
			&Point3::new(
				camera_position.x() as f32,
				camera_position.y() as f32,
				0.0
			),
			&Vector3::new(0.0, 0.0, 1.0),
		);

		let params = GridParams {
			transform: projection.mul(&view).into_fixed(),
		};

		self.graphics.draw(
			&self.grid.batch,
			&params,
			&self.frame
		);
	}

	fn draw_craft(&mut self, body: &Body, camera: &Vec2, texture_id: &str) {
		let texture = self.textures[texture_id.to_string()];
		self.draw_texture(&body.position, camera, &texture);

		let mut text_position = body.position + texture.size + texture.offset;
		self.draw_text(
			text_position,
			camera,
			format!("pos: {:i} / {:i}",
				body.position.x() as int,
				body.position.y() as int
			)
			.as_slice()
		);

		text_position = text_position - Vec2(0.0, 15.0);
		self.draw_text(
			text_position,
			camera,
			format!("att: {:+04i}", body.attitude.degrees()).as_slice()
		);
	}

	fn draw_ui_overlay(&mut self, attitude: Radians) {
		let camera = Vec2(0.0, 0.0);

		let left   = -(self.window.width as f64) / 2.0;
		let right  = -left;
		let bottom = -(self.window.height as f64) / 2.0;


		self.draw_text(
			Vec2(left + 20.0, bottom + 40.0),
			&camera,
			"Change course with the left and right cursor keys"
		);
		self.draw_text(
			Vec2(left + 20.0, bottom + 20.0),
			&camera,
			"Shoot missiles with Enter"
		);

		self.draw_text(
			Vec2(right - 50.0, bottom + 40.0),
			&camera,
			format!("{:+04i}", attitude.degrees()).as_slice()
		);
	}

	fn draw_text(&mut self, mut position: Vec2, camera: &Vec2, text: &str) {
		for c in text.chars() {
			let (offset, advance) = {
				let ref glyph = self.glyphs[c];
				(glyph.offset, glyph.advance)
			};

			if c != ' ' {
				let texture = self.textures[c.to_string()];

				self.draw_texture(
					&(position + offset),
					camera,
					&texture
				);
			}

			position = position + advance;
		}
	}

	fn draw_texture(
		&mut self,
		position: &Vec2,
		camera  : &Vec2,
		texture : &Texture
	) {
		let Vec2(pos_x, pos_y) = position + texture.offset;
		let &Vec2(camera_x, camera_y) = camera;

		let params = TextureParams {
			screen_size: [self.window.width as f32, self.window.height as f32],
			camera_pos : [camera_x as f32, camera_y as f32],
			texture_pos: [pos_x as f32, pos_y as f32],
			tex        : texture.param,
		};

		self.graphics.draw(
			&texture.batch,
			&params,
			&self.frame
		);
	}
}


struct Grid {
	batch: GridBatch,
}

impl Grid {
	fn new(graphics: &mut Graphics) -> Grid {
		let grid_data = vec![
			Vertex::for_grid([ -700.0, -600.0, 0.0 ]),
			Vertex::for_grid([ -700.0,  600.0, 0.0 ]),
			Vertex::for_grid([ -500.0, -600.0, 0.0 ]),
			Vertex::for_grid([ -500.0,  600.0, 0.0 ]),
			Vertex::for_grid([ -300.0, -600.0, 0.0 ]),
			Vertex::for_grid([ -300.0,  600.0, 0.0 ]),
			Vertex::for_grid([ -100.0, -600.0, 0.0 ]),
			Vertex::for_grid([ -100.0,  600.0, 0.0 ]),
			Vertex::for_grid([  100.0, -600.0, 0.0 ]),
			Vertex::for_grid([  100.0,  600.0, 0.0 ]),
			Vertex::for_grid([  300.0, -600.0, 0.0 ]),
			Vertex::for_grid([  300.0,  600.0, 0.0 ]),
			Vertex::for_grid([  500.0, -600.0, 0.0 ]),
			Vertex::for_grid([  500.0,  600.0, 0.0 ]),
			Vertex::for_grid([  700.0, -600.0, 0.0 ]),
			Vertex::for_grid([  700.0,  600.0, 0.0 ]),

			Vertex::for_grid([ -700.0, -600.0, 0.0 ]),
			Vertex::for_grid([  700.0, -600.0, 0.0 ]),
			Vertex::for_grid([ -700.0, -400.0, 0.0 ]),
			Vertex::for_grid([  700.0, -400.0, 0.0 ]),
			Vertex::for_grid([ -700.0, -200.0, 0.0 ]),
			Vertex::for_grid([  700.0, -200.0, 0.0 ]),
			Vertex::for_grid([ -700.0,    0.0, 0.0 ]),
			Vertex::for_grid([  700.0,    0.0, 0.0 ]),
			Vertex::for_grid([ -700.0,  200.0, 0.0 ]),
			Vertex::for_grid([  700.0,  200.0, 0.0 ]),
			Vertex::for_grid([ -700.0,  400.0, 0.0 ]),
			Vertex::for_grid([  700.0,  400.0, 0.0 ]),
			Vertex::for_grid([ -700.0,  600.0, 0.0 ]),
			Vertex::for_grid([  700.0,  600.0, 0.0 ]),
		];

		let mesh  = graphics.device.create_mesh(grid_data);
		let slice = mesh.to_slice(gfx::Line);

		let program = graphics.device
			.link_program(
				GRID_VERTEX_SHADER.clone(),
				GRID_FRAGMENT_SHADER.clone()
			)
			.unwrap_or_else(|error| fail!("error linking program: {}", error));

		let batch = graphics
			.make_batch(
				&program,
				&mesh,
				slice,
				&gfx::DrawState::new().blend(gfx::BlendAlpha)
			)
			.unwrap();

		Grid {
			batch: batch,
		}
	}
}


struct Texture {
	batch : TextureBatch,
	param : gfx::shade::TextureParam,
	size  : Vec2,
	offset: Vec2,
}

impl Texture {
	fn from_glyph(graphics: &mut Graphics, glyph: &Glyph) -> Texture {
		let Vec2(width, height) = glyph.size;

		let data = Vec::from_fn(
			glyph.data.len() * 4,
			|i| {
				if (i + 1) % 4 == 0 {
					glyph.data[i / 4]
				}
				else {
					255
				}
			}
		);

		Texture::new(
			graphics,
			width as f32,
			height as f32,
			&data,
			false,
		)
	}

	fn from_image(graphics: &mut Graphics, image: Image) -> Texture {
		Texture::new(
			graphics,
			image.width as f32,
			image.height as f32,
			&image.data,
			true,
		)
	}

	fn new(
		graphics: &mut Graphics,
		width   : f32,
		height  : f32,
		data    : &Vec<u8>,
		center  : bool,
	) -> Texture {
		let vertices = vec![
			Vertex::for_texture([   0.0,    0.0, 0.0 ], [ 0.0, 1.0 ]),
			Vertex::for_texture([ width,    0.0, 0.0 ], [ 1.0, 1.0 ]),
			Vertex::for_texture([   0.0, height, 0.0 ], [ 0.0, 0.0 ]),
			Vertex::for_texture([ width, height, 0.0 ], [ 1.0, 0.0 ]),
		];

		let mesh  = graphics.device.create_mesh(vertices);
		let slice = mesh.to_slice(gfx::TriangleStrip);

		let program = graphics.device
			.link_program(
				TEXTURE_VERTEX_SHADER.clone(),
				TEXTURE_FRAGMENT_SHADER.clone()
			)
			.unwrap_or_else(|error| fail!("error linking program: {}", error));

		let texture_info = TextureInfo {
			width : width as u16,
			height: height as u16,
			depth : 1,
			levels: -1,
			kind  : gfx::tex::Texture2D,
			format: gfx::tex::RGBA8,
		};

		let texture = graphics.device.create_texture(texture_info).unwrap();
		graphics.device.update_texture(
			&texture,
			&texture_info.to_image_info(),
			data
		)
		.unwrap();

		let sampler = graphics.device.create_sampler(
			gfx::tex::SamplerInfo::new(
				gfx::tex::Bilinear,
				gfx::tex::Clamp
			)
		);

		let batch = graphics
			.make_batch(
				&program,
				&mesh,
				slice,
				&gfx::DrawState::new().blend(gfx::BlendAlpha)
			)
			.unwrap();

		let size   = Vec2(width as f64, height as f64);
		let offset = if center { size * -0.5 } else { Vec2(0.0, 0.0) };

		Texture {
			batch : batch,
			param : (texture, Some(sampler)),
			size  : size,
			offset: offset,
		}
	}
}
