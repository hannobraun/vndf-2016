use std::collections::HashMap;
use std::rc::Rc;

use cgmath::{
	mod,
	deg,
	Angle,
	ApproxEq,
	Basis2,
	Deg,
	Matrix,
	Matrix4,
	Rotation,
	Rotation2,
	Vector,
	Vector2,
	Vector3,
};
use gfx;

use font::{
	Font,
	Glyph,
};
use game::ecs::Planet as GamePlanet;
use game::physics::Body;
use images::Images;
use platform::{
	Camera,
	Frame,
	Input,
};
use render::{
	Drawer,
	Graphics,
	Transform,
};
use render::drawers::{
	Base,
	BaseDrawer,
	Billboard,
	BillboardDrawer,
	Line,
	LineDrawer,
	NavDisc,
	NavDiscDrawer,
	Planet,
	PlanetDrawer,
};
use window::Window;

use super::texture::Texture;


pub struct Renderer {
	graphics: Graphics,
	window  : Rc<Window>,

	frame: gfx::Frame,

	base_drawer     : BaseDrawer,
	billboard_drawer: BillboardDrawer,
	line_drawer     : LineDrawer,
	planet_drawer   : PlanetDrawer,
	nav_disc_drawer : NavDiscDrawer,

	glyphs        : HashMap<char, Glyph>,
	glyph_textures: HashMap<char, Texture>,
	image_textures: HashMap<String, Texture>,

	bases: Vec<Base>,
}

impl Renderer {
	pub fn new(window: Rc<Window>, images: Images, font: Font) -> Renderer {
		let mut graphics = gfx::Graphics::new(window.new_device());

		let frame      = gfx::Frame::new(window.width, window.height);
		let draw_state = gfx::DrawState::new()
			.multi_sample()
			.blend(gfx::BlendAlpha)
			.depth(gfx::state::Less, true);

		let base_drawer: BaseDrawer =
			Drawer::new(&mut graphics, &draw_state);
		let billboard_drawer: BillboardDrawer =
			Drawer::new(&mut graphics, &draw_state);
		let line_drawer: LineDrawer =
			Drawer::new(&mut graphics, &draw_state);
		let planet_drawer: PlanetDrawer =
			Drawer::new(&mut graphics, &draw_state);
		let nav_disc_drawer: NavDiscDrawer =
			Drawer::new(&mut graphics, &draw_state);

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

			base_drawer     : base_drawer,
			billboard_drawer: billboard_drawer,
			line_drawer     : line_drawer,
			planet_drawer   : planet_drawer,
			nav_disc_drawer : nav_disc_drawer,

			glyphs        : glyphs,
			glyph_textures: glyph_textures,
			image_textures: image_textures,

			bases: Vec::new(),
		}
	}

	pub fn render(&mut self, frame: &Frame) {
		self.bases.clear();

		let projection      = self.perspective();
		let view_projection = projection.mul(&frame.camera.to_transform());

		self.graphics.clear(
			gfx::ClearData {
				color  : [0.0, 0.0, 0.0, 1.0],
				depth  : 1.0,
				stencil: 0,
			},
			gfx::COLOR | gfx::DEPTH,
			&self.frame
		);

		for planet in frame.planets.iter() {
			self.draw_planet(
				planet,
				&frame.camera,
				&projection,
				&view_projection
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

		self.draw_rings(view_projection, &frame.camera);

		for base in self.bases.iter() {
			self.base_drawer.draw(
				&mut self.graphics,
				&self.frame,
				base,
			);
		}

		self.draw_ui_overlay(frame.input);

		self.graphics.end_frame();
		self.window.swap_buffers();
	}

	fn draw_planet(
		&mut self,
		planet    : &GamePlanet,
		camera    : &Camera,
		projection: &Transform,
		transform : &Transform
	) {
		let center = Vector3::new(
			camera.center.x as f32,
			camera.center.y as f32,
			camera.center.z as f32,
		);
		let position = Vector3::new(
			planet.position.x as f32,
			planet.position.y as f32,
			planet.position.z as f32,
		);

		self.planet_drawer.draw(
			&mut self.graphics,
			&self.frame,
			&Planet {
				position  : position,
				radius    : planet.radius as f32,
				color     : planet.color,
				projection: *projection,
				camera    : *camera,
			}
		);

		self.draw_line_to_disc(&center, &position, transform);
	}

	fn draw_craft(&mut self, body: &Body, camera: &Camera, icon_id: &str) {
		let texture = self.image_textures[icon_id.to_string()];

		let view_projection = self.perspective()
			.mul(&camera.to_transform());

		let center = Vector3::new(
			camera.center.x as f32,
			camera.center.y as f32,
			camera.center.z as f32,
		);
		let position = Vector3::new(
			body.position.x as f32,
			body.position.y as f32,
			body.position.z as f32,
		);

		self.billboard_drawer.draw(
			&mut self.graphics,
			&self.frame,
			&Billboard {
				position   : position,
				offset     : Vector2::zero(),
				texture    : texture,
				transform  : view_projection,
				screen_size: self.window.size,
			},
		);

		if !center.approx_eq_eps(&position, &50.0) {
			self.draw_line_to_disc(&center, &position, &view_projection);
		}

		let text_offset = texture.size.div_s(2.0);
		self.draw_text(
			format!("pos: {:i} / {:i} / {:i}",
				body.position.x as int,
				body.position.y as int,
				body.position.z as int,
			)
			.as_slice(),
			&position,
			&text_offset,
			&view_projection,
		);

		let text_offset = text_offset - Vector2::new(0.0, 15.0);
		self.draw_text(
			format!("vel: {:i} / {:i} / {:i}",
				body.velocity.x as int,
				body.velocity.y as int,
				body.velocity.z as int,
			).as_slice(),
			&position,
			&text_offset,
			&view_projection,
		);
	}

	fn draw_rings(&mut self, view_projection: Transform, camera: &Camera) {
		let camera_center = Vector3::new(
			camera.center.x as f32,
			camera.center.y as f32,
			camera.center.z as f32,
		);

		let transform = view_projection.mul(
			&Matrix4::from_translation(&camera_center)
		);

		// transform[3][3] is proportional to the camera distance, thus scaling
		// the vertex with it will make the size independent of the zoom level.
		let radius = transform[3][3] * 0.55;

		let ring_radius = radius * 0.9;
		self.draw_text(
			format!("{} km", ring_radius.round()).as_slice(),
			&(camera_center + Vector3::new(ring_radius, 0.0, 0.0)),
			&Vector2::zero(),
			&view_projection,
		);

		let ring_radius = radius * 0.15 + (radius * 0.9 - radius * 0.15) / 2.0;
		self.draw_text(
			format!("{} km", ring_radius.round()).as_slice(),
			&(camera_center + Vector3::new(ring_radius, 0.0, 0.0)),
			&Vector2::zero(),
			&view_projection,
		);

		let ring_radius = radius * 0.15;
		self.draw_text(
			format!("{} km", ring_radius.round()).as_slice(),
			&(camera_center + Vector3::new(ring_radius, 0.0, 0.0)),
			&Vector2::zero(),
			&view_projection,
		);

		let angles: &[f32] = [
			30.0,
			60.0,
			90.0,
			120.0,
			150.0,
			180.0,
			210.0,
			240.0,
			270.0,
			300.0,
			330.0,
		];
		for &angle in angles.iter() {
			let rotation: Basis2<f32> = Rotation2::from_angle(Angle::from(deg(angle)));
			self.draw_text(
				format!("{}°", angle as u16).as_slice(),
				&(camera_center + rotation.rotate_vector(&Vector2::new(radius, 0.0)).extend(0.0)),
				&Vector2::zero(),
				&view_projection,
			);
		}


		self.nav_disc_drawer.draw(
			&mut self.graphics,
			&self.frame,
			&NavDisc {
				radius   : radius,
				transform: transform,
			},
		);
	}

	fn draw_ui_overlay(&mut self, input: Input) {
		let projection = self.ortho();

		let right  = self.window.size.x;


		self.draw_text(
			"Move camera with WASD; change zoom with R and F",
			&Vector3::zero(),
			&Vector2::new(20.0, 60.0),
			&projection,
		);
		self.draw_text(
			"Change attitude with the cursor keys, toggle thrust with Space",
			&Vector3::zero(),
			&Vector2::new(20.0, 40.0),
			&projection,
		);
		self.draw_text(
			"Shoot missiles with Enter",
			&Vector3::zero(),
			&Vector2::new(20.0, 20.0),
			&projection,
		);

		self.draw_text(
			format!("{}", input.attitude).as_slice(),
			&Vector3::zero(),
			&Vector2::new(right - 100.0, 40.0),
			&projection,
		);
		self.draw_text(
			if input.thrust { "Thrust ON" } else { "Thrust OFF" },
			&Vector3::zero(),
			&Vector2::new(right - 100.0, 20.0),
			&projection,
		);
	}

	fn draw_text(
		&mut self,
		text         : &str,
		position     : &Vector3<f32>,
		screen_offset: &Vector2<f32>,
		transform    : &Transform
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

				self.billboard_drawer.draw(
					&mut self.graphics,
					&self.frame,
					&Billboard {
						position   : *position,
						offset     : screen_offset + total_offset,
						texture    : texture,
						transform  : *transform,
						screen_size: self.window.size,
					},
				);
			}

			total_advance = total_advance + advance;
		}
	}

	fn draw_line_to_disc(
		&mut self,
		center   : &Vector3<f32>,
		position : &Vector3<f32>,
		transform: &Transform
	) {
		self.line_drawer.draw(
			&mut self.graphics,
			&self.frame,
			&Line {
				center   : *center,
				position : *position,
				transform: *transform,
			}
		);

		self.bases.push(Base {
			center   : *center,
			position : *position,
			transform: *transform,
		});
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
