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
}
