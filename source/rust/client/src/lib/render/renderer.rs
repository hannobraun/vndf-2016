use std::fmt::Write;

use nalgebra::{
	cast,
	Ortho3,
};

use interface::Frame;
use render::draw::{
	GlyphDrawer,
	ShipDrawer,
};
use render::base::Graphics;


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

		for (_, ship) in &frame.ships {
			self.ship_drawer.draw(&mut self.graphics, &cast(ship.position));
		}

		self.graphics.flush();
	}
}
