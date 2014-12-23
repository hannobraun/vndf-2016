use libc;
use std::cmp::min;
use std::io::{
	stdout,
	IoResult,
	LineBufferedWriter,
};
use std::io::stdio::StdWriter;
use std::mem::swap;

use termios::Termios;

use super::{
	Pos,
	ScreenBuffer,
};


pub struct Screen {
	stdout  : LineBufferedWriter<StdWriter>,
	buffer_a: ScreenBuffer,
	buffer_b: ScreenBuffer,
	cursor  : Option<(Pos, Pos)>,
}

impl Screen {
	pub fn new(width: Pos, height: Pos) -> IoResult<Screen> {
		let mut termios = Termios::get(libc::STDIN_FILENO);
		termios.echo(false);
		termios.canonical_input(false);
		termios.set(libc::STDIN_FILENO);

		let mut stdout = stdout();
		match write!(&mut stdout, "\x1b[2J") { // clear screen
			Ok(())     => (),
			Err(error) => return Err(error),
		}

		let buffer_a = ScreenBuffer::new(width, height);
		let buffer_b = buffer_a.clone();

		Ok(Screen {
			stdout  : stdout,
			buffer_a: buffer_a,
			buffer_b: buffer_b,
			cursor  : None,
		})
	}

	pub fn buffer(&mut self) -> &mut ScreenBuffer {
		&mut self.buffer_a
	}

	pub fn cursor(&mut self, pos: Option<(Pos, Pos)>) {
		self.cursor = pos;
	}

	pub fn submit(&mut self) -> IoResult<()> {
		{
			let mut iter = self.buffer_a.iter().zip(self.buffer_b.iter());
			for ((x, y, c_a), (_, _, c_b)) in iter {
				if c_a != c_b {
					try!(write!(
						&mut self.stdout,
						"\x1b[{};{}H", // move cursor
						y + 1, x + 1,
					));
					if c_a.bold {
						try!(write!(
							&mut self.stdout,
							"\x1b[1m", // set bold
						));
					}
					try!(write!(
						&mut self.stdout,
						"\x1b[{}m", // set foreground color
						c_a.foreground_color.foreground_code(),
					));
					if let Some(color) = c_a.background_color {
						try!(write!(
							&mut self.stdout,
							"\x1b[{}m", // set background color
							color.background_code(),
						));
					}
					try!(self.stdout.write_char(c_a.c));
					if c_a.bold {
						try!(write!(
							&mut self.stdout,
							"\x1b[0m", // reset attributes
						));
					}
				}
			}
		}

		swap(&mut self.buffer_a, &mut self.buffer_b);
		self.buffer_a.clear();

		match self.cursor {
			Some((x, y)) => {
				let x = min(x, self.buffer_a.width()  - 1);
				let y = min(y, self.buffer_a.height() - 1);

				try!(write!(
					&mut self.stdout,
					"\x1b[?25h", // show cursor
				));
				try!(write!(
					&mut self.stdout,
					"\x1b[{};{}H", // set cursor
					y + 1, x + 1
				));
			},
			None => {
				try!(write!(
					&mut self.stdout,
					"\x1b[?25l", // hide cursor
				));
			},
		}

		try!(self.stdout.flush());
		Ok(())
	}
}
