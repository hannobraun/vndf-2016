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
			self.ship_drawer.draw(&mut self.graphics, &cast(ship.position));

			// draw ship id
			let vsid: Vec<char> = ship_id.to_string().chars().collect();
			let glyph_offset = 9;
			let pos_offset = ((glyph_offset * vsid.len())
							  as f64 / 2.0); //centers ship id
			for (x, c) in vsid.iter().enumerate() {
				self.glyph_drawer.draw_at(
					(ship.position[0] - pos_offset + ((x * glyph_offset) as f64)),
					(ship.position[1]+20.0),
					*c,
					&mut self.graphics,
				);
			}

			// draw ship broadcast
			if let Some(ship_comm) = frame.broadcasts.get(&ship_id) {
				let glyph_offset = 9;
				let pos_offset = ((glyph_offset * ship_comm.len())
								  as f64 / 2.0); //centers ship id
				for (x, c) in ship_comm.chars().enumerate() {
					self.glyph_drawer.draw_at(
						(ship.position[0] - pos_offset + ((x * glyph_offset) as f64)),
						(ship.position[1]-40.0),
						c,
						&mut self.graphics,
						);
				}
			}
		}

		self.graphics.flush();
	}
}
