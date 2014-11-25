use libc;

use termios::Termios;


pub trait Output {
	fn render(&mut self, i: u8);
}


pub struct PlayerOutput;

impl PlayerOutput {
	pub fn new() -> PlayerOutput {
		let mut termios = Termios::get(libc::STDIN_FILENO);
		termios.echo(false);
		termios.canonical_input(false);
		termios.set(libc::STDIN_FILENO);

		PlayerOutput
	}
}

impl Output for PlayerOutput {
	fn render(&mut self, i: u8) {
		print!("\x1b[2J\x1b[H");
		print!("{}\n", i);
	}
}
