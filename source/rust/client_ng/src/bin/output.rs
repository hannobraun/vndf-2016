pub trait Output {
	fn render(&mut self, i: u8);
}


pub struct PlayerOutput;

impl Output for PlayerOutput {
	fn render(&mut self, i: u8) {
		print!("\x1b[1A\x1b[2K");
		print!("{}\n", i);
	}
}
