pub struct PlayerOutput;

impl PlayerOutput {
	pub fn render(&self, i: u8) {
		print!("\x1b[1A\x1b[2K");
		print!("{}\n", i);
	}
}
