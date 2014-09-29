use std::collections::HashMap;
use std::rc::Rc;

use cgmath::{
	mod,
	Deg,
	Matrix,
	Matrix4,
	Vector,
	Vector2,
	Vector3,
	Vector4,
};
use gfx;

use font::{
	Font,
	Glyph,
};
use images::Images;
use physics::Body;
use platform::{
	Camera,
	Frame,
	Input,
};
use window::Window;

use super::{
	Graphics,
	Transform,
};
use super::billboard::Billboard;
use super::grid::Grid;
use super::icon::Icon;
use super::planet::Planet;
use super::texture::Texture;


pub struct Renderer {
	graphics: Graphics,
	window  : Rc<Window>,

	frame: gfx::Frame,

	billboards: HashMap<String, Billboard>,
	grid      : Grid,
	planet    : Planet,
	icons     : HashMap<String, Icon>,

	glyphs: HashMap<char, Glyph>,
}

impl Renderer {
	pub fn new(window: Rc<Window>, images: Images, font: Font) -> Renderer {
		let mut graphics = gfx::Graphics::new(window.new_device());

		let frame      = gfx::Frame::new(window.width, window.height);
		let draw_state = gfx::DrawState::new()
			.multi_sample()
			.blend(gfx::BlendAlpha)
			.depth(gfx::state::Less, true);

		let grid   = Grid::new(&mut graphics, &draw_state);
		let planet = Planet::new(&mut graphics, &draw_state, 2576.0);

		let mut billboards = HashMap::new();
		let mut glyphs     = HashMap::new();
		let mut icons      = HashMap::new();
		for (path, image) in images.into_iter() {
			let texture = Texture::from_image(&image, &mut graphics);

			billboards.insert(
				path,
				Billboard::new(&mut graphics, &draw_state, texture)
			);
		}
		for (c, glyph) in font.into_iter() {
			if c != ' ' {
				let texture = Texture::from_glyph(&glyph, &mut graphics);
				icons.insert(
					c.to_string(),
					Icon::new(
						&mut graphics,
						&draw_state,
						texture,
						false,
					),
				);
			}
			glyphs.insert(c, glyph);
		}

		Renderer {
			graphics: graphics,
			window  : window,

			frame: frame,

			billboards: billboards,
			grid      : grid,
			planet    : planet,
			icons     : icons,

			glyphs: glyphs,
		}
	}

	pub fn render(&mut self, frame: &Frame) {
		let projection = self.perspective();

		self.graphics.clear(
			gfx::ClearData {
				color  : [0.0, 0.0, 0.0, 1.0],
				depth  : 1.0,
				stencil: 0,
			},
			gfx::Color | gfx::Depth,
			&self.frame
		);

		self.grid.draw(
			&mut self.graphics,
			&self.frame,
			&frame.camera,
			projection
		);



		self.planet.draw(
			&mut self.graphics,
			&self.frame,
			projection,
			&frame.camera,
		);

		for body in frame.ships.iter() {
			self.draw_craft(
				body,
				&frame.camera,
				"images/spaceship.png"
			);
		}

		for body in frame.missiles.iter() {
			self.draw_craft(
				body,
				&frame.camera,
				"images/missile.png"
			);
		}

		self.draw_ui_overlay(frame.input);

		self.graphics.end_frame();
		self.window.swap_buffers();
	}

	fn draw_craft(&mut self, body: &Body, camera: &Camera, icon_id: &str) {
		let billboard = self.billboards[icon_id.to_string()];
		let view_projection = self.perspective()
			.mul(&camera.to_transform());
		let screen_position = view_projection
			.mul_v(&Vector4::new(
				body.position.x as f32,
				body.position.y as f32,
				body.position.z as f32,
				1.0,
			));
		let screen_position = screen_position.div_s(screen_position.w);

		let transform = self.ortho()
			.mul(&Matrix4::from_translation(&Vector3::new(
				screen_position.x / self.window.size.x + self.window.size.x / 2.0,
				screen_position.y / self.window.size.y + self.window.size.y / 2.0,
				0.0,
			)));

		billboard.draw(
			&mut self.graphics,
			&self.frame,
			&Vector3::new(body.position.x as f32, body.position.y as f32, body.position.z as f32),
			&view_projection,
			&self.window.size,
		);

		let mut text_position = Vector2::new(screen_position.x, screen_position.y) + billboard.size.div_s(2.0);
		self.draw_text(
			format!("pos: {:i} / {:i} / {:i}",
				body.position.x as int,
				body.position.y as int,
				body.position.z as int,
			)
			.as_slice(),
			&transform.mul(&Matrix4::from_translation(&text_position.extend(0.0))),
		);

		text_position = text_position - Vector2::new(0.0, 15.0);
		self.draw_text(
			format!("vel: {:i} / {:i} / {:i}",
				body.velocity.x as int,
				body.velocity.y as int,
				body.velocity.z as int,
			).as_slice(),
			&transform.mul(&Matrix4::from_translation(&text_position.extend(0.0))),
		);
	}

	fn draw_ui_overlay(&mut self, input: Input) {
		let projection = self.ortho();

		let right  = self.window.size.x;


		self.draw_text(
			"Move camera with WASD; change zoom with R and F",
			&projection.mul(&Matrix4::from_translation(&Vector2::new(20.0, 60.0).extend(0.0)))
		);
		self.draw_text(
			"Change attitude with the cursor keys, toggle thrust with Space",
			&projection.mul(&Matrix4::from_translation(&Vector2::new(20.0, 40.0).extend(0.0))),
		);
		self.draw_text(
			"Shoot missiles with Enter",
			&projection.mul(&Matrix4::from_translation(&Vector2::new(20.0, 20.0).extend(0.0))),
		);

		self.draw_text(
			format!("{}", input.attitude).as_slice(),
			&projection.mul(&Matrix4::from_translation(&Vector2::new(right - 100.0, 40.0).extend(0.0))),
		);
		self.draw_text(
			if input.thrust { "Thrust ON" } else { "Thrust OFF" },
			&projection.mul(&Matrix4::from_translation(&Vector2::new(right - 100.0, 20.0).extend(0.0))),
		);
	}

	fn draw_text(&mut self, text: &str, transform: &Transform) {
		let mut total_advance = Vector2::zero();

		for c in text.chars() {
			let (offset, advance) = {
				let ref glyph = self.glyphs[c];
				(glyph.offset, glyph.advance)
			};

			if c != ' ' {
				let icon = self.icons[c.to_string()];

				icon.draw(
					&mut self.graphics,
					&self.frame,
					&transform.mul(&Matrix4::from_translation(&(offset.extend(0.0) + total_advance.extend(0.0)))),
				);
			}

			total_advance = total_advance + advance;
		}
	}

	fn ortho(&self) -> Transform {
		cgmath::ortho(
			0.0, self.window.size.x,
			0.0, self.window.size.y,
			-1.0, 1.0,
		)
	}

	fn perspective(&self) -> Transform {
		cgmath::perspective(
			Deg { s: 45.0f32 },
			self.window.size.x / self.window.size.y,
			10.0, 100000.0,
		)
	}
}
