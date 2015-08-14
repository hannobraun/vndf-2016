use std::fmt::Write;

use nalgebra::{
	cast,
	Mat4,
	Ortho3,
};

use client::interface::Frame;
use client::render::base::Graphics;
use client::window::Window;
use client::render::base::color;
use client::render::draw::{
	GlyphDrawer,
	ShipDrawer,
};


pub struct Renderer {
	glyph_drawer: GlyphDrawer,
	ship_drawer : ShipDrawer,

}

impl Renderer {
	pub fn new(window: &Window) -> Renderer {
		let mut graphics = window.create_graphics();
		
		let glyph_drawer = GlyphDrawer::new(&mut graphics);
		let ship_drawer  = ShipDrawer::new(&mut graphics);

		Renderer {
			glyph_drawer: glyph_drawer,
			ship_drawer : ship_drawer,
		}
	}

	/// get new ortho transform matrix based on window size specified
	fn get_transform(size: (u32,u32)) -> Mat4<f32> {
		Ortho3::new(
			size.0 as f32, size.1 as f32,
			-1.0, 1.0,
			)
			.to_mat()
	}
	
	pub fn render(
		&mut self,
		output : &[String],
		command: (&str,usize),
		frame  : &Frame,
		window : &Window,
	) {
		let     window_size = window.get_size();
		let     transform   = Renderer::get_transform(window_size);
		let mut graphics    = window.create_graphics();

		graphics.clear();

		for (y, line) in output.iter().enumerate() {
			self.render_text(
				&line,
				position_cli(0, y, window_size),
				false,
				transform,
				&mut graphics,
			);
		}
		
		let mut command_line = String::new();
		let prompt_ypos = 23;
		
		write!(&mut command_line, "> {}", command.0)
			.unwrap_or_else(|e| panic!("Error writing to String: {}", e));

		
		self.render_text(
			&command_line,
			position_cli(0, prompt_ypos, window_size),
			false,
			transform,
			&mut graphics,
		);

		//draw cursor position in prompt
		self.render_text(
			&"_".to_string(),
			position_cli(command.1 + 2, prompt_ypos, window_size),
			false,
			transform,
			&mut graphics,
		);
		

		for (ship_id, ship) in &frame.ships {
			let mut color = color::Colors::blue();
			if let Some(sid) = frame.ship_id {
				if *ship_id == sid  { color = color::Colors::green_spring(); }
			}
			self.ship_drawer.draw(
				&cast(ship.position),
				color,
				transform,
				&mut graphics,
			);

			// draw ship id
			self.render_text(
				&ship_id.to_string(),
				[ship.position[0],ship.position[1]+20.0],
				true,
				transform,
				&mut graphics,
			);

			// draw ship broadcast
			if let Some(ship_comm) = frame.broadcasts.get(&ship_id) {
				self.render_text(
					ship_comm,
					[ship.position[0],ship.position[1]-40.0],
					true,
					transform,
					&mut graphics,
				);
			}

			// draw ship position
			let pos = format!("pos: ({}, {})", ship.position[0], ship.position[1]);
			self.render_text(
				&pos,
				[ship.position[0]+30.0,ship.position[1]+10.0],
				false,
				transform,
				&mut graphics,
			);

			// draw ship velocity
			let vel = format!("vel: ({}, {})", ship.velocity[0], ship.velocity[1]);
			self.render_text(
				&vel,
				[ship.position[0]+30.0,ship.position[1]-10.0],
				false,
				transform,
				&mut graphics,
			);
		}

		graphics.flush();
	}

	// NOTE: glyph size offset is currently hardcoded to 9px
	fn render_text(
		&mut self,
		text     : &String,
		pos      : [f64;2],
		center   : bool,
		transform: Mat4<f32>,
		graphics : &mut Graphics,
	) {
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
			self.glyph_drawer.draw(
				(pos[0] - pos_offset as f64 + ((x * glyph_offset) as f64)),
				pos[1],
				c,
				color::Colors::white(),
				transform,
				graphics,
			);
		}
	}
}


/// This is used to position CLI text
/// It takes in to account the window sizing
fn position_cli(x: usize, y: usize, window_size: (u32, u32)) -> [f64;2] {
	let (width, height) = window_size;

	let pad_x = 10.0f64;
	let pad_y = 30.0f64;
	let offset_x = 9.0;
	let offset_y = 18.0;

	[(-1.0 * ((width as f64/2.0) - pad_x)) + offset_x * x as f64,
	 ((height as f64/2.0) - pad_y) + offset_y * (y as f64 * -1.0),]
}
