use std::io::IoResult;

use client::render::Frame;

use super::Render;


pub struct HeadlessRenderer;

impl HeadlessRenderer {
	pub fn new() -> IoResult<HeadlessRenderer> {
		Ok(HeadlessRenderer)
	}
}

impl Render for HeadlessRenderer {
	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		print!("{}\n", frame.to_json());
		Ok(())
	}
}
