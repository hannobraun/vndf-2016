#[deriving(Clone, Eq, PartialEq)]
pub enum Color {
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
}

impl Color {
	pub fn default() -> Color {
		Color::White
	}

	pub fn foreground_code(&self) -> u16 {
		match *self {
			Color::Black   => 30,
			Color::Red     => 31,
			Color::Green   => 32,
			Color::Yellow  => 33,
			Color::Blue    => 34,
			Color::Magenta => 35,
			Color::Cyan    => 36,
			Color::White   => 37,
		}
	}
}
