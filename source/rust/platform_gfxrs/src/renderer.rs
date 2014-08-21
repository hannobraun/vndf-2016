use std::rc::Rc;

use gfx::{
	mod,
	Device,
	DeviceHelper,
};
use gfx::tex::TextureInfo;

use physics::{
	Body,
	Vec2,
};
use platform::Frame;
use window::Window;


#[vertex_format]
struct Vertex {
	position : [f32, ..2],
	tex_coord: [f32, ..2],
}

#[shader_param(GridProgram)]
struct GridParams {
	screen_size: [f32, ..2],
	camera_pos : [f32, ..2],
}

#[shader_param(ShipProgram)]
struct ShipParams {
	screen_size: [f32, ..2],
	camera_pos : [f32, ..2],
	ship_pos   : [f32, ..2],
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

static SHIP_VERTEX_SHADER: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform vec2 screen_size;
		uniform vec2 camera_pos;
		uniform vec2 ship_pos;

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

			vec2 translated = position + ship_pos + camera_trans;
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
	device  : gfx::GlDevice,
	renderer: gfx::Renderer,
	window  : Rc<Window>,

	frame: gfx::Frame,
	state: gfx::DrawState,

	grid: Grid,
	ship: Ship,
}

impl Renderer {
	pub fn new(window: Rc<Window>) -> Renderer {
		let mut device   = window.new_device();
		let     renderer = device.create_renderer();

		let frame = gfx::Frame::new(window.width, window.height);
		let state = gfx::DrawState::new();

		let grid = Grid::new(&mut device);
		let ship = Ship::new(&mut device);

		Renderer {
			device  : device,
			renderer: renderer,
			window  : window,

			frame: frame,
			state: state,

			grid: grid,
			ship: ship,
		}
	}

	pub fn render(&mut self, frame: &Frame) {
		self.renderer.clear(
			gfx::ClearData {
				color  : Some(gfx::Color([0.0, 0.0, 0.0, 1.0])),
				depth  : None,
				stencil: None,
			},
			&self.frame
		);

		self.draw_grid(&frame.camera);

		for body in frame.ships.iter() {
			self.draw_ship(body, &frame.camera);
		}

		self.device.submit(self.renderer.as_buffer());
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

		self.renderer
			.draw(
				&self.grid.mesh,
				self.grid.mesh.get_slice(),
				&self.frame,
				(&self.grid.program, &params),
				&self.state
			)
			.unwrap();
	}

	fn draw_ship(&mut self, body: &Body, &Vec2(camera_x, camera_y): &Vec2) {
		let Vec2(ship_x, ship_y) = body.position;

		let texture_info = TextureInfo {
			width       : 48,
			height      : 48,
			depth       : 1,
			mipmap_range: (0, -1),
			kind        : gfx::tex::Texture2D,
			format      : gfx::tex::RGBA8,
		};

		let texture = self.device.create_texture(texture_info).unwrap();
		self.device.update_texture(
			&texture,
			&texture_info.to_image_info(),
			&Vec::from_elem(48*48*4, 0x77u8)
		)
		.unwrap();

		let params = ShipParams {
			screen_size: [self.window.width as f32, self.window.height as f32],
			camera_pos : [camera_x as f32, camera_y as f32],
			ship_pos   : [ship_x as f32, ship_y as f32],
			tex        : (texture, None)
		};

		self.renderer
			.draw(
				&self.ship.mesh,
				self.ship.mesh.get_slice(),
				&self.frame,
				(&self.ship.program, &params),
				&self.state
			)
			.unwrap();
	}
}


struct Grid {
	mesh   : gfx::Mesh,
	program: GridProgram,
}

impl Grid {
	fn new(device: &mut gfx::GlDevice) -> Grid {
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

		let mesh = device.create_mesh(grid_data, gfx::Line);

		let program =
			device.link_program(
				GRID_VERTEX_SHADER.clone(),
				GRID_FRAGMENT_SHADER.clone()
			)
			.unwrap_or_else(|error| fail!("error linking program: {}", error));

		Grid {
			mesh   : mesh,
			program: program,
		}
	}
}


struct Ship {
	mesh   : gfx::Mesh,
	program: ShipProgram,
}

impl Ship {
	fn new(device: &mut gfx::GlDevice) -> Ship {
		let vertices = vec![
			Vertex { position: [  0.0,  0.0 ], tex_coord: [ 0.0, 0.0 ] },
			Vertex { position: [ 48.0,  0.0 ], tex_coord: [ 1.0, 0.0 ] },
			Vertex { position: [  0.0, 48.0 ], tex_coord: [ 0.0, 1.0 ] },
			Vertex { position: [ 48.0, 48.0 ], tex_coord: [ 1.0, 1.0 ] },
		];

		let mesh = device.create_mesh(vertices, gfx::TriangleStrip);

		let program =
			device.link_program(
				SHIP_VERTEX_SHADER.clone(),
				TEXTURE_FRAGMENT_SHADER.clone()
			)
			.unwrap_or_else(|error| fail!("error linking program: {}", error));

		Ship {
			mesh   : mesh,
			program: program,
		}
	}
}
