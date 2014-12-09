#[deriving(Clone, Eq, PartialEq)]
pub enum Color {
	Normal(ColorName),
	Bright(ColorName),
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
