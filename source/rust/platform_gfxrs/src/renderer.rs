use std::collections::HashMap;
use std::rc::Rc;

use gfx::{
	mod,
	Device,
	DeviceHelper,
	ToSlice,
};
use gfx::tex::TextureInfo;

use font::Font;
use images::{
	Image,
	Images,
};
use physics::{
	Body,
	Vec2,
};
use platform::Frame;
use window::Window;


type Graphics = gfx::Graphics<gfx::GlDevice, gfx::GlCommandBuffer>;

#[vertex_format]
struct Vertex {
	position : [f32, ..2],
	tex_coord: [f32, ..2],
}

#[shader_param(GridBatch)]
struct GridParams {
	screen_size: [f32, ..2],
	camera_pos : [f32, ..2],
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

		uniform vec2 screen_size;
		uniform vec2 camera_pos;

		in vec2 position;

		void main() {
			mat4 m = mat4(
				2.0 / screen_size.x,                 0.0,  0.0 , 0.0,
				                0.0, 2.0 / screen_size.y,  0.0 , 0.0,
				                0.0,                 0.0, -0.01, 0.0,
				               -1.0,                -1.0,  0.0 , 1.0);

			vec2 camera_trans = screen_size * 0.5 - camera_pos;

			gl_Position = m * vec4(position + camera_trans, 0.0, 1.0);
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

		in vec2 position;
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

			vec2 translated = position + texture_pos + camera_trans;
			gl_Position = m * vec4(translated, 0.0, 1.0);

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
}

impl Renderer {
	pub fn new(window: Rc<Window>, images: Images, font: Font) -> Renderer {
		let mut graphics = gfx::Graphics::new(window.new_device());

		let frame = gfx::Frame::new(window.width, window.height);

		let grid = Grid::new(&mut graphics);

		let mut crafts = HashMap::new();
		for (path, image) in images.move_iter() {
			crafts.insert(path, Texture::from_image(&mut graphics, image));
		}

		Renderer {
			graphics: graphics,
			window  : window,

			frame: frame,

			grid    : grid,
			textures: crafts,
		}
	}

	pub fn render(&mut self, frame: &Frame) {
		self.graphics.clear(
			gfx::ClearData {
				color  : Some([0.0, 0.0, 0.0, 1.0]),
				depth  : None,
				stencil: None,
			},
			&self.frame
		);

		self.draw_grid(&frame.camera);

		for body in frame.ships.iter() {
			self.draw_craft(body, &frame.camera, "images/spaceship.png");
		}

		for body in frame.missiles.iter() {
			self.draw_craft(body, &frame.camera, "images/missile.png");
		}

		self.graphics.end_frame();
		self.window.swap_buffers();
	}

	fn draw_grid(&mut self, camera: &Vec2) {
		let &Vec2(mut camera_x, mut camera_y) = camera;

		camera_x = camera_x % 200.0;
		camera_y = camera_y % 200.0;

		let params = GridParams {
			screen_size: [self.window.width as f32, self.window.height as f32],
			camera_pos : [camera_x as f32, camera_y as f32],
		};

		self.graphics.draw(
			&self.grid.batch,
			&params,
			&self.frame
		);
	}

	fn draw_craft(&mut self, body: &Body, camera: &Vec2, texture_id: &str) {
		let ref texture = self.textures[texture_id.to_string()];

		let Vec2(pos_x, pos_y) = body.position + texture.offset;
		let &Vec2(camera_x, camera_y) = camera;

		let params = TextureParams {
			screen_size: [self.window.width as f32, self.window.height as f32],
			camera_pos : [camera_x as f32, camera_y as f32],
			texture_pos: [pos_x as f32, pos_y as f32],
			tex        : (texture.texture, None)
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
			Vertex { position: [ -700.0, -600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [ -700.0,  600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [ -500.0, -600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [ -500.0,  600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [ -300.0, -600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [ -300.0,  600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [ -100.0, -600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [ -100.0,  600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [  100.0, -600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [  100.0,  600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [  300.0, -600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [  300.0,  600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [  500.0, -600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [  500.0,  600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [  700.0, -600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [  700.0,  600.0 ], tex_coord: [ 0.0, 0.0 ] },

			Vertex { position: [ -700.0, -600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [  700.0, -600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [ -700.0, -400.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [  700.0, -400.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [ -700.0, -200.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [  700.0, -200.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [ -700.0,    0.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [  700.0,    0.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [ -700.0,  200.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [  700.0,  200.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [ -700.0,  400.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [  700.0,  400.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [ -700.0,  600.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [  700.0,  600.0 ], tex_coord: [ 0.0, 0.0 ] },
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
	batch  : TextureBatch,
	texture: gfx::TextureHandle,
	offset : Vec2,
}

impl Texture {
	fn from_image(graphics: &mut Graphics, image: Image) -> Texture {
		Texture::new(
			graphics,
			image.width as f32,
			image.height as f32,
			&image.data
		)
	}

	fn new(
		graphics    : &mut Graphics,
		image_width : f32,
		image_height: f32,
		data        : &Vec<u8>,
	) -> Texture {
		let w = image_width;
		let h = image_height;

		let vertices = vec![
			Vertex { position: [ 0.0, 0.0 ], tex_coord: [ 0.0, 1.0 ] },
			Vertex { position: [ w  , 0.0 ], tex_coord: [ 1.0, 1.0 ] },
			Vertex { position: [ 0.0, h   ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [ w  , h   ], tex_coord: [ 1.0, 0.0 ] },
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
			width : image_width as u16,
			height: image_height as u16,
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

		let batch = graphics
			.make_batch(
				&program,
				&mesh,
				slice,
				&gfx::DrawState::new().blend(gfx::BlendAlpha)
			)
			.unwrap();

		Texture {
			batch  : batch,
			texture: texture,
			offset : Vec2(-w as f64 / 2.0, -h as f64 / 2.0),
		}
	}
}
