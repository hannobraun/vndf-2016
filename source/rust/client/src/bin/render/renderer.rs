use std::fmt::Write;

use nalgebra::{
	Mat4,
	Ortho3,
};

use render::{
	GlyphDrawer,
	Graphics,
};


pub struct Renderer {
	graphics    : Graphics,
	transform   : Mat4<f32>,
	glyph_drawer: GlyphDrawer,
}

impl Renderer {
	pub fn new(mut graphics: Graphics, size: (f32, f32)) -> Renderer {
		let transform =
			Ortho3::new(
				size.0, size.1,
				-1.0, 1.0,
			)
			.to_mat();

		let glyph_drawer = GlyphDrawer::new(&mut graphics);

		Renderer {
			graphics    : graphics,
			transform   : transform,
			glyph_drawer: glyph_drawer,
		}
	}

	pub fn render(&mut self, output: &[String], command: &str) {
		self.graphics.clear();

		for (y, line) in output.iter().enumerate() {
			for (x, c) in line.chars().enumerate() {
				self.glyph_drawer.draw(
					x as u16,
					y as u16,
					c,
					&self.transform,
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
				&self.transform,
				&mut self.graphics,
			);
		}

		self.graphics.graphics.end_frame();
	}
}
