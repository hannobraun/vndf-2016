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
use game::physics::Body;
use images::Images;
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
use super::planet::Planet;
use super::rings::Rings;
use super::texture::Texture;


pub struct Renderer {
	graphics: Graphics,
	window  : Rc<Window>,

	frame: gfx::Frame,

	billboard: Billboard,
	planet   : Planet,
	rings    : Rings,

	glyphs        : HashMap<char, Glyph>,
	glyph_textures: HashMap<char, Texture>,
	image_textures: HashMap<String, Texture>,

}

impl Renderer {
	pub fn new(window: Rc<Window>, images: Images, font: Font) -> Renderer {
		let mut graphics = gfx::Graphics::new(window.new_device());

		let frame      = gfx::Frame::new(window.width, window.height);
		let draw_state = gfx::DrawState::new()
			.multi_sample()
			.blend(gfx::BlendAlpha)
			.depth(gfx::state::Less, true);

		let billboard = Billboard::new(&mut graphics, &draw_state);
		let planet    = Planet::new(&mut graphics, &draw_state);
		let rings     = Rings::new(&mut graphics, &draw_state);

		let mut glyphs         = HashMap::new();
		let mut glyph_textures = HashMap::new();
		let mut image_textures = HashMap::new();

		for (path, image) in images.into_iter() {
			let texture = Texture::from_image(&image, &mut graphics);
			image_textures.insert(path, texture);
		}

		for (c, glyph) in font.into_iter() {
			if c != ' ' {
				let texture = Texture::from_glyph(&glyph, &mut graphics);
				glyph_textures.insert(c, texture);
			}
			glyphs.insert(c, glyph);
		}

		Renderer {
			graphics: graphics,
			window  : window,

			frame: frame,

			billboard: billboard,
			planet   : planet,
			rings    : rings,

			glyphs        : glyphs,
			glyph_textures: glyph_textures,
			image_textures: image_textures,

		}
	}

	pub fn render(&mut self, frame: &Frame) {
		let projection      = self.perspective();
		let view_projection = projection.mul(&frame.camera.to_transform());

		self.graphics.clear(
			gfx::ClearData {
				color  : [0.0, 0.0, 0.0, 1.0],
				depth  : 1.0,
				stencil: 0,
			},
			gfx::Color | gfx::Depth,
			&self.frame
		);

		for planet in frame.planets.iter() {
			let position = Vector3::new(
				planet.position.x as f32,
				planet.position.y as f32,
				planet.position.z as f32,
			);

			self.planet.draw(
				&mut self.graphics,
				&self.frame,
				position,
				planet.radius as f32,
				planet.color,
				projection,
				&frame.camera,
			);
		}

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

		self.rings.draw(
			&mut self.graphics,
			&self.frame,
			&view_projection.mul(&Matrix4::from_translation(
				&Vector3::new(
					frame.camera.center.x as f32,
					frame.camera.center.y as f32,
					frame.camera.center.z as f32,
				)
			)),
		);

		self.draw_ui_overlay(frame.input);

		self.graphics.end_frame();
		self.window.swap_buffers();
	}

	fn draw_craft(&mut self, body: &Body, camera: &Camera, icon_id: &str) {
		let texture = self.image_textures[icon_id.to_string()];

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

		self.billboard.draw(
			&mut self.graphics,
			&self.frame,
			&Vector3::new(body.position.x as f32, body.position.y as f32, body.position.z as f32),
			&texture,
			&view_projection,
			&self.window.size,
		);

		let mut text_position = Vector2::new(screen_position.x, screen_position.y) + texture.size.div_s(2.0);
		self.draw_text(
			format!("pos: {:i} / {:i} / {:i}",
				body.position.x as int,
				body.position.y as int,
				body.position.z as int,
			)
			.as_slice(),
			&text_position.extend(0.0),
			&transform,
		);

		text_position = text_position - Vector2::new(0.0, 15.0);
		self.draw_text(
			format!("vel: {:i} / {:i} / {:i}",
				body.velocity.x as int,
				body.velocity.y as int,
				body.velocity.z as int,
			).as_slice(),
			&text_position.extend(0.0),
			&transform,
		);
	}

	fn draw_ui_overlay(&mut self, input: Input) {
		let projection = self.ortho();

		let right  = self.window.size.x;


		self.draw_text(
			"Move camera with WASD; change zoom with R and F",
			&Vector2::new(20.0, 60.0).extend(0.0),
			&projection,
		);
		self.draw_text(
			"Change attitude with the cursor keys, toggle thrust with Space",
			&Vector2::new(20.0, 40.0).extend(0.0),
			&projection,
		);
		self.draw_text(
			"Shoot missiles with Enter",
			&Vector2::new(20.0, 20.0).extend(0.0),
			&projection,
		);

		self.draw_text(
			format!("{}", input.attitude).as_slice(),
			&Vector2::new(right - 100.0, 40.0).extend(0.0),
			&projection,
		);
		self.draw_text(
			if input.thrust { "Thrust ON" } else { "Thrust OFF" },
			&Vector2::new(right - 100.0, 20.0).extend(0.0),
			&projection,
		);
	}

	fn draw_text(
		&mut self,
		text     : &str,
		position : &Vector3<f32>,
		transform: &Transform
	) {
		let mut total_advance = Vector2::zero();

		for c in text.chars() {
			let (offset, advance) = {
				let ref glyph = self.glyphs[c];
				(glyph.offset, glyph.advance)
			};

			if c != ' ' {
				let texture = self.glyph_textures[c];

				let offset_to_edge = texture.size.mul_s(0.5);
				let total_offset   = offset + offset_to_edge + total_advance;

				self.billboard.draw(
					&mut self.graphics,
					&self.frame,
					position,
					&texture,
					&transform.mul(&Matrix4::from_translation(
						&total_offset.extend(0.0)
					)),
					&self.window.size,
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
