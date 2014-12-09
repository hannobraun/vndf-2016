#[deriving(Clone, Eq, PartialEq)]
pub enum Color {
	Normal(ColorName),
	Bright(ColorName),
}

impl Color {
	pub fn default() -> Color {
		Color::Bright(ColorName::White)
	}
}


#[deriving(Clone, Eq, PartialEq)]
pub enum ColorName {
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
}
