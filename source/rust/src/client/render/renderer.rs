use std::fmt::Write;

use nalgebra::{
	cast,
	Ortho3,
};

use client::interface::Frame;
use client::render::base::Graphics;
use client::render::draw::{
	GlyphDrawer,
	ShipDrawer,
};


pub struct Renderer {
	graphics    : Graphics,
	glyph_drawer: GlyphDrawer,
	ship_drawer : ShipDrawer,
}

impl Renderer {
	pub fn new(mut graphics: Graphics, size: (f32, f32)) -> Renderer {
		let transform =
			Ortho3::new(
				size.0, size.1,
				-1.0, 1.0,
			)
			.to_mat();

		let glyph_drawer = GlyphDrawer::new(&mut graphics, transform);
		let ship_drawer  = ShipDrawer::new(&mut graphics, transform);

		Renderer {
			graphics    : graphics,
			glyph_drawer: glyph_drawer,
			ship_drawer : ship_drawer,
		}
	}

	pub fn render(&mut self, output: &[String], command: &str, frame: &Frame) {
		self.graphics.clear();

		for (y, line) in output.iter().enumerate() {
			for (x, c) in line.chars().enumerate() {
				self.glyph_drawer.draw(
					x as u16,
					y as u16,
					c,
					&mut self.graphics,
				);
			}
		}

		let mut command_line = String::new();

		write!(&mut command_line, "> {}_", command)
			.unwrap_or_else(|e| panic!("Error writing to String: {}", e));

		for (x, c) in command_line.chars().enumerate() {
			self.glyph_drawer.draw(
				x as u16,
				23,
				c,
				&mut self.graphics,
			);
		}

		for (ship_id, ship) in &frame.ships {
			let mut color = [0.0,0.0,1.0];
			if let Some(sid) = frame.ship_id {
				if *ship_id == sid  { color = [0.0,1.0,0.5]; }
			}
			self.ship_drawer.draw(&mut self.graphics,
								  &cast(ship.position),
								  color);

			// draw ship id
			self.render_text(&ship_id.to_string(),
							 [ship.position[0],ship.position[1]+20.0],
							 true);

			// draw ship broadcast
			if let Some(ship_comm) = frame.broadcasts.get(&ship_id) {
				self.render_text(ship_comm,
								 [ship.position[0],ship.position[1]-40.0],
								 true);
			}

			// draw ship position
			let pos = format!("pos: ({}, {})", ship.position[0], ship.position[1]);
			self.render_text(&pos,
							 [ship.position[0]+30.0,ship.position[1]+10.0],
							 false);

			// draw ship velocity
			let vel = format!("vel: ({}, {})", ship.velocity[0], ship.velocity[1]);
			self.render_text(&vel,
							 [ship.position[0]+30.0,ship.position[1]-10.0],
							 false);
		}

		self.graphics.flush();
	}

	// NOTE: glyph size offset is currently hardcoded to 9px
	fn render_text (&mut self, text: &String, pos: [f64;2], center: bool) {
		let glyph_offset = 9;

		let pos_offset = if center {
			// For reasons I don't fully understand, the text doesn't look sharp
			// when the offset is fractional. We're preventing this here by
			// keeping it as an integer up here and only cast below.
			(glyph_offset * text.chars().count()) / 2
		}
		else {
			0
		};
		
		for (x, c) in text.chars().enumerate() {
			self.glyph_drawer.draw_at(
				(pos[0] - pos_offset as f64 + ((x * glyph_offset) as f64)),
				pos[1],
				c,
				[1.0,1.0,1.0],
				&mut self.graphics,
				);
		}
	}
}
